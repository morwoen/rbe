extern crate rbe;
extern crate sdl2;

use rbe::{ AddressBar, Drawable };

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::{ Event, WindowEventId };

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("RBE: Rust Browser Engine", 800, 600)
        .position_centered()
        .opengl()
        .borderless()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();
    let addressbar = AddressBar::new();

    let handle_event = |event| {
        match event {
            Event::Quit {..}
            | Event::KeyDown { keycode: Some(Keycode::Escape), .. }
            | Event::Window { win_event_id: WindowEventId::Close, .. } => {
                false
            },
            Event::FingerDown { x: x, y: y, dx: dx, dy: dy, .. } => {
                println!("{} {} {} {}", x, y, dx, dy);
                true
            },
            _ => true
        }
    };

    let mut draw = || {
        renderer.set_draw_color(Color::RGB(230, 230, 230));
        renderer.clear();
        addressbar.draw(&mut renderer);
        renderer.present();
    };

    while running {
        draw();

        running = handle_event(event_pump.wait_event());

        for event in event_pump.poll_iter() {
            running = running && handle_event(event);
        }
    }
}
