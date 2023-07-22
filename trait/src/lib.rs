pub use wgpu_trait_macro::VertexLayout;

//

use std::mem::size_of;

use glam::{DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};

//

/// # VertexLayout trait (and macro)
///
/// Generates vertex buffer layouts ([`wgpu::VertexBufferLayout`])
///
/// Fields need to implement [`AttributeFormat`]
///
/// # Usage
///
/// ```ignore
/// use wgpu_macro::{AttributeFormat, VertexLayout};
/// use glam::Vec3;
///
/// #[derive(VertexLayout)]
/// struct Vertex {
///     pos: Vec3,
/// }
///
/// device.create_render_pipeline(&RenderPipelineDescriptor {
///     vertex: VertexState {
///         buffers: &[Vertex::LAYOUT_VERTEX],
///         ..
///     },
///     ..
/// });
/// ```
pub trait VertexLayout: Sized {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute];

    const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<Self>() as _,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: Self::ATTRIBUTES,
    };

    const LAYOUT_VERTEX: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<Self>() as _,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: Self::ATTRIBUTES,
    };

    const LAYOUT_INSTANCE: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<Self>() as _,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: Self::ATTRIBUTES,
    };
}

/// # AttributeFormat
///
/// Vertex format for a field, used in [`VertexLayout`]
pub trait AttributeFormat {
    const FORMAT: wgpu::VertexFormat;
}

macro_rules! impl_multi {
    ($($from:ty => $to:ident),* $(,)?) => {
        $(
            impl AttributeFormat for $from {
                const FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::$to;
            }
        )*
    };
}

impl_multi! {
    f32 => Float32,

    (f32,) => Float32,
    (f32, f32) => Float32x2,
    (f32, f32, f32) => Float32x3,
    (f32, f32, f32, f32) => Float32x4,

    [f32; 1] => Float32,
    [f32; 2] => Float32x2,
    [f32; 3] => Float32x3,
    [f32; 4] => Float32x4,

    Vec2 => Float32x2,
    Vec3 => Float32x3,
    Vec4 => Float32x4,

    f64 => Float64,

    (f64,) => Float64,
    (f64, f64) => Float64x2,
    (f64, f64, f64) => Float64x3,
    (f64, f64, f64, f64) => Float64x4,

    [f64; 1] => Float64,
    [f64; 2] => Float64x2,
    [f64; 3] => Float64x3,
    [f64; 4] => Float64x4,

    DVec2 => Float64x2,
    DVec3 => Float64x3,
    DVec4 => Float64x4,

    u32 => Uint32,

    (u32,) => Uint32,
    (u32, u32) => Uint32x2,
    (u32, u32, u32) => Uint32x3,
    (u32, u32, u32, u32) => Uint32x4,

    [u32; 1] => Uint32,
    [u32; 2] => Uint32x2,
    [u32; 3] => Uint32x3,
    [u32; 4] => Uint32x4,

    UVec2 => Uint32x2,
    UVec3 => Uint32x3,
    UVec4 => Uint32x4,

    i32 => Sint32,

    (i32,) => Sint32,
    (i32, i32) => Sint32x2,
    (i32, i32, i32) => Sint32x3,
    (i32, i32, i32, i32) => Sint32x4,

    [i32; 1] => Sint32,
    [i32; 2] => Sint32x2,
    [i32; 3] => Sint32x3,
    [i32; 4] => Sint32x4,

    IVec2 => Sint32x2,
    IVec3 => Sint32x3,
    IVec4 => Sint32x4,
}
