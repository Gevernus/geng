use super::*;

pub const TRUE: Bool = true;
pub const FALSE: Bool = false;

pub const VERSION: Enum = web_sys::WebGlRenderingContext::VERSION;
pub const LINK_STATUS: Enum = web_sys::WebGlRenderingContext::LINK_STATUS;
pub const ACTIVE_UNIFORMS: Enum = web_sys::WebGlRenderingContext::ACTIVE_UNIFORMS;
pub const ACTIVE_ATTRIBUTES: Enum = web_sys::WebGlRenderingContext::ACTIVE_ATTRIBUTES;
pub const COMPILE_STATUS: Enum = web_sys::WebGlRenderingContext::COMPILE_STATUS;
pub const CULL_FACE: Enum = web_sys::WebGlRenderingContext::CULL_FACE;
pub const VERTEX_SHADER: Enum = web_sys::WebGlRenderingContext::VERTEX_SHADER;
pub const FRAGMENT_SHADER: Enum = web_sys::WebGlRenderingContext::FRAGMENT_SHADER;
pub const LINEAR_MIPMAP_LINEAR: Enum = web_sys::WebGlRenderingContext::LINEAR_MIPMAP_LINEAR;
pub const TEXTURE0: Enum = web_sys::WebGlRenderingContext::TEXTURE0;
pub const CLAMP_TO_EDGE: Enum = web_sys::WebGlRenderingContext::CLAMP_TO_EDGE;
pub const REPEAT: Enum = web_sys::WebGlRenderingContext::REPEAT;
pub const LINEAR: Enum = web_sys::WebGlRenderingContext::LINEAR;
pub const NEAREST: Enum = web_sys::WebGlRenderingContext::NEAREST;
pub const COLOR_ATTACHMENT0: Enum = web_sys::WebGlRenderingContext::COLOR_ATTACHMENT0;
pub const TEXTURE_2D: Enum = web_sys::WebGlRenderingContext::TEXTURE_2D;
pub const ALWAYS: Enum = web_sys::WebGlRenderingContext::ALWAYS;
pub const ARRAY_BUFFER: Enum = web_sys::WebGlRenderingContext::ARRAY_BUFFER;
pub const BACK: Enum = web_sys::WebGlRenderingContext::BACK;
pub const BLEND: Enum = web_sys::WebGlRenderingContext::BLEND;
pub const COLOR_BUFFER_BIT: Enum = web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT;
pub const DEPTH_ATTACHMENT: Enum = web_sys::WebGlRenderingContext::DEPTH_ATTACHMENT;
pub const DEPTH_BUFFER_BIT: Enum = web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT;
pub const DEPTH_COMPONENT: Enum = web_sys::WebGlRenderingContext::DEPTH_COMPONENT16;
pub const DYNAMIC_DRAW: Enum = web_sys::WebGlRenderingContext::DYNAMIC_DRAW;
pub const FLOAT: Enum = web_sys::WebGlRenderingContext::FLOAT;
pub const FLOAT_VEC2: Enum = web_sys::WebGlRenderingContext::FLOAT_VEC2;
pub const FLOAT_VEC3: Enum = web_sys::WebGlRenderingContext::FLOAT_VEC3;
pub const FLOAT_VEC4: Enum = web_sys::WebGlRenderingContext::FLOAT_VEC4;
pub const INT: Enum = web_sys::WebGlRenderingContext::INT;
pub const INT_VEC2: Enum = web_sys::WebGlRenderingContext::INT_VEC2;
pub const INT_VEC3: Enum = web_sys::WebGlRenderingContext::INT_VEC3;
pub const INT_VEC4: Enum = web_sys::WebGlRenderingContext::INT_VEC4;
pub const FLOAT_MAT2: Enum = web_sys::WebGlRenderingContext::FLOAT_MAT2;
pub const FLOAT_MAT3: Enum = web_sys::WebGlRenderingContext::FLOAT_MAT3;
pub const FLOAT_MAT4: Enum = web_sys::WebGlRenderingContext::FLOAT_MAT4;
pub const FRAMEBUFFER: Enum = web_sys::WebGlRenderingContext::FRAMEBUFFER;
pub const FRAMEBUFFER_COMPLETE: Enum = web_sys::WebGlRenderingContext::FRAMEBUFFER_COMPLETE;
pub const FRONT: Enum = web_sys::WebGlRenderingContext::FRONT;
pub const GREATER: Enum = web_sys::WebGlRenderingContext::GREATER;
pub const LEQUAL: Enum = web_sys::WebGlRenderingContext::LEQUAL;
pub const LESS: Enum = web_sys::WebGlRenderingContext::LESS;
pub const LINES: Enum = web_sys::WebGlRenderingContext::LINES;
pub const LINE_LOOP: Enum = web_sys::WebGlRenderingContext::LINE_LOOP;
pub const LINE_STRIP: Enum = web_sys::WebGlRenderingContext::LINE_STRIP;
pub const NO_ERROR: Enum = web_sys::WebGlRenderingContext::NO_ERROR;
pub const ONE_MINUS_SRC_ALPHA: Enum = web_sys::WebGlRenderingContext::ONE_MINUS_SRC_ALPHA;
pub const POINTS: Enum = web_sys::WebGlRenderingContext::POINTS;
pub const RENDERBUFFER: Enum = web_sys::WebGlRenderingContext::RENDERBUFFER;
pub const RGBA: Enum = web_sys::WebGlRenderingContext::RGBA;
pub const SRC_ALPHA: Enum = web_sys::WebGlRenderingContext::SRC_ALPHA;
pub const STATIC_DRAW: Enum = web_sys::WebGlRenderingContext::STATIC_DRAW;
pub const TEXTURE_MAG_FILTER: Enum = web_sys::WebGlRenderingContext::TEXTURE_MAG_FILTER;
pub const TEXTURE_MIN_FILTER: Enum = web_sys::WebGlRenderingContext::TEXTURE_MIN_FILTER;
pub const TEXTURE_WRAP_S: Enum = web_sys::WebGlRenderingContext::TEXTURE_WRAP_S;
pub const TEXTURE_WRAP_T: Enum = web_sys::WebGlRenderingContext::TEXTURE_WRAP_T;
pub const TRIANGLES: Enum = web_sys::WebGlRenderingContext::TRIANGLES;
pub const TRIANGLE_FAN: Enum = web_sys::WebGlRenderingContext::TRIANGLE_FAN;
pub const TRIANGLE_STRIP: Enum = web_sys::WebGlRenderingContext::TRIANGLE_STRIP;
pub const UNSIGNED_BYTE: Enum = web_sys::WebGlRenderingContext::UNSIGNED_BYTE;
// pub const PROGRAM_POINT_SIZE: Enum = web_sys::WebGlRenderingContext::PROGRAM_POINT_SIZE;
pub const DEPTH_TEST: Enum = web_sys::WebGlRenderingContext::DEPTH_TEST;
pub const ALPHA: Enum = web_sys::WebGlRenderingContext::ALPHA;
pub const UNPACK_ALIGNMENT: Enum = web_sys::WebGlRenderingContext::UNPACK_ALIGNMENT;
pub const INVALID_ENUM: Enum = web_sys::WebGlRenderingContext::INVALID_ENUM;
pub const INVALID_VALUE: Enum = web_sys::WebGlRenderingContext::INVALID_VALUE;
pub const INVALID_OPERATION: Enum = web_sys::WebGlRenderingContext::INVALID_OPERATION;
pub const OUT_OF_MEMORY: Enum = web_sys::WebGlRenderingContext::OUT_OF_MEMORY;
pub const INVALID_FRAMEBUFFER_OPERATION: Enum =
    web_sys::WebGlRenderingContext::INVALID_FRAMEBUFFER_OPERATION;
