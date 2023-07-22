use std::mem::size_of;

use glam::{Vec2, Vec3, Vec4};
use wgpu::VertexStepMode;
use wgpu_trait::VertexLayout;

//

#[test]
fn sample() {
    #[derive(VertexLayout)]
    struct Test {
        _pos: Vec4,
        _col: Vec3,
        _uv: Vec2,
        _v: f32,
    }

    let left = Test::LAYOUT_VERTEX;
    let right = wgpu::VertexBufferLayout {
        array_stride: size_of::<Test>() as _,
        step_mode: VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 16,
                shader_location: 1,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 28,
                shader_location: 2,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32,
                offset: 36,
                shader_location: 3,
            },
        ],
    };

    assert_eq!(left, right, "left: `{left:#?}`,\nright: `{right:#?}`");
}
