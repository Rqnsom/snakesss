use super::color::Color;
use super::food::Food;
use super::snake::{Corner, Direction, Snake};
use graphics::{DrawState, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::input::RenderArgs;
use piston::{Button, Key};

/// Very simple statemachine
enum Gamestate {
    Ongoing,

    /// String contains info about the game outcome
    Done(String),
}

pub struct Game {
    /// Something to render my snakey snacks
    gl: GlGraphics,

    /// Help for printing text
    glyph: GlyphCache<'static>,

    /// Player 1
    snek1: Snake,

    /// Player 2
    snek2: Snake,

    /// Simple game state
    gamestate: Gamestate,

    /// Beware! My snakes are eating a melon.
    food: Food,
}

impl Game {
    pub fn new(opengl: OpenGL) -> Game {
        let snake1 = Snake::new(Corner::UpperLeft);
        let snake2 = Snake::new(Corner::LowerRight);
        let food = Food::new(vec![&snake1, &snake2]);

        let glyph = GlyphCache::new(
            "assets/FiraSans-Regular.ttf",
            (),
            TextureSettings::new(),
        )
        .unwrap();

        Self {
            gl: GlGraphics::new(opengl),
            glyph,
            snek1: snake1,
            snek2: snake2,
            gamestate: Gamestate::Ongoing,
            food,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_, gl| {
            graphics::clear(*Color::Green.get(), gl);
        });

        self.food.render(&mut self.gl, args);
        self.snek1.render(&mut self.gl, args);
        self.snek2.render(&mut self.gl, args);

        if let Gamestate::Done(ref status) = self.gamestate {
            self.gl.draw(args.viewport(), |c, gl| {
                graphics::text::Text::new_color(*Color::GameEndText.get(), 32)
                    .draw(
                        format!("Game over: {}", status).as_str(),
                        &mut self.glyph,
                        &DrawState::default(),
                        c.transform.trans(100.0, 120.0),
                        gl,
                    )
                    .unwrap();
            })
        };
    }

    /// Update snake movement and check for any suspicios actions which
    /// can affect the gamestate
    pub fn update(&mut self) {
        if let Gamestate::Ongoing = self.gamestate {
            let player1_location = self.snek1.update(&self.food);
            let player2_location = self.snek2.update(&self.food);

            // Frontal collision! Possibility for this depends on:
            // - starting offset of both snakes
            // - oddness of the window width/height
            //
            // With the current setup, this should be an impossible scenario
            if player1_location == player2_location {
                self.gamestate = Gamestate::Done(String::from("Draw"));
                return;
            }

            let snake1_collision = self.snek1.check_any_collisons(&self.snek2);
            let snake2_collision = self.snek2.check_any_collisons(&self.snek1);

            if snake1_collision && snake2_collision {
                self.gamestate = Gamestate::Done(String::from("Draw"));
                return;
            }
            if snake1_collision {
                self.gamestate = Gamestate::Done(format!("{} won", self.snek2));
                return;
            }
            if snake2_collision {
                self.gamestate = Gamestate::Done(format!("{} won", self.snek1));
                return;
            }

            if player1_location == self.food.pos()
                || player2_location == self.food.pos()
            {
                self.food = Food::new(vec![&self.snek1, &self.snek2]);
            }
        }
    }

    /// Return true in case game is over. Truth is to be revealed eventually.
    pub fn check_button(&mut self, btn: &Button) -> bool {
        if let Gamestate::Done(_) = self.gamestate {
            return true;
        }

        match *btn {
            Button::Keyboard(Key::Up) => self.snek1.set_direcion(Direction::Up),
            Button::Keyboard(Key::Down) => {
                self.snek1.set_direcion(Direction::Down)
            }
            Button::Keyboard(Key::Right) => {
                self.snek1.set_direcion(Direction::Right)
            }
            Button::Keyboard(Key::Left) => {
                self.snek1.set_direcion(Direction::Left)
            }
            Button::Keyboard(Key::W) => self.snek2.set_direcion(Direction::Up),
            Button::Keyboard(Key::S) => {
                self.snek2.set_direcion(Direction::Down)
            }
            Button::Keyboard(Key::D) => {
                self.snek2.set_direcion(Direction::Right)
            }
            Button::Keyboard(Key::A) => {
                self.snek2.set_direcion(Direction::Left)
            }
            _ => (),
        };

        false
    }
}
