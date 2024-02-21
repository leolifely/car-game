extern crate sdl2;
use sdl2::{pixels::Color, rect::Rect, event::Event, keyboard::Keycode, ttf};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Car Game", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Initialize event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}