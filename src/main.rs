extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use rand::{thread_rng, Rng};
use std::collections::LinkedList;

mod app;
mod square;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = app::App {
        gl: GlGraphics::new(opengl),
        square: LinkedList::new(),
    };

    let mut i = 0.0;
    let mut rng = thread_rng();

    while i <= 600.0 {
        let mut j = 0.0;
        while j <= 600.0 {
            let rnd_arr: [f32; 8] = rng.gen();

            let mut r = rnd_arr[0] * 2.0;
            r = r.max(0.3).min(2.0);

            let mut red: f32 = rnd_arr[1];
            let mut green: f32 = rnd_arr[2];
            let mut blue: f32 = rnd_arr[3];
            let mut tr: f32 = rnd_arr[4];

            red = red.max(0.2).min(1.0);
            green = green.max(0.2).min(1.0);
            blue = blue.max(0.2).min(1.0);
            tr = tr.max(0.5).min(1.0);

            app.square.push_front(app::square::Square {
                color: [red, green, blue, tr],
                x: i,
                y: j,
                rot_speed: r as f64 * 2.0,
                rotation: 0f64,
            });
            j += 50.0;
        }
        i += 50.0;
    }

    let mut e_settings = EventSettings::new();
    e_settings.ups = 120;
    let mut events = Events::new(e_settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
