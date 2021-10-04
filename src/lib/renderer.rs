#![allow(dead_code)]
use std::sync::Arc;
use vulkano::buffer::{ BufferUsage, CpuAccessibleBuffer, CpuBufferPool, cpu_pool::CpuBufferPoolSubbuffer };
use vulkano::command_buffer::{ AutoCommandBufferBuilder, CommandBufferUsage, DynamicState, PrimaryAutoCommandBuffer, SubpassContents };
use vulkano::descriptor_set::{ DescriptorSet, PersistentDescriptorSet };
use vulkano::device::{ Device, Queue, DeviceExtensions };
use vulkano::device::physical::{ PhysicalDevice, PhysicalDeviceType };
use vulkano::format::Format;
use vulkano::image::ImageAccess;
use vulkano::image::attachment::AttachmentImage;
use vulkano::image::swapchain::SwapchainImage;
use vulkano::image::view::ImageView;
use vulkano::instance::Instance;
use vulkano::memory::pool::StdMemoryPool;
use vulkano::pipeline::blend::{ AttachmentBlend, BlendFactor, BlendOp };
use vulkano::pipeline::{ GraphicsPipeline, GraphicsPipelineAbstract };
use vulkano::pipeline::viewport::Viewport;
use vulkano::render_pass::{ Framebuffer, FramebufferAbstract, RenderPass, Subpass };
use vulkano::swapchain::{ FullscreenExclusive, PresentMode, Surface, SurfaceTransform, Swapchain, SwapchainAcquireFuture, SwapchainCreationError };
use vulkano::sync::{ FlushError, GpuFuture };
use vulkano::Version;
use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;
use winit::window::{ Window, WindowBuilder };
use cgmath::{ Rad, Deg, perspective };
use egui_winit_vulkano::Gui;

use crate::{
    buffer_objects::*,
    camera::Camera,
    shaders::{ deferred, directional, ambient },
    object::Viewable,
    light::DirectionalLight,
    logger::{ self, MessageEmitter }
};

enum RenderStage {
    Stopped,
    Deferred,
    Ambient,
    Directional,
    NeedsRedraw
}

pub struct Renderer {
    instance: Arc<Instance>,
    pub surface: Arc<Surface<Window>>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub final_images: Vec<Arc<ImageView<Arc<SwapchainImage<Window>>>>>,
    camera: Camera,
    swapchain: Arc<Swapchain<Window>>,
    vp_buffer: Arc<CpuAccessibleBuffer<VPBufferObject>>,
    model_buffer: CpuBufferPool<ModelBufferObject>,
    ambient_buffer: Arc<CpuAccessibleBuffer<AmbientBufferObject>>,
    directional_buffer: CpuBufferPool<DirectionalBufferObject>,
    render_pass: Arc<RenderPass>,
    deferred_pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
    directional_pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
    ambient_pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
    dummy_verts: Arc<CpuAccessibleBuffer<[DummyVertex]>>,
    dynamic_state: DynamicState,
    framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
    color_buffer: Arc<AttachmentImage>,
    normal_buffer: Arc<AttachmentImage>,
    vp_set: Arc<dyn DescriptorSet + Send + Sync>,
    render_stage: RenderStage,
    commands: Option<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>>,
    img_index: usize,
    acquire_future: Option<SwapchainAcquireFuture<Window>>,
}

impl Renderer {
    //{{{ Renderer::new(...)
    pub fn new(event_loop: &EventLoop<()>, camera: Camera) -> Self {
        let instance = { 
            let ext = vulkano_win::required_extensions();
            Instance::new(None, Version::V1_2, &ext, None).unwrap()
        };

        let device_ext = DeviceExtensions { khr_swapchain: true, ..DeviceExtensions::none() };
        let surface = WindowBuilder::new().build_vk_surface(&event_loop, instance.clone()).unwrap();
        
        let physical = PhysicalDevice::enumerate(&instance)
            .filter(|&p| {
                p.supported_extensions().is_superset_of(&device_ext)
            })
            .filter_map(|p| {
                p.queue_families()
                    .find(|&q| {
                        q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
                    })
                    .map(|_| p)
            })
            .min_by_key(|p| {
                match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    PhysicalDeviceType::Other => 4,
                }
            }).unwrap();

        logger::log_debug(&format!("Using device: {} (type: {:?})", physical.properties().device_name, physical.properties().device_type), MessageEmitter::Renderer);

        let queue_family = physical.queue_families().find(|&q| {
            q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
        }).unwrap();

        let (device, mut queues) = Device::new(
            physical,
            physical.supported_features(),
            &device_ext,
            [(queue_family, 0.5)].iter().cloned()
        ).unwrap();

