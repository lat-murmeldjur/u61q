#![allow(warnings)]
use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};
#[derive(BufferContents, Vertex, Debug, Clone, Copy)]
#[repr(C)]
pub struct Position {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
}

#[derive(BufferContents, Vertex, Debug, Clone, Copy)]
#[repr(C)]
pub struct Normal {
    #[format(R32G32B32_SFLOAT)]
    pub normal: [f32; 3],
}
