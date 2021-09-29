use std::sync::Arc;
use std::io::Cursor;
use std::fs;
use vulkano::device::{ Device, Queue };
use vulkano::format::Format;
use vulkano::image::{ ImageDimensions, ImmutableImage, view::ImageView };
use vulkano::sampler::Sampler;
use vulkano::command_buffer::{ PrimaryAutoCommandBuffer, CommandBufferExecFuture };
use vulkano::sync::NowFuture;

pub trait Material {
    fn get_color(&self) -> [f32; 3];
    fn add_texture(&mut self, tex_path: &str);
    fn has_texture(&self) -> bool;
    fn get_texture_buffer(&self, queue: &Arc<Queue>) -> (Arc<ImageView<Arc<ImmutableImage>>>, CommandBufferExecFuture<NowFuture, PrimaryAutoCommandBuffer>);
    fn get_texture_sampler(&self, device: &Arc<Device>) -> Arc<Sampler>;
}

#[derive(Clone)]
pub struct Diffuse {
    color: [f32; 3],
    texture_data: Option<(Vec<u8>, ImageDimensions)>,
}

impl Diffuse {
    pub fn new(color: [f32; 3]) -> Self {
        let mut d = Diffuse {
            color,
            texture_data: None,
        };
        d.add_texture("models/textures/null_texture.png");

        d
    }
}

impl Material for Diffuse { 
    fn get_color(&self) -> [f32; 3] {
        self.color
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
