use std::sync::Arc;
use std::io::Cursor;
use std::fs;
use vulkano::device::{ Device, Queue };
use vulkano::format::Format;
use vulkano::pipeline::GraphicsPipeline;
use crate::shaders::{ self, FragShader, VertShader };
use crate::buffer_objects::Vertex;
use vulkano::render_pass::{ RenderPass, Subpass };
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::vertex::BuffersDefinition;
use vulkano::image::{ ImageDimensions, ImmutableImage, view::ImageView };
use vulkano::sampler::Sampler;
use vulkano::command_buffer::{ PrimaryAutoCommandBuffer, CommandBufferExecFuture };
use vulkano::sync::NowFuture;

pub trait Material {
    fn get_color(&self) -> [f32; 3];
    fn get_vertex_shader(&self, device: &Arc<Device>) -> Box<dyn VertShader>;
    fn get_fragment_shader(&self, device: &Arc<Device>) -> Box<dyn FragShader>;
    fn get_pipeline(&self, device: &Arc<Device>, dimensions: [u32; 2], color_format: Format) -> Arc<GraphicsPipeline<BuffersDefinition>>;
    fn get_render_pass(&self, device: &Arc<Device>, color_format: Format) -> Arc<RenderPass>;
    fn add_texture(&mut self, tex_path: &str);
    fn has_texture(&self) -> bool;
    fn get_texture_buffer(&self, queue: &Arc<Queue>) -> (Arc<ImageView<Arc<ImmutableImage>>>, CommandBufferExecFuture<NowFuture, PrimaryAutoCommandBuffer>);
    fn get_texture_sampler(&self, device: &Arc<Device>) -> Arc<Sampler>;
}

pub struct Diffuse {
    pub color: [f32; 3],
    pub texture_data: Option<(Vec<u8>, ImageDimensions)>,
}

impl Diffuse {
    pub fn new(color: [f32; 3]) -> Self {
        let mut d = Diffuse {
            color,
            texture_data: None
        };
        d.add_texture("models/textures/null_texture.png");

        d
    }
}

impl Material for Diffuse { 
    fn get_color(&self) -> [f32; 3] {
        self.color
    }

    fn get_vertex_shader(&self, device: &Arc<Device>) -> Box<dyn VertShader> {
        Box::new(shaders::vs::Shader::load(device.clone()).unwrap())
    }
    
    fn get_fragment_shader(&self, device: &Arc<Device>) -> Box<dyn FragShader> {
        Box::new(shaders::fs::Shader::load(device.clone()).unwrap())
    }

    fn get_pipeline(&self, device: &Arc<Device>, dimensions: [u32; 2], color_format: Format) -> Arc<GraphicsPipeline<BuffersDefinition>> {
        let vs = self.get_vertex_shader(device);
        let fs = self.get_fragment_shader(device);

        let render_pass = self.get_render_pass(device, color_format);
        
        let pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input(
                    BuffersDefinition::new()
                        .vertex::<Vertex>()
                )
                .vertex_shader(vs.get_entry_point(), ())
                .triangle_list()
                .viewports(vec![
                    Viewport {
                        origin: [0.0, 0.0],
                        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
                        depth_range: 0.0..1.0
                    }
                ])
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.get_entry_point(), ())
                .depth_stencil_simple_depth()
                .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );

        pipeline
    }

    fn get_render_pass(&self, device: &Arc<Device>, color_format: Format) -> Arc<RenderPass> {
        Arc::new(
            vulkano::single_pass_renderpass!(
                device.clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: color_format,
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
        )
    }
    
   fn add_texture(&mut self, tex_path: &str) {
        let png_bytes = fs::read(&tex_path).unwrap(); 
        let cursor = Cursor::new(png_bytes);
        let decoder = png::Decoder::new(cursor);
        let mut reader = decoder.read_info().unwrap();
        let info = reader.info();
        let dimensions = ImageDimensions::Dim2d {
            width: info.width,
            height: info.height,
            array_layers: 1
        };
        let mut image_data = Vec::new();
        image_data.resize((info.width * info.height * 4) as usize, 0);
        reader.next_frame(&mut image_data).unwrap();

        self.texture_data = Some((image_data, dimensions));
    }

    fn has_texture(&self) -> bool {
        self.texture_data.is_some()
    }

    fn get_texture_buffer(&self, queue: &Arc<Queue>) -> (Arc<ImageView<Arc<ImmutableImage>>>, CommandBufferExecFuture<NowFuture, PrimaryAutoCommandBuffer>) {
        let (tex_bytes, dimensions) = self.texture_data.as_ref().unwrap();
        
        let (image, future) = ImmutableImage::from_iter(
            tex_bytes.iter().cloned(),
            *dimensions,
            vulkano::image::MipmapsCount::One,
            Format::R8G8B8A8Srgb,
            queue.clone()
        ).unwrap();
        
        (ImageView::new(image).unwrap(), future)
    }

    fn get_texture_sampler(&self, device: &Arc<Device>) -> Arc<Sampler> {
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
}
