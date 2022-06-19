//! # Simple 2-player snake game
//!
//! ## Movement
//! - Red player: _arrow keys_
//! - Blue player: _WASD keys_
//!
//! Every melon consumed lengthens the snake.
//! - How to win: **Avoid losing!**
//! - How to lose: **Hit the enemy snake**!
//! - How to make peace: **Frontal collision with the other snake** (ain't
//!   these snakes peculiar creatures...)
//!
//! General strategy:
//!  - _The bigger your snake becomes, the higher the chance your opponent will
//!  not be able to avoid your elooongated snake body._
//!
//! ## About
//! This was done as an educational challenge as a one-day *hackathon*.
//! After watching this [video](https://www.youtube.com/watch?v=HCwMb0KslX8),
//! which sets up the coding ground for the basic snake game.
//!
//! I wish I had more time to implement multiple gamestates (like *MainMenu,
//! ScoreScree...*), but it is what it is.
//!
//! Enjoy!
//!
use game::Game;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{ButtonEvent, ButtonState, EventLoop};

mod color;
mod config;
mod food;
mod game;
mod position;
mod snake;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Snakesss",
        [config::WINDOW_HEIGHT, config::WINDOW_WIDTH],
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    // Create a new game and run it.
    let mut game = Game::new(opengl);

    let mut events = Events::new(EventSettings::new()).ups(16);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                // We are 'breaking' from here so to allow the players to
                // check latest status and exit on any keyboard click
                if game.check_button(&k.button) {
                    break;
                }
            }
        }

        if e.update_args().is_some() {
            game.update();
        }
    }
}
