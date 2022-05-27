use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use std::collections::LinkedList;

#[path = "./square.rs"]
pub mod square;

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub square: LinkedList<square::Square>,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BGCOLOR: [f32; 4] = [0.0, 0.0, 0.0, 0.3];

        self.gl.draw(args.viewport(), |_c, gl| {
            clear(BGCOLOR, gl);
        });

        let iter = self.square.iter_mut();

        iter.for_each(|i| {
            i.render(&mut self.gl, &args);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let iter = self.square.iter_mut();
        iter.for_each(|i| {
            i.update(&args);
        });
    }
}
