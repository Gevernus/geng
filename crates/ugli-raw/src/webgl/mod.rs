use batbox::{js_sys, web_sys};

pub type BitField = u32;
pub type Bool = bool;
pub type ClampedFloat = f32;
pub type Enum = u32;
pub type Float = f32;
pub type Int = i32;
pub type IntPtr = i32;
pub type SizeI = i32;
pub type UByte = u8;
pub type UInt = u32;
pub type SizeIPtr = i32;

pub struct Context {
    inner: web_sys::WebGlRenderingContext,
    angle_instanced_arrays: web_sys::AngleInstancedArrays,
    oes_standard_derivatives: web_sys::OesStandardDerivatives,
}

impl Context {
    pub fn new(webgl_rendering_context: web_sys::WebGlRenderingContext) -> Self {
        use wasm_bindgen::JsCast;
        let angle_instanced_arrays = webgl_rendering_context
            .get_extension("ANGLE_instanced_arrays")
            .unwrap()
            .expect("ANGLE_instanced_arrays not supported?");
        let oes_standard_derivatives = webgl_rendering_context
            .get_extension("OES_standard_derivatives")
            .unwrap()
            .expect("OES_standard_derivatives not supported?");
        // Unchecked casts here because the type is different in different browsers
        let angle_instanced_arrays =
            angle_instanced_arrays.unchecked_into::<web_sys::AngleInstancedArrays>();
        let oes_standard_derivatives =
            oes_standard_derivatives.unchecked_into::<web_sys::OesStandardDerivatives>();
        Self {
            inner: webgl_rendering_context,
            angle_instanced_arrays,
            oes_standard_derivatives,
        }
    }
}

mod buffer;
mod constants;
mod draw;
mod framebuffer;
mod program_shader;
mod renderbuffer;
mod state;
mod texture;
mod uniform_attribute;
mod view;

pub use buffer::*;
pub use constants::*;
pub use draw::*;
pub use framebuffer::*;
pub use program_shader::*;
pub use renderbuffer::*;
pub use state::*;
pub use texture::*;
pub use uniform_attribute::*;
pub use view::*;
