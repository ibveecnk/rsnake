#![allow(dead_code)]

use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

pub struct Square {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub rotation: f64,
    pub rot_speed: f64,
    pub color: [f32; 4],
    square: graphics::types::Rectangle,
}

impl Square {
    pub fn new(
        x: f64,
        y: f64,
        width: f64,
        rotation: f64,
        rot_speed: f64,
        color: [f32; 4],
    ) -> Square {
        Square {
            x: x,
            y: y,
            width: width,
            rotation: rotation,
            rot_speed: rot_speed,
            color: color,
            square: rectangle::square(0.0, 0.0, width),
        }
    }
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform
                .trans(self.x, self.y)
                .rot_rad(self.rotation.cos())
                .trans(-0.5 * self.width, -0.5 * self.width);

            // println!("{:?}", self.color);
            // Draw a box rotating around the middle of the screen.
            rectangle(self.color, self.square, transform, gl);
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
