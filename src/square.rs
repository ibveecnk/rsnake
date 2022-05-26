#![allow(dead_code)]
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

pub struct Square {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub rot_speed: f64,
    pub color: [f32; 4],
}

impl Square {
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform
                .trans(self.x, self.y)
                .rot_rad(self.rotation.cos())
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(self.color, square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let new_rot = self.rotation + self.rot_speed * args.dt;
        if new_rot >= 360.0 {
            self.rotation = 0.0;
        } else {
            self.rotation = new_rot;
        }
    }
}
