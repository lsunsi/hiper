#![warn(clippy::pedantic)]

mod escape;
mod html;
mod raw;
mod render;

pub use raw::Raw;
pub use render::Render;

pub const DOCTYPE: Raw<&str> = Raw("<!DOCTYPE html>");
