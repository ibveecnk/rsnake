use opengl_graphics::GlGraphics;
use piston::{
    input::{RenderArgs, UpdateArgs},
    Button, ButtonArgs, ButtonState,
};
use rand::Rng;
use std::collections::LinkedList;

use crate::settings;

#[path = "./square.rs"]
pub mod square;

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub snake: LinkedList<square::Square>,
    pub food: square::Square,
    pub last_key: piston::Button,
    pub score: i64,
}

impl App {
    fn update_snake(&mut self, x: f64, y: f64) {
        let v_x = settings::SPEED * x;
        let v_y = settings::SPEED * y;

        let iter = self.snake.iter_mut();

        iter.for_each(|i| {
            i.mov_speed_x = v_x;
            i.mov_speed_y = v_y;
        })
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BGCOLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            clear(BGCOLOR, gl);
        });

        self.food.render(&mut self.gl, args);

        let iter = self.snake.iter_mut();

        iter.for_each(|i| {
            i.render(&mut self.gl, args);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let mut rng = rand::thread_rng();

        self.food.update(args);
        let iter = self.snake.iter_mut();
        iter.for_each(|i| {
            i.update(args);
            if i.x >= self.food.x - self.food.width / 2.0
                && i.x <= self.food.x + self.food.width / 2.0
                && i.y >= self.food.y - self.food.width / 2.0
                && i.y <= self.food.y + self.food.width / 2.0
            {
                // Generate new random food
                self.food = square::Square::new(
                    rng.gen_range(5.0..(settings::WINDOWSIZE[0] - 5_f64)),
                    rng.gen_range(5.0..(settings::WINDOWSIZE[1] - 5_f64)),
                    10.0,
                    0.0,
                    0.0,
                    [255.0, 50.0, 50.0, 255.0],
                    square::SquareType::Food,
                );

                // Rudimentary score counter
                // TODO: draw score on screen
                self.score += 1;
                println!("Score: {}", self.score);
            }
        });
    }

    pub fn input(&mut self, args: &ButtonArgs) {
        if args.state == ButtonState::Press {
            match args.button {
                Button::Keyboard(piston::Key::W) => {
                    if self.last_key != Button::Keyboard(piston::Key::S) {
                        self.update_snake(0.0, -1.0);
                        self.last_key = args.button;
                    }
                }
                Button::Keyboard(piston::Key::A) => {
                    if self.last_key != Button::Keyboard(piston::Key::D) {
                        self.update_snake(-1.0, 0.0);
                        self.last_key = args.button;
                    }
                }
                Button::Keyboard(piston::Key::S) => {
                    if self.last_key != Button::Keyboard(piston::Key::W) {
                        self.update_snake(0.0, 1.0);
                        self.last_key = args.button;
                    }
                }
                Button::Keyboard(piston::Key::D) => {
                    if self.last_key != Button::Keyboard(piston::Key::A) {
                        self.update_snake(1.0, 0.0);
                        self.last_key = args.button;
                    }
                }
                _ => return,
            }
        }
    }
}