        let queue = queues.next().unwrap();

        let (swapchain, images) = {
            let caps = surface.capabilities(physical).unwrap();
            let mut usage = caps.supported_usage_flags;
            usage.depth_stencil_attachment = false;
            usage.storage = false;

            let (format, color_space) = caps.supported_formats[0];
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let dimensions: [u32; 2] = surface.window().inner_size().into();

            Swapchain::start(device.clone(), surface.clone())
                .num_images(caps.min_image_count)
                .dimensions(dimensions)
                .format(format)
                .layers(1)
                .usage(usage)
                .sharing_mode(&queue)
                .transform(SurfaceTransform::Identity)
                .composite_alpha(alpha)
                .present_mode(PresentMode::Fifo)
                .fullscreen_exclusive(FullscreenExclusive::Default)
                .clipped(true)
                .color_space(color_space)
                .build()
                .unwrap()
        };

        let vp_buffer = CpuAccessibleBuffer::from_data(
            device.clone(), 
            BufferUsage::all(), 
            false, 
            camera.get_vp_buffer(surface.window().inner_size().into())
        ).unwrap();

        let ambient_buffer = CpuAccessibleBuffer::from_data(
            device.clone(),
            BufferUsage::all(),
            false,
            AmbientBufferObject {
                color: [1.0; 3].into(),
                intensity: 0.1
            }
        ).unwrap();
        
        let model_buffer = CpuBufferPool::<ModelBufferObject>::uniform_buffer(device.clone());
        let directional_buffer = CpuBufferPool::<DirectionalBufferObject>::uniform_buffer(device.clone());

