use super::Drawable;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct AddressBar {
    link: String
}

impl AddressBar {
    pub fn new () -> AddressBar {
        AddressBar {
            link: String::new()
        }
    }
}

impl Drawable for AddressBar {
    fn draw (&self, renderer: &mut Renderer) {
        let w;
        {
            let window = renderer.window().unwrap();
            w = window.size();
        }

        renderer.set_draw_color(Color::RGB(200, 200, 200));
        renderer.fill_rect(Rect::new(0, 0, w.0, 35).unwrap().unwrap());
    }
}
