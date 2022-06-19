use super::color::Color;
use super::config;
use super::position::Pos;
use super::snake::Snake;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use rand::Rng;

pub struct Food {
    p: Pos,
}

impl Food {
    /// Generate new `food` on a random location
    ///
    /// It needs to know where snakes are so food is not generated
    /// on a snake!
    pub fn new(snakes: Vec<&Snake>) -> Food {
        let mut rng = rand::thread_rng();

        // It's not true that it never loops!
        #[allow(clippy::never_loop)]
        let p = loop {
            let melon_pos = Pos {
                x: rng.gen_range(0..config::ROWS),
                y: rng.gen_range(0..config::COLS),
            };

            for snake in snakes.iter() {
                for snake_part in snake.body.iter() {
                    if *snake_part == melon_pos {
                        continue;
                    }
                }
            }

            break melon_pos;
        };

        Self { p }
    }

    pub fn pos(&self) -> Pos {
        self.p
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let square = graphics::rectangle::square(
            (self.p.x * config::SNAKE_WIDTH) as f64,
            (self.p.y * config::SNAKE_WIDTH) as f64,
            config::SNAKE_WIDTH as f64,
        );

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(*Color::Melon.get(), square, transform, gl);
        })
    }
}
