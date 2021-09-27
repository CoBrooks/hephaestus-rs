//{{{ Dependencies
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, DynamicState, SubpassContents, CommandBufferExecFuture, PrimaryAutoCommandBuffer
};
use vulkano::descriptor_set::persistent::PersistentDescriptorSet;
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::format::Format;
use vulkano::image::view::ImageView;
use vulkano::image::{ImageUsage, SwapchainImage, attachment::AttachmentImage, ImmutableImage};
use vulkano::instance::Instance;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::pipeline::vertex::BuffersDefinition;
use vulkano::render_pass::{Framebuffer, FramebufferAbstract, RenderPass, Subpass};
use vulkano::sampler::Sampler;
use vulkano::swapchain;
use vulkano::swapchain::{AcquireError, Surface, Swapchain, SwapchainCreationError};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture, NowFuture};
use vulkano::Version;
use vulkano_win::VkSurfaceBuild;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use cgmath::{Rad, Deg, Point3, Vector3, Matrix4};
use std::time::Instant;
//}}}

use crate::{
    buffer_objects::*,
    world::*,
    shaders::{ vs, fs },
    object::Object,
};

#[allow(dead_code)]
pub struct Engine {
    surface: Arc<Surface<Window>>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    pub images: Vec<Arc<SwapchainImage<Window>>>,
    vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    index_buffer: Arc<CpuAccessibleBuffer<[u16]>>,
    uniform_buffers: Vec<Arc<CpuAccessibleBuffer<UniformBufferObject>>>,
    render_pass: Arc<RenderPass>,
    start_time: Instant,
    pub world: World
}

impl Engine {
    //{{{ Engine::initialize() -> engine
    pub fn initialize(world: World) -> Self {
        let start_time = Instant::now();

        let required_extensions = vulkano_win::required_extensions();
        
        let instance = Instance::new(None, Version::V1_1, &required_extensions, None).unwrap();

        let event_loop = EventLoop::new();
        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();

        let physical_device_index = Self::get_physical_device_index(&instance, &surface);
        let (device, queue) = Self::init_device(&instance, &surface, &physical_device_index);

        let physical_device = Self::get_physical_device(&instance, &physical_device_index);
        let (swapchain, images) = Self::init_swapchain(&device, physical_device, &surface, &queue);

        let (vertex_buffer, index_buffer) = Self::get_buffers(&device, &world);
        
        let dimensions = images[0].dimensions();
        let (render_pass, _) = Self::create_graphics_pipeline(&device, &swapchain, dimensions);

        let uniform_buffers = Self::create_uniform_buffers(&device, &images, start_time);

        Self {
            surface,
            device,
            queue,
            swapchain, 
            images,
            vertex_buffer,
            index_buffer,
            render_pass,
            uniform_buffers,
            start_time,
            world
        }
    }
    //}}}

    //{{{ Engine::get_physical_device_index(instance, surface) -> physical_device_index
    fn get_physical_device_index(instance: &Arc<Instance>, surface: &Arc<Surface<Window>>) -> usize {
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };
        
        let physical_device = PhysicalDevice::enumerate(instance)
            .filter(|&p| {
                p.supported_extensions().is_superset_of(&device_extensions)
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
            })
            .unwrap();