pub const CONTEXT_LOST: Enum = web_sys::WebGlRenderingContext::CONTEXT_LOST_WEBGL;
pub const NEVER: Enum = web_sys::WebGlRenderingContext::NEVER;
pub const EQUAL: Enum = web_sys::WebGlRenderingContext::EQUAL;
pub const NOTEQUAL: Enum = web_sys::WebGlRenderingContext::NOTEQUAL;
pub const GEQUAL: Enum = web_sys::WebGlRenderingContext::GEQUAL;
pub const STENCIL_TEST: Enum = web_sys::WebGlRenderingContext::STENCIL_TEST;
pub const KEEP: Enum = web_sys::WebGlRenderingContext::KEEP;
pub const ZERO: Enum = web_sys::WebGlRenderingContext::ZERO;
pub const REPLACE: Enum = web_sys::WebGlRenderingContext::REPLACE;
pub const INCR: Enum = web_sys::WebGlRenderingContext::INCR;
pub const INCR_WRAP: Enum = web_sys::WebGlRenderingContext::INCR_WRAP;
pub const DECR: Enum = web_sys::WebGlRenderingContext::DECR;
pub const DECR_WRAP: Enum = web_sys::WebGlRenderingContext::DECR_WRAP;
pub const INVERT: Enum = web_sys::WebGlRenderingContext::INVERT;
pub const DEPTH_STENCIL: Enum = web_sys::WebGlRenderingContext::DEPTH_STENCIL;
pub const DEPTH_STENCIL_ATTACHMENT: Enum = web_sys::WebGlRenderingContext::DEPTH_STENCIL_ATTACHMENT;
pub const ONE: Enum = web_sys::WebGlRenderingContext::ONE;
pub const SRC_COLOR: Enum = web_sys::WebGlRenderingContext::SRC_COLOR;
pub const ONE_MINUS_SRC_COLOR: Enum = web_sys::WebGlRenderingContext::ONE_MINUS_SRC_COLOR;
pub const DST_COLOR: Enum = web_sys::WebGlRenderingContext::DST_COLOR;
pub const ONE_MINUS_DST_ALPHA: Enum = web_sys::WebGlRenderingContext::ONE_MINUS_DST_ALPHA;
pub const DST_ALPHA: Enum = web_sys::WebGlRenderingContext::DST_ALPHA;
pub const SRC_ALPHA_SATURATE: Enum = web_sys::WebGlRenderingContext::SRC_ALPHA_SATURATE;
pub const STENCIL_BUFFER_BIT: Enum = web_sys::WebGlRenderingContext::STENCIL_BUFFER_BIT;
