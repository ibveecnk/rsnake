#![allow(dead_code)]

use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

use crate::settings;

#[derive(Debug, Clone, Copy)]

pub enum SquareType {
    Head,
    Tail,
    Food,
}

#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub mov_speed_x: f64,
    pub mov_speed_y: f64,
    pub color: [f32; 4],
    pub square_type: SquareType,
    square: graphics::types::Rectangle,
}

impl Square {
    pub fn new(
        x: f64,
        y: f64,
        width: f64,
        mov_speed_x: f64,
        mov_speed_y: f64,
        color: [f32; 4],
        square_type: SquareType,
    ) -> Square {
        Square {
            x,
            y,
            width,
            mov_speed_x,
            mov_speed_y,
            color,
            square_type,
            square: rectangle::square(0.0, 0.0, width),
        }
    }
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform
                .trans(self.x, self.y)
                // transform 0,0 point to center of square
                .trans(-0.5 * self.width, -0.5 * self.width);

            // Draw the rectangle
            rectangle(self.color, self.square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.x += self.mov_speed_x * args.dt;
        self.y += self.mov_speed_y * args.dt;

        if self.x - self.width / 2.0 < 0.0 {
            self.x = settings::WINDOWSIZE[0] - self.width / 2.0;
        }
        if self.x + self.width / 2.0 > settings::WINDOWSIZE[0] {
            self.x = 0.0 + self.width / 2.0;
        }
        if self.y - self.width / 2.0 < 0.0 {
            self.y = settings::WINDOWSIZE[1] - self.width / 2.0;
        }
        if self.y + self.width / 2.0 > settings::WINDOWSIZE[1] {
            self.y = 0.0 + self.width / 2.0;
        }
    }

    pub fn intersect(self, square: Square) -> bool {
        if (self.x - self.width / 2.0 >= square.x - square.width / 2.0
            || self.x + self.width / 2.0 >= square.x - square.width / 2.0)
            && (self.x - self.width / 2.0 <= square.x + square.width / 2.0
                || self.x + self.width / 2.0 <= square.x + square.width / 2.0)
            && (self.y - self.width / 2.0 >= square.y - square.width / 2.0
                || self.y + self.width / 2.0 >= square.y - square.width / 2.0)
            && (self.y - self.width / 2.0 <= square.y + square.width / 2.0
                || self.y + self.width / 2.0 <= square.y + square.width / 2.0)
        {
            return true;
        }
        false
    }
}