        physical_device.index()
    }
    //}}}

    //{{{ Engine::get_physical_device(instance, physical_device_index) -> physical_device
    fn get_physical_device<'a>(instance: &'a Arc<Instance>, physical_device_index: &usize) -> PhysicalDevice<'a> {
        *PhysicalDevice::enumerate(instance)
            .enumerate()
            .find(|(index, _)| index == physical_device_index)
            .map(|(_, p)| p)
            .iter()
            .min_by_key(|p| {
                match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    PhysicalDeviceType::Other => 4,
                }
            })
            .unwrap()
    }
    //}}}

    //{{{ Engine::init_device(instance, surface, physical_device_index) -> (device, queue)
    fn init_device(instance: &Arc<Instance>, surface: &Arc<Surface<Window>>, physical_device_index: &usize) -> (Arc<Device>, Arc<Queue>) {
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };
        
        let physical_device = Self::get_physical_device(instance, physical_device_index);
        let queue_family = physical_device.queue_families()
            .find(|&q| {
                q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
            }).unwrap();

        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );
        
        // Initialize device
        let (device, mut queues) = Device::new(
            physical_device,
            &Features::none(),
            &physical_device
                .required_extensions()
                .union(&device_extensions),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .unwrap();

        let queue = queues.next().unwrap();

        (device, queue)
    }
    //}}}

    //{{{ Engine::init_swapchain(device, physical_device, surface, queue) -> (swapchain, images)
    fn init_swapchain(device: &Arc<Device>, physical_device: PhysicalDevice, surface: &Arc<Surface<Window>>, queue: &Arc<Queue>) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
        let (swapchain, images) = {
            let caps = surface.capabilities(physical_device).unwrap();

            let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();

            let format = caps.supported_formats[0].0;

            let dimensions: [u32; 2] = surface.window().inner_size().into();

            Swapchain::start(device.clone(), surface.clone())
                .num_images(caps.min_image_count)
                .format(format)
                .dimensions(dimensions)
                .usage(ImageUsage::color_attachment())
                .sharing_mode(queue)
                .composite_alpha(composite_alpha)
                .build()
                .unwrap()
        };

        (swapchain, images)
    }
    //}}}

    //{{{ Engine::get_buffers(device) -> (vertex_buffer, index_buffer)
    fn get_buffers(device: &Arc<Device>, world: &World) -> (Arc<CpuAccessibleBuffer<[Vertex]>>, Arc<CpuAccessibleBuffer<[u16]>>) {
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            false,
            world.get_vertices().iter().cloned(),
        ).unwrap();
      
        let index_buffer = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            false,
            world.get_indices().iter().cloned()
        ).unwrap();

        (vertex_buffer, index_buffer)
    }
    //}}}

    //{{{ Engine::create_graphics_pipeline(device, swapchain) -> (render_pass, pipeline)
    fn create_graphics_pipeline(device: &Arc<Device>, swapchain: &Arc<Swapchain<Window>>, dimensions: [u32; 2]) -> (Arc<RenderPass>, Arc<GraphicsPipeline<BuffersDefinition>>) {

        let vs = vs::Shader::load(device.clone()).unwrap();
        let fs = fs::Shader::load(device.clone()).unwrap();
        
        let render_pass = Arc::new(
            vulkano::single_pass_renderpass!(
                device.clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: swapchain.format(),
                        samples: 1,
                    },
                    depth: {
                        load: Clear,
                        store: DontCare,
                        format: Format::D16Unorm,
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {depth}
                }
            )
            .unwrap(),
        );
        
        let pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input(
                    BuffersDefinition::new()
                        .vertex::<Vertex>()
                )
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .viewports(vec![
                    Viewport {
                        origin: [0.0, 0.0],
                        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
                        depth_range: 0.0..1.0
                    }
                ])
                .fragment_shader(fs.main_entry_point(), ())
                .depth_stencil_simple_depth()
                .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );

        (render_pass, pipeline)
    }
    //}}}
    
    //{{{ Engine::create_uniform_buffers(device, images) -> uniform_buffers
    fn create_uniform_buffers(device: &Arc<Device>, images: &Vec<Arc<SwapchainImage<Window>>>, start_time: Instant) -> Vec<Arc<CpuAccessibleBuffer<UniformBufferObject>>> {
        let mut uniform_buffers: Vec<Arc<CpuAccessibleBuffer<UniformBufferObject>>> = Vec::new();
        let dimensions: [u32; 2] = [images[0].dimensions()[0], images[0].dimensions()[1]];

        for _ in 0..images.len() {
            uniform_buffers.push(Self::create_uniform_buffer(&device, start_time, dimensions));
        }

        uniform_buffers
    }
    //}}}
    
    //{{{ Engine::create_uniform_buffer() -> buffer
    fn create_uniform_buffer(device: &Arc<Device>, start_time: Instant, dimensions: [u32; 2]) -> Arc<CpuAccessibleBuffer<UniformBufferObject>> {
        let duration = Instant::now().duration_since(start_time);
        let elapsed = (duration.as_secs() * 1000) + (duration.subsec_millis() as u64);

        let model = Matrix4::from_angle_z(Rad::from(Deg(elapsed as f32 * 0.1))); // change 0.0 to 0.xx to spin

        let view = Matrix4::look_at_rh(
            Point3::new(2.0, 2.0, 2.0),
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0) // "up vector"
        );

        let mut proj = cgmath::perspective(Rad::from(Deg(45.0)), dimensions[0] as f32 / dimensions[1] as f32, 0.1, 10.0);

        proj.y.y *= -1.0;
        
        CpuAccessibleBuffer::from_data(
            device.clone(),
            BufferUsage::uniform_buffer_transfer_destination(),
            false,
            UniformBufferObject { model, view, proj }
        ).unwrap()
    }
    //}}}
    


    fn get_image_sampler(device: &Arc<Device>) -> Arc<Sampler> {
        Sampler::new(
            device.clone(),
            vulkano::sampler::Filter::Linear,
            vulkano::sampler::Filter::Linear,
            vulkano::sampler::MipmapMode::Nearest,
            vulkano::sampler::SamplerAddressMode::Repeat,
            vulkano::sampler::SamplerAddressMode::Repeat,
            vulkano::sampler::SamplerAddressMode::Repeat,
            0.0, 1.0, 0.0, 0.0
        ).unwrap()
    }

    //{{{ engine.main_loop()
    pub fn main_loop(mut self) {
        let mut dynamic_state = DynamicState::none();

        let (mut pipeline, mut framebuffers) =
            Self::window_size_dependent_setup(&self.device, &self.images, self.render_pass.clone(), &mut dynamic_state);

        let mut recreate_swapchain = false;

        let mut previous_frame_end = Some(sync::now(self.device.clone()).boxed());

        let event_loop = EventLoop::new();

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    recreate_swapchain = true;
                }
                Event::RedrawEventsCleared => {
                    previous_frame_end.as_mut().unwrap().cleanup_finished();

                    if recreate_swapchain {
                        let dimensions: [u32; 2] = self.surface.window().inner_size().into();
                        let (new_swapchain, new_images) =
                            match self.swapchain.recreate().dimensions(dimensions).build() {
                                Ok(r) => r,
                                Err(SwapchainCreationError::UnsupportedDimensions) => return,
                                Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                            };

                        self.swapchain = new_swapchain;

                        let new = Self::window_size_dependent_setup(
                            &self.device,
                            &new_images,
                            self.render_pass.clone(),
                            &mut dynamic_state,
                        );

                        pipeline = new.0;
                        framebuffers = new.1;

                        recreate_swapchain = false;
                    }
                    
                    self.uniform_buffers = Self::create_uniform_buffers(&self.device, &self.images, self.start_time);

                    let (image_num, suboptimal, acquire_future) =
                        match swapchain::acquire_next_image(self.swapchain.clone(), None) {
                            Ok(r) => r,
                            Err(AcquireError::OutOfDate) => {
                                recreate_swapchain = true;
                                return;
                            }
                            Err(e) => panic!("Failed to acquire next image: {:?}", e),
                        };

                    if suboptimal {
                        recreate_swapchain = true;
                    }
                    
                    let clear_values = vec![self.world.void_color.into(), 1f32.into()]; // vec![[0.02, 0.02, 0.02, 1.0].into()];

                    let mut builder = AutoCommandBufferBuilder::primary(
                        self.device.clone(),
                        self.queue.family(),
                        CommandBufferUsage::OneTimeSubmit,
                    )
                    .unwrap();

                    builder
                        .begin_render_pass(
                            framebuffers[image_num].clone(),
                            SubpassContents::Inline,
                            clear_values,
                        ).unwrap();

                    let device = self.device.clone();
                    self.world.objects
                        .iter()
                        .for_each(|o| o.add_to_render_commands(
                                        &device,
                                        &self.queue,
                                        self.images[0].dimensions(),
                                        self.swapchain.format(),
                                        &dynamic_state,
                                        &self.world.camera,
                                        &mut builder
                                        )
                                  );

                    builder
                        /*.draw_indexed(
                            pipeline.clone(),
                            &dynamic_state,
                            self.vertex_buffer.clone(),
                            self.index_buffer.clone(),
                            (tex_set.clone(), ubo_set.clone()),
                            (),
                        ) // can be called multiple times with different data to render ui
                        .unwrap() */
                        .end_render_pass()
                        .unwrap();

                    let command_buffer = builder.build().unwrap();

                    let future = previous_frame_end
                        .take()
                        .unwrap()
                        .join(acquire_future)
                        //.join(tex_future)
                        .then_execute(self.queue.clone(), command_buffer)
                        .unwrap()
                        .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
                        .then_signal_fence_and_flush();

                    match future {
                        Ok(future) => {
                            previous_frame_end = Some(future.boxed());
                        }
                        Err(FlushError::OutOfDate) => {
                            recreate_swapchain = true;
                            previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                        }
                        Err(e) => {
                            println!("Failed to flush future: {:?}", e);
                            previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                        }
                    }
                }
                _ => (),
            }
        });
    }
    //}}}

    //{{{ Engine::window_size_dependent_setup(images, render_pass, dynamic_state) -> (pipeline, framebuffers)
    fn window_size_dependent_setup(
        device: &Arc<Device>,
        images: &[Arc<SwapchainImage<Window>>],
        render_pass: Arc<RenderPass>,
        dynamic_state: &mut DynamicState,
    ) -> (Arc<GraphicsPipeline<BuffersDefinition>>, Vec<Arc<dyn FramebufferAbstract + Send + Sync>>) {
        let dimensions = images[0].dimensions();
    
        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [dimensions[0] as f32, dimensions[1] as f32],
            depth_range: 0.0..1.0,
        };
        dynamic_state.viewports = Some(vec![viewport]);

        let depth_buffer = ImageView::new(
            AttachmentImage::transient(device.clone(), dimensions, Format::D16Unorm).unwrap()
        ).unwrap();

        let framebuffers = images.iter()
            .map(|image| {
                let view = ImageView::new(image.clone()).unwrap();
                Arc::new(
                    Framebuffer::start(render_pass.clone())
                        .add(view)
                        .unwrap()
                        .add(depth_buffer.clone())
                        .unwrap()
                        .build()
                        .unwrap(),
                ) as Arc<dyn FramebufferAbstract + Send + Sync>
            })
            .collect::<Vec<_>>();

        let vs = vs::Shader::load(device.clone()).unwrap();
        let fs = fs::Shader::load(device.clone()).unwrap();

        let pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input(
                    BuffersDefinition::new()
                        .vertex::<Vertex>()
                )
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_list()
                .viewports(vec![
                    Viewport {
                        origin: [0.0, 0.0],
                        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
                        depth_range: 0.0..1.0
                    }
                ])
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .depth_stencil_simple_depth()
                .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );
        
        (pipeline, framebuffers)
    }
    //}}}
}
