//! The virtual console embedded in rCore kernel.
//!
//! The [`Console`] can be built on top of either a [`TextBuffer`] or a frame buffer ([`DrawTarget`]).
//!
//! This crate is no_std compatible.
//!
//! It can be tested in SDL2 with the help of [`embedded_graphics_simulator`](https://docs.rs//embedded-graphics/#simulator) crate.
//! See examples for details.

#![no_std]
#![deny(unsafe_code)]
#![deny(warnings)]

#[macro_use]
extern crate alloc;

#[cfg(feature = "log")]
#[macro_use]
extern crate log;
#[cfg(not(feature = "log"))]
#[macro_use]
mod log;

pub use color::*;
pub use console::*;
pub use embedded_graphics::{self, prelude::Pixel, DrawTarget};
pub use graphic::*;
pub use text_buffer::*;
pub use text_buffer_cache::*;

mod color;
mod console;
mod escape_parser;
mod graphic;
mod text_buffer;
mod text_buffer_cache;
