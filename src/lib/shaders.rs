pub mod deferred {
    pub mod vs {
        vulkano_shaders::shader! {
            ty: "vertex",
            path: "src/shaders/deferred_vert.glsl"
        }
    }
    
    pub mod fs {
        vulkano_shaders::shader! {
            ty: "fragment",
            path: "src/shaders/deferred_frag.glsl"
        }
    }
}
pub mod directional {
    pub mod vs {
        vulkano_shaders::shader! {
            ty: "vertex",
            path: "src/shaders/directional_vert.glsl"
        }
    }
    
    pub mod fs {
        vulkano_shaders::shader! {
            ty: "fragment",
            path: "src/shaders/directional_frag.glsl"
        }
    }
}
pub mod ambient {
    pub mod vs {
        vulkano_shaders::shader! {
            ty: "vertex",
            path: "src/shaders/ambient_vert.glsl"
        }
    }
    
    pub mod fs {
        vulkano_shaders::shader! {
            ty: "fragment",
            path: "src/shaders/ambient_frag.glsl"
        }
    }
}