        let render_pass = Arc::new(vulkano::ordered_passes_renderpass!(
            device.clone(),
            attachments: {
                final_color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.format(),
                    samples: 1,
                },
                color: {
                    load: Clear,
                    store: DontCare,
                    format: Format::A2B10G10R10UnormPack32,
                    samples: 1,
                },
                normals: {
                    load: Clear,
                    store: DontCare,
                    format: Format::R16G16B16A16Sfloat,
                    samples: 1,
                },
                depth: {
                    load: Clear,
                    store: DontCare,
                    format: Format::D16Unorm,
                    samples: 1,
                }
            },
            passes: [
                {
                    color: [color, normals],
                    depth_stencil: {depth},
                    input: []
                },
                {
                    color: [final_color],
                    depth_stencil: {},
                    input: [color, normals]
                }
            ]
        ).unwrap());

        let deferred_pass = Subpass::from(render_pass.clone(), 0).unwrap();
        let lighting_pass = Subpass::from(render_pass.clone(), 1).unwrap();

        let deferred_vs = deferred::vs::Shader::load(device.clone()).unwrap();
        let deferred_fs = deferred::fs::Shader::load(device.clone()).unwrap();
        
        let directional_vs = directional::vs::Shader::load(device.clone()).unwrap();
        let directional_fs = directional::fs::Shader::load(device.clone()).unwrap();
        
        let ambient_vs = ambient::vs::Shader::load(device.clone()).unwrap();
        let ambient_fs = ambient::fs::Shader::load(device.clone()).unwrap();

        let deferred_pipeline = Arc::new(GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(deferred_vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(deferred_fs.main_entry_point(), ())
            .depth_stencil_simple_depth()
            .front_face_counter_clockwise()
            .cull_mode_back()
            .render_pass(deferred_pass.clone())
            .build(device.clone())
            .unwrap()
        );
        
        let directional_pipeline = Arc::new(GraphicsPipeline::start()
            .vertex_input_single_buffer::<DummyVertex>()
            .vertex_shader(directional_vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(directional_fs.main_entry_point(), ())
            .blend_collective(AttachmentBlend {
                enabled: true,
                color_op: BlendOp::Add,
                color_source: BlendFactor::One,
                color_destination: BlendFactor::One,
                alpha_op: BlendOp::Max,
                alpha_source: BlendFactor::One,
                alpha_destination: BlendFactor::One,
                mask_red: true,
                mask_green: true,
                mask_blue: true,
                mask_alpha: true
            })
            .front_face_counter_clockwise()
            .cull_mode_back()
            .render_pass(lighting_pass.clone())
            .build(device.clone())
            .unwrap()
        );
        
        let ambient_pipeline = Arc::new(GraphicsPipeline::start()
            .vertex_input_single_buffer::<DummyVertex>()
            .vertex_shader(ambient_vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(ambient_fs.main_entry_point(), ())
            .blend_collective(AttachmentBlend {
                enabled: true,
                color_op: BlendOp::Add,
                color_source: BlendFactor::One,
                color_destination: BlendFactor::One,
                alpha_op: BlendOp::Max,
                alpha_source: BlendFactor::One,
                alpha_destination: BlendFactor::One,
                mask_red: true,
                mask_green: true,
                mask_blue: true,
                mask_alpha: true
            })
            .front_face_counter_clockwise()
            .cull_mode_back()
            .render_pass(lighting_pass.clone())
            .build(device.clone())
            .unwrap()
        );

        let dummy_verts = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::vertex_buffer(),
            false,
            DummyVertex::list().iter().cloned()
        ).unwrap();

        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
            compare_mask: None,
            write_mask: None,
            reference: None
        };

        let (framebuffers, color_buffer, normal_buffer) = Renderer::window_size_dependent_setup(device.clone(), &images, render_pass.clone(), &mut dynamic_state);

        let vp_layout = deferred_pipeline.layout().descriptor_set_layouts().get(0).unwrap();
        let vp_set = Arc::new(PersistentDescriptorSet::start(vp_layout.clone())
            .add_buffer(vp_buffer.clone()).unwrap()
            .build().unwrap()
        );

        let render_stage = RenderStage::Stopped;
        
        let commands = None;
        let img_index = 0;
        let acquire_future = None;
        
        let images =
            images.into_iter().map(|image| ImageView::new(image).unwrap()).collect::<Vec<_>>();
        
        Self {
            instance,
            surface,
            device,
            queue,
            camera,
            swapchain,
            vp_buffer,
            model_buffer,
            ambient_buffer,
            directional_buffer,
            render_pass,
            deferred_pipeline,
            directional_pipeline,
            ambient_pipeline,
            dummy_verts,
            dynamic_state,
            framebuffers,
            color_buffer,
            normal_buffer,
            vp_set,
            render_stage,
            commands,
            img_index,
            acquire_future,
            final_images: images
        }
    }
    //}}}

    pub fn start(&mut self, clear_color: [f32; 4]) {
        match self.render_stage {
            RenderStage::Stopped => {
                self.render_stage = RenderStage::Deferred;
            },
            RenderStage::NeedsRedraw => {
                self.recreate_swapchain();
                self.commands = None;
                self.render_stage = RenderStage::Stopped;
                return;
            },
            _ => {
                self.render_stage = RenderStage::Stopped;
                self.commands = None;
                return;
            }
        }

        let (img_index, suboptimal, acquire_future) = match vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None) {
            Ok(r) => r,
            Err(vulkano::swapchain::AcquireError::OutOfDate) => {
                self.recreate_swapchain();
                return;
            },
            Err(err) => {
                panic!("{:?}", err)
            }
        };

        if suboptimal {
            self.recreate_swapchain();
            return;
        }

        let clear_values = vec![[0.0; 4].into(), clear_color.into(), clear_color.into(), 1f32.into()];

        let mut commands = AutoCommandBufferBuilder::primary(self.device.clone(), self.queue.family(), CommandBufferUsage::OneTimeSubmit).unwrap();
        commands
            .begin_render_pass(self.framebuffers[img_index].clone(), SubpassContents::Inline, clear_values)
            .unwrap();

        self.commands = Some(commands);
        self.img_index = img_index;
        self.acquire_future = Some(acquire_future);
    }

    pub fn geometry(&mut self, model: &dyn Viewable) {
        match self.render_stage {
            RenderStage::Deferred => { },
            RenderStage::NeedsRedraw => {
                self.recreate_swapchain();
                self.render_stage = RenderStage::Stopped;
                self.commands = None;
                return;
            },
            _ => {
                self.render_stage = RenderStage::Stopped;
                self.commands = None;
                return;
            }
        }

        let model_buffer = {
            let model_matrix = model.transform().model_matrix();

            let uniform_data = ModelBufferObject {
                model: model_matrix.clone(),
                normals: model_matrix.clone()
            };

            self.model_buffer.next(uniform_data).unwrap()
        };

        let deferred_layout = self.deferred_pipeline.layout().descriptor_set_layouts().get(0).unwrap();
        let model_set = Arc::new(
            PersistentDescriptorSet::start(deferred_layout.clone())
                .add_buffer(model_buffer.clone()).unwrap()
                .build().unwrap()
        );
        
        // Make sure vertex color is up-to-date (allows dynamically changing color in update loop)
        let model_color = model.get_material().get_color();
        let vertices: Vec<Vertex> = model.get_vertices()
            .clone()
            .iter()
            .map(|&v| Vertex { color: model_color, ..v })
            .collect();

        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::vertex_buffer(),
            false,
            vertices.iter().cloned()
        ).unwrap();

        let index_buffer = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::index_buffer(),
            false,
            model.get_indices().iter().cloned()
        ).unwrap();

        let (texture, mut texture_future) = model.get_material().get_texture_buffer(&self.queue);
        let sampler = model.get_material().get_texture_sampler(&self.device);
        let layout = self.deferred_pipeline.layout().descriptor_set_layouts().get(2).unwrap();
        let tex_set = Arc::new(
            PersistentDescriptorSet::start(layout.clone())
                .add_sampled_image(texture.clone(), sampler)
                .unwrap()
                .build()
                .unwrap()
        );
        texture_future.cleanup_finished();

        let mut commands = self.commands.take().unwrap();
        commands
            .draw_indexed(
                self.deferred_pipeline.clone(),
                &self.dynamic_state,
                vec![vertex_buffer.clone()],
                index_buffer.clone(),
                vec![self.vp_set.clone(), model_set.clone(), tex_set.clone()],
                (),
            ).unwrap();
        self.commands = Some(commands);
    }

    pub fn ambient(&mut self) {
        match self.render_stage {
            RenderStage::Deferred => {
                self.render_stage = RenderStage::Ambient;
            },
            RenderStage::Ambient => {
                return;
            },
            RenderStage::NeedsRedraw => {
                self.recreate_swapchain();
                self.commands = None;
                self.render_stage = RenderStage::Stopped;
                return;
            },
            _ => {
                self.commands = None;
                self.render_stage = RenderStage::Stopped;
                return;
            }
        }

        let ambient_layout = self.ambient_pipeline.layout().descriptor_set_layouts().get(0).unwrap();
        let ambient_set = Arc::new(PersistentDescriptorSet::start(ambient_layout.clone())
            .add_image(ImageView::new(self.color_buffer.clone()).unwrap()).unwrap()
            .add_image(ImageView::new(self.normal_buffer.clone()).unwrap()).unwrap()
            .add_buffer(self.ambient_buffer.clone()).unwrap()
            .build().unwrap()
        );

        let mut commands = self.commands.take().unwrap();
        commands
            .next_subpass(SubpassContents::Inline)
            .unwrap()
            .draw(
                self.ambient_pipeline.clone(),
                &self.dynamic_state,
                vec![self.dummy_verts.clone()],
                ambient_set.clone(),
                ()
            )
            .unwrap();
        self.commands = Some(commands);
    }

    pub fn directional(&mut self, directional_light: &DirectionalLight) {
        match self.render_stage {
            RenderStage::Ambient => {
                self.render_stage = RenderStage::Directional;
            },
            RenderStage::Directional => { },
            RenderStage::NeedsRedraw => {
                self.recreate_swapchain();
                self.commands = None;
                self.render_stage = RenderStage::Stopped;
                return;
            },
            _ => {
                self.commands = None;
                self.render_stage = RenderStage::Stopped;
                return;
            }
        }

        let directional_buffer = self.generate_directional_buffer(&self.directional_buffer, &directional_light);

        let directional_layout = self.directional_pipeline.layout().descriptor_set_layouts().get(0).unwrap();
        let directional_set = Arc::new(PersistentDescriptorSet::start(directional_layout.clone())
            .add_image(ImageView::new(self.color_buffer.clone()).unwrap()).unwrap()
            .add_image(ImageView::new(self.normal_buffer.clone()).unwrap()).unwrap()
            .add_buffer(directional_buffer.clone()).unwrap()
            .build().unwrap()
        );

        let mut commands = self.commands.take().unwrap();
        commands
            .draw(
                self.directional_pipeline.clone(),
                &self.dynamic_state,
                vec![self.dummy_verts.clone()],
                directional_set.clone(),
                ()
            )
            .unwrap();
        self.commands = Some(commands);
    }

    pub fn finish(&mut self, previous_frame_end: &mut Option<Box<dyn GpuFuture>>, gui: &mut Gui) {
        match self.render_stage {
            RenderStage::Directional => { },
            RenderStage::NeedsRedraw => {
                self.recreate_swapchain();
                self.commands = None;
                self.render_stage = RenderStage::Stopped;
                return;
            },
            _ => {
                self.commands = None;
                self.render_stage = RenderStage::Stopped;
                return;
            }
        }

        let mut commands = self.commands.take().unwrap();
        commands
            .end_render_pass()
            .unwrap();
        let command_buffer = commands.build().unwrap();

        let af = self.acquire_future.take().unwrap();
        let command_future = af.then_execute(self.queue.clone(), command_buffer).unwrap()
            .then_signal_fence_and_flush().unwrap();
        
        let after_future = gui.draw_on_image(command_future, self.final_images.get(self.img_index).unwrap().clone()).then_signal_fence_and_flush().unwrap();

        match after_future.wait(None) {
            Ok(x) => x,
            Err(e) => logger::log_error(&format!("{:?}", e), MessageEmitter::Renderer)
        }

        let future = previous_frame_end.take().unwrap().join(after_future)
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), self.img_index)
            .then_signal_fence_and_flush();
        
        match future {
            Ok(future) => {
                *previous_frame_end = Some(Box::new(future) as Box<_>);
            },
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain();
                *previous_frame_end = Some(Box::new(vulkano::sync::now(self.device.clone())) as Box<_>);
            },
            Err(e) => {
                println!("Failed to flush future: {:?}", e);
                *previous_frame_end = Some(Box::new(vulkano::sync::now(self.device.clone())) as Box<_>);
            }
        }


        self.commands = None;
        self.render_stage = RenderStage::Stopped;
    }

    pub fn recreate_swapchain(&mut self) {
        self.render_stage = RenderStage::NeedsRedraw;
        self.commands = None;

        let dimensions: [u32; 2] = self.surface.window().inner_size().into();
        let (new_swapchain, new_images) = match self.swapchain.recreate().dimensions(dimensions).build() {
            Ok(r) => r,
            Err(SwapchainCreationError::UnsupportedDimensions) => return,
            Err(e) => panic!("{:?}", e)
        };
        self.camera.proj = perspective(Rad::from(Deg(110.0)), dimensions[0] as f32 / dimensions[1] as f32, 0.01, 100.0);

        self.swapchain = new_swapchain;
        let (new_framebuffers, new_color_buffer, new_normal_buffer) = Self::window_size_dependent_setup(self.device.clone(), &new_images, self.render_pass.clone(), &mut self.dynamic_state);
        self.framebuffers = new_framebuffers;
        self.color_buffer = new_color_buffer;
        self.normal_buffer = new_normal_buffer;
        self.final_images = new_images.into_iter().map(|i| ImageView::new(i).unwrap()).collect();

        self.vp_buffer = CpuAccessibleBuffer::from_data(self.device.clone(), BufferUsage::all(), false, self.camera.get_vp_buffer(dimensions)).unwrap();

        let vp_layout = self.deferred_pipeline.layout().descriptor_set_layouts().get(0).unwrap();
        self.vp_set = Arc::new(PersistentDescriptorSet::start(vp_layout.clone())
            .add_buffer(self.vp_buffer.clone()).unwrap()
            .build().unwrap()
        );

        self.render_stage = RenderStage::Stopped;
    }

    fn generate_directional_buffer(&self, pool: &CpuBufferPool<DirectionalBufferObject>, light: &DirectionalLight)
        -> CpuBufferPoolSubbuffer<DirectionalBufferObject, Arc<StdMemoryPool>> {
        let uniform_data = DirectionalBufferObject {
            position: light.position.into(),
            color: light.color.into()
        };

        pool.next(uniform_data).unwrap()
    }
    
    fn window_size_dependent_setup(
        device: Arc<Device>,
        images: &[Arc<SwapchainImage<Window>>],
        render_pass: Arc<RenderPass>,
        dynamic_state: &mut DynamicState,
    ) -> (Vec<Arc<dyn FramebufferAbstract + Send + Sync>>, Arc<AttachmentImage>, Arc<AttachmentImage>) {
        let dimensions = images[0].dimensions();
    
        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [dimensions.width() as f32, dimensions.height() as f32],
            depth_range: 0.0..1.0,
        };
        dynamic_state.viewports = Some(vec![viewport]);

        let color_buffer = AttachmentImage::transient_input_attachment(device.clone(), dimensions.width_height(), Format::A2B10G10R10UnormPack32).unwrap();
        let normal_buffer = AttachmentImage::transient_input_attachment(device.clone(), dimensions.width_height(), Format::R16G16B16A16Sfloat).unwrap();
        let depth_buffer = AttachmentImage::transient_input_attachment(device.clone(), dimensions.width_height(), Format::D16Unorm).unwrap();

        (images.iter().map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    // This can't be best-practice...
                    .add(ImageView::new(image.clone()).unwrap()).unwrap()
                    .add(ImageView::new(color_buffer.clone()).unwrap()).unwrap()
                    .add(ImageView::new(normal_buffer.clone()).unwrap()).unwrap()
                    .add(ImageView::new(depth_buffer.clone()).unwrap()).unwrap()
                    .build().unwrap()
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        }).collect::<Vec<_>>(), color_buffer.clone(), normal_buffer.clone())
    }
}
