pub mod api;

pub use api::*;
use crate::gl::context::GLContext;
use crate::gl::feature::EntryGLFn;

impl GL21 for GLContext {
    unsafe fn entry(&self) -> &EntryGLFn { &self.entry }
}