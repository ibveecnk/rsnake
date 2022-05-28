#![allow(dead_code)]

use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

#[path = "./settings.rs"]
pub mod settings;

pub struct Square {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub rotation: f64,
    pub rot_speed: f64,
    pub mov_speed_x: f64,
    pub mov_speed_y: f64,
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
        mov_speed_x: f64,
        mov_speed_y: f64,
        color: [f32; 4],
    ) -> Square {
        Square {
            x: x,
            y: y,
            width: width,
            rotation: rotation,
            rot_speed: rot_speed,
            mov_speed_x: mov_speed_x,
            mov_speed_y: mov_speed_y,
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
                // transform rotating point to center of square d
                .trans(-0.5 * self.width, -0.5 * self.width);

            // println!("{:?}", self.color);
            // Draw a box rotating around the middle of the screen.
            rectangle(self.color, self.square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let new_rot = self.rotation + self.rot_speed * args.dt;
        // prevent rotation overflow
        if new_rot >= 360.0 {
            self.rotation = 0.0;
        } else {
            self.rotation = new_rot;
        }
        self.x += self.mov_speed_x * args.dt;
        self.y += self.mov_speed_y * args.dt;

        if self.x <= 0.0 || self.x >= settings::WINDOWSIZE[0] {
            self.mov_speed_x = -self.mov_speed_x;
        }
        if self.y <= 0.0 || self.y >= settings::WINDOWSIZE[1] {
            self.mov_speed_y = -self.mov_speed_y;
        }
    }
}
