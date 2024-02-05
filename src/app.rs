use graphics::clear;
use opengl_graphics::GlGraphics;
use piston::{
    input::{RenderArgs, UpdateArgs},
    Button, ButtonArgs, ButtonState,
};
use rand::Rng;

use crate::{settings, square};

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub snake: Vec<square::Square>,
    pub food: Option<square::Square>,
    pub last_key: Option<piston::Button>,
    pub score: i64,
}

impl App {
    #[must_use]
    pub fn new(gl: GlGraphics) -> Self {
        let mut new_app = Self {
            gl,
            snake: Vec::new(),
            food: None,
            last_key: None,
            score: 0,
        };

        new_app.gen_food();
        new_app
    }
    fn update_snake(&mut self, x: f64, y: f64) {
        let v_x = settings::SPEED * x;
        let v_y = settings::SPEED * y;

        self.snake[0].mov_speed_x = v_x;
        self.snake[0].mov_speed_y = v_y;
    }

    fn gen_food(&mut self) {
        let mut rng = rand::thread_rng();

        self.food = Some(square::Square::new(
            rng.gen_range(5.0..(settings::WINDOWSIZE[0] - 5_f64)),
            rng.gen_range(5.0..(settings::WINDOWSIZE[1] - 5_f64)),
            10.0,
            0.0,
            0.0,
            [1.0, 0.0, 0.0, 1.0],
            square::Type::Food,
        ));
    }

    fn increase_snake_length(&mut self, n: i64) {
        for _ in 0..n {
            let last_x = self.snake[self.snake.len() - 1]
                .mov_speed_x
                .mul_add(-0.4, self.snake[self.snake.len() - 1].x);
            let last_y = self.snake[self.snake.len() - 1]
                .mov_speed_y
                .mul_add(-0.4, self.snake[self.snake.len() - 1].y);

            self.snake.push(square::Square::new(
                last_x,
                last_y,
                10.0,
                0.0,
                0.0,
                [0.1, 0.7, 0.3, 1.0],
                square::Type::Tail,
            ));
        }
        self.score += n;
    }

    #[allow(dead_code)]
    pub fn reset_tail(&mut self) -> usize {
        let len = self.snake.len();
        self.snake = vec![self.snake[0]];
        len
    }

    fn game_over(&mut self) {
        #[allow(clippy::print_stdout)]
        {
            println!("Game Over! Your score was {}.", self.score);
        }
        self.score = 0;
        self.snake = vec![self.snake[0]];
    }

    pub fn render(&mut self, args: &RenderArgs) {
        const BGCOLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            clear(BGCOLOR, gl);
        });

        if let Some(food) = &mut self.food {
            food.render(&mut self.gl, args);
        }

        // Reverse, so head is drawn on top
        for i in (0..self.snake.len()).rev() {
            self.snake[i].render(&mut self.gl, args);
        }
    }

    pub fn update(&mut self, args: UpdateArgs) {
        if let Some(food) = &mut self.food {
            food.update(args);
        }
        self.snake[0].update(args);

        // Check for Food/Snake collision
        if let Some(food) = &self.food {
            if self.snake[0].intersect(food) {
                self.gen_food();
                self.increase_snake_length(1);

                // Rudimentary score counter
                // TODO: draw score on screen

                #[allow(clippy::print_stdout)]
                {
                    println!("Score: {}", self.score);
                }
            }
        }

        for i in (1..self.snake.len()).rev() {
            // magic number is the bonding force
            let mut x_diff = self.snake[i - 1].x - self.snake[i].x;
            let mut y_diff = self.snake[i - 1].y - self.snake[i].y;

            // wall teleportation fix (avoid gigantic speed in wrong direction)
            if x_diff > 0.9 * settings::WINDOWSIZE[0] {
                x_diff -= settings::WINDOWSIZE[0];
            } else if x_diff < -0.9 * settings::WINDOWSIZE[0] {
                x_diff += settings::WINDOWSIZE[0];
            }
            if y_diff > 0.9 * settings::WINDOWSIZE[1] {
                y_diff -= settings::WINDOWSIZE[1];
            } else if y_diff < -0.9 * settings::WINDOWSIZE[1] {
                y_diff += settings::WINDOWSIZE[1];
            }
            self.snake[i].mov_speed_x = (x_diff) * 10.0;
            self.snake[i].mov_speed_y = (y_diff) * 10.0;
            self.snake[i].update(args);

            if i < 3 {
                // This is necessary so we dont game over due to the nature of tail following algorithm
                continue;
            }

            let head = self.snake[0];
            let ele = self.snake[i];

            if ele.intersect(&head) {
                self.game_over();
                return;
            }
        }
    }

    pub fn input(&mut self, args: &ButtonArgs) {
        if args.state == ButtonState::Press {
            match args.button {
                Button::Keyboard(piston::Key::W) => {
                    if self.last_key != Some(Button::Keyboard(piston::Key::S)) {
                        self.update_snake(0.0, -1.0);
                        self.last_key = Some(args.button);
                    }
                }
                Button::Keyboard(piston::Key::A) => {
                    if self.last_key != Some(Button::Keyboard(piston::Key::D)) {
                        self.update_snake(-1.0, 0.0);
                        self.last_key = Some(args.button);
                    }
                }
                Button::Keyboard(piston::Key::S) => {
                    if self.last_key != Some(Button::Keyboard(piston::Key::W)) {
                        self.update_snake(0.0, 1.0);
                        self.last_key = Some(args.button);
                    }
                }
                Button::Keyboard(piston::Key::D) => {
                    if self.last_key != Some(Button::Keyboard(piston::Key::A)) {
                        self.update_snake(1.0, 0.0);
                        self.last_key = Some(args.button);
                    }
                }
                _ => (),
            }
        }
    }
}
