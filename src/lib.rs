extern crate sdl2;
use sdl2::render::Renderer;

// parsing and layout generation
pub mod dom;
pub mod html;
pub mod css;
pub mod style;
pub mod layout;
pub mod parser;
pub mod display;

// UI widgets
pub mod addressbar;
pub use addressbar::AddressBar;

// network module
pub mod network;

pub trait Drawable {
    fn draw (&self, renderer: &mut Renderer);
}
