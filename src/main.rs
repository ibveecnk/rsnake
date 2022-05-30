extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;
use std::collections::LinkedList;

mod app;
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
        square: LinkedList::new(),
    };

    const MAX_VEL: f64 = 100.0;

    for _i in 0..1000 as i64 {
        app.square.push_front(app::square::Square::new(
            settings::WINDOWSIZE[0] / 2.0,
            settings::WINDOWSIZE[1] / 2.0,
            1.0,
            0.0,
            0.0,
            rng.gen_range(-MAX_VEL..MAX_VEL),
            rng.gen_range(-MAX_VEL..MAX_VEL),
            [0.0, 255.0, 0.0, 255.0],
        ));
    }

    println!("Created {} objects.", app.square.len());

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
    }
}
