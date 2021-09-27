use vulkano::pipeline::shader::GraphicsEntryPoint;

pub trait VertShader: Shader { } 
pub trait FragShader: Shader { }

pub trait Shader {
    fn get_entry_point(&self) -> GraphicsEntryPoint;
}

pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/vert.glsl"
    }
}
impl VertShader for vs::Shader { }
impl Shader for vs::Shader {
    fn get_entry_point(&self) -> GraphicsEntryPoint {
        self.main_entry_point()
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/frag.glsl"
    }
}
impl FragShader for fs::Shader { }
impl Shader for fs::Shader {
    fn get_entry_point(&self) -> GraphicsEntryPoint {
        self.main_entry_point()
    }
}
