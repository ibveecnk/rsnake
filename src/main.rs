extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use app::App;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::ButtonEvent;

mod app;
mod settings;
mod square;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("RSnake", settings::WINDOWSIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .map_or_else(|_| panic!("Could not initialize Window"), |window| window);

    let mut app = App::new(GlGraphics::new(opengl));

    app.snake.push(square::Square::new(
        settings::WINDOWSIZE[0] / 2.0,
        settings::WINDOWSIZE[1] / 2.0,
        10.0,
        0.0,
        0.0,
        [0.0, 1.0, 0.0, 1.0],
        square::Type::Head,
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
            app.update(args);
        }

        if let Some(args) = e.button_args() {
            app.input(&args);
        }
    }
}
