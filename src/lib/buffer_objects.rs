use cgmath::{ Matrix4, Vector3 };

#[derive(Default, Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2]
}
vulkano::impl_vertex!(Vertex, position, color, normal, uv);

#[derive(Default, Debug, Clone, Copy)]
pub struct DummyVertex {
    pub position: [f32; 2]
}
vulkano::impl_vertex!(DummyVertex, position);

impl DummyVertex {
    // DummyVertices are only intended to be used by shaders that don't require geometry input
    pub fn list() -> [DummyVertex; 6] {
        [
            DummyVertex { position: [-1.0, -1.0] },
            DummyVertex { position: [-1.0, 1.0] },
            DummyVertex { position: [1.0, 1.0] },
            DummyVertex { position: [-1.0, -1.0] },
            DummyVertex { position: [1.0, 1.0] },
            DummyVertex { position: [1.0, -1.0] },
        ]
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct UniformBufferObject {
    pub model: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub proj: Matrix4<f32>
}

#[derive(Clone, Copy)]
pub struct VPBufferObject {
    pub view: Matrix4<f32>,
    pub proj: Matrix4<f32>
}

#[derive(Clone, Copy)]
pub struct ModelBufferObject {
    pub model: Matrix4<f32>,
    pub normals: Matrix4<f32>
}

#[derive(Clone, Copy)]
pub struct AmbientBufferObject {
    pub color: Vector3<f32>,
    pub intensity: f32
}

#[derive(Clone, Copy)]
pub struct DirectionalBufferObject {
    pub position: [f32; 4],
    pub color: Vector3<f32>
}
