mod car;

extern crate sdl2;
use std::path::Path;
use sdl2::{pixels::Color, rect::Rect, event::Event, keyboard::Keycode, ttf};
use car::{Car, Direction, Position};
use std::time::{Instant, Duration, SystemTime};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    let mut car = Car::new(Position(0, 0));
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Car Game", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let img = image::open(&Path::new("assets/car.png")).unwrap();
    let img = img.to_rgba8();
    let (width, height) = img.dimensions();
    let pitch = width * 4;
    let mut raw_data = img.into_raw();
    let surface = sdl2::surface::Surface::from_data(
        raw_data.as_mut_slice(), width, height, pitch,
        sdl2::pixels::PixelFormatEnum::RGBA32
    ).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Initialize event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut last_frame_time = Instant::now();
    let mut now = SystemTime::now();
    'running: loop {
        let current_time = Instant::now();
        let frame_duration = current_time.duration_since(last_frame_time);
        let frame_duration_in_seconds = frame_duration.as_secs_f64();

        if frame_duration_in_seconds > 0.0 {
            let fps = 1.0 / frame_duration_in_seconds;
            println!("FPS: {}", fps);
        }
        for event in event_pump.poll_iter() {

            last_frame_time = current_time;

            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    car.set_direction(Direction::Left);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    car.set_direction(Direction::Right);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    car.set_direction(Direction::Straight);
                },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    car.set_direction(Direction::Straight);
                },
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    car.set_direction(Direction::Straight);
                },
                _ => {}
            }
        }
        if now.elapsed().unwrap().as_millis() > 1 {
            car.move_car();
            now = SystemTime::now();
        }
        car.draw(&mut canvas, &texture);
        canvas.present();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        last_frame_time = current_time;
    }
}