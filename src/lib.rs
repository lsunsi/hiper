#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]

mod escape;
mod html;
mod raw;
mod render;

pub use raw::Raw;
pub use render::Render;

/// common raw html doctype value
pub const DOCTYPE: Raw<&str> = Raw("<!DOCTYPE html>");
