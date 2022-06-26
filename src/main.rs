extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{ButtonEvent, Key};
use rand::Rng;
use std::vec::Vec;

mod app;

#[path = "./math.rs"]
pub mod math;
#[path = "./settings.rs"]
pub mod settings;
#[path = "./square.rs"]
pub mod square;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let mut rng = rand::thread_rng();

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("RSnake", settings::WINDOWSIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let mut app = app::App {
        gl: GlGraphics::new(opengl),
        snake: Vec::new(),
        food: app::square::Square::new(
            rng.gen_range(5.0..(settings::WINDOWSIZE[0] - 5_f64)),
            rng.gen_range(5.0..(settings::WINDOWSIZE[1] - 5_f64)),
            10.0,
            0.0,
            0.0,
            [1.0, 0.0, 0.0, 1.0],
            app::square::SquareType::Food,
        ),
        last_key: piston::Button::Keyboard(Key::AcHome),
        score: 0,
    };

    app.snake.push(app::square::Square::new(
        settings::WINDOWSIZE[0] / 2.0,
        settings::WINDOWSIZE[1] / 2.0,
        10.0,
        0.0,
        0.0,
        [0.0, 1.0, 0.0, 1.0],
        app::square::SquareType::Head,
    ));

    let mut e_settings = EventSettings::new();
    e_settings.ups = 120;
    let mut events = Events::new(e_settings);
    while let Some(e) = events.next(&mut window) {
        // Render loop
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        // update loop
        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.button_args() {
            app.input(&args);
        }
    }
}
