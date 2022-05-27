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

#[path = "./square.rs"]
pub mod square;

fn main() {
    const WINDOWSIZE: [f64; 2] = [1000.0, 600.0];
    const INCR: f64 = 10.0;

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", WINDOWSIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let mut app = app::App {
        gl: GlGraphics::new(opengl),
        square: LinkedList::new(),
    };

    let mut i = 0.0;
    let mut rng = thread_rng();

    // generate squares
    while i <= WINDOWSIZE[0] {
        let mut j = 0.0;
        while j <= WINDOWSIZE[1] {
            // generate 5 random numbers
            let rnd_arr: [f32; 5] = rng.gen();

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

            let square =
                app::square::Square::new(i, j, INCR, 0f64, r as f64 * 2.0, [red, green, blue, tr]);

            app.square.push_front(square);
            j += INCR;
        }
        i += INCR;
    }

    println!("Created {} squares.", app.square.len());

    use std::time::Instant;
    let time = Instant::now();
    let mut now: [std::time::Instant; 2] = [time, time];
    let mut cnt: [u64; 2] = [0, 0];

    let mut e_settings = EventSettings::new();
    e_settings.ups = 120;
    let mut events = Events::new(e_settings);
    while let Some(e) = events.next(&mut window) {
        // Render loop
        if let Some(args) = e.render_args() {
            if cnt[0] == 99 {
                let elapsed = now[0].elapsed();
                println!("100 render cycles took: {:.2?}", elapsed);
                now[0] = Instant::now();
            }
            app.render(&args, cnt[0]);
            cnt[0] += 1;
            cnt[0] %= 100;
        }

        // update loop
        if let Some(args) = e.update_args() {
            if cnt[1] == 99 {
                let elapsed = now[1].elapsed();
                println!("100 update cycles took: {:.2?}", elapsed);
                now[1] = Instant::now();
            }
            app.update(&args, cnt[1]);
            cnt[1] += 1;
            cnt[1] %= 100;
        }
    }
}
