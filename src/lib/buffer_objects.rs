use cgmath::Matrix4;

#[derive(Default, Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2]
}
vulkano::impl_vertex!(Vertex, position, color, normal);

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct UniformBufferObject {
    pub model: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub proj: Matrix4<f32>
}
