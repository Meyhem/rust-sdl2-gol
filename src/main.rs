mod extensions;
mod life_engine;

extern crate sdl2; 

use std::ops::{Sub, Add};
use std::time::{Duration, Instant};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas};
use sdl2::video::{Window};

use extensions::ToMilis;
use life_engine::{LifeEngine, Cell};

#[derive(Debug)]
struct GameState {
    /* Injected by engine */
    delta_time: Duration,
    window_width: u16,
    window_height: u16,

    /* Custom properties */
    delta_buffer: Duration
}

fn engine_tick(state: &mut GameState, canvas: &mut Canvas<Window>, life_engine: &mut LifeEngine) {
    state.delta_buffer = state.delta_buffer.add(state.delta_time);

    if state.delta_buffer.as_safe_milis() < 50 {
        return;
    }
    state.delta_buffer = Duration::from_millis(0);    
    
    let rect_width = state.window_width / life_engine.width;
    let rect_height = state.window_height / life_engine.height;

    for y in 0..life_engine.height {
        for x in 0..life_engine.width {
            let rect = sdl2::rect::Rect::new((x*rect_width) as i32 , (y*rect_height) as i32, rect_width as u32, rect_height as u32);
            let cell = life_engine.at(x as i16, y as i16);
            if cell == Cell::Alive {
                canvas.set_draw_color(Color::RGB(0x00, 0xB5, 0x30));
            } else {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }

            match canvas.fill_rect(rect) {
                Err(e) => panic!("Cannot render rect {:?}", e),
                _ => {}
            }
        }
    }

    life_engine.step();
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mat_width = 192;
    let mat_height = 102;
    let screen_width = 1920u16;
    let screen_height = 1080u16;
    let prob_fill = 0.05;

    let window = video_subsystem.window("Game of life", screen_width as u32, screen_height as u32)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    
    
    let mut life_engine = LifeEngine::new(mat_width, mat_height);
    life_engine.fill_random(prob_fill);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_tick = Instant::now();
    let mut current_tick = Instant::now();
    let mut state = GameState { 
        delta_time: Duration::from_millis(0),
        delta_buffer: Duration::from_millis(0),
        window_width: screen_width,
        window_height: screen_height
    };

    'running: loop {
        current_tick = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::from_millis(1));
        state.delta_time = current_tick.sub(last_tick);
        last_tick = current_tick;

        engine_tick(&mut state, &mut canvas, &mut life_engine);
        canvas.present();
    }
}