use super::color::Color;
use super::config;
use super::food::Food;
use super::position::Pos;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::collections::LinkedList;
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

/// Since this is a hackaton, this guy decides color and direction!
pub enum Corner {
    UpperLeft,
    LowerRight,
}

impl Corner {
    fn get_starting_position(self) -> Vec<Pos> {
        const LOWEST_ROW: u32 = config::ROWS - 1;
        const LOWEST_COL: u32 = config::COLS - 1;

        match self {
            Corner::UpperLeft => vec![
                Pos { x: 0, y: 0 },
                Pos { x: 0, y: 1 },
                Pos { x: 0, y: 2 },
                Pos { x: 0, y: 3 },
            ],
            Corner::LowerRight => vec![
                Pos {
                    x: LOWEST_ROW,
                    y: LOWEST_COL,
                },
                Pos {
                    x: LOWEST_ROW,
                    y: LOWEST_COL - 1,
                },
                Pos {
                    x: LOWEST_ROW,
                    y: LOWEST_COL - 2,
                },
                Pos {
                    x: LOWEST_ROW,
                    y: LOWEST_COL - 3,
                },
            ],
        }
    }
}

pub struct Snake {
    pub body: LinkedList<Pos>,
    dir: Direction,
    color: Color,

    /// Previous direction must be kept, because keyboard event loop might
    /// handle multiple 'change direction' events, and all of those must be
    /// compared with the original previous direction.
    prev_dir: Direction,
}

impl Snake {
    pub fn new(corner: Corner) -> Snake {
        let (color, dir) = match corner {
            Corner::UpperLeft => (Color::Red, Direction::Right),
            Corner::LowerRight => (Color::Blue, Direction::Left),
        };

        Self {
            body: LinkedList::from_iter(
                (corner.get_starting_position()).into_iter(),
            ),
            dir,
            color,
            prev_dir: dir,
        }
    }

    pub fn set_direcion(&mut self, direction: Direction) {
        self.dir = match direction {
            Direction::Down if self.prev_dir != Direction::Up => {
                Direction::Down
            }
            Direction::Up if self.prev_dir != Direction::Down => Direction::Up,
            Direction::Left if self.prev_dir != Direction::Right => {
                Direction::Left
            }
            Direction::Right if self.prev_dir != Direction::Left => {
                Direction::Right
            }
            _ => self.dir,
        };
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&pos| {
                graphics::rectangle::square(
                    (pos.x * config::SNAKE_WIDTH) as f64,
                    (pos.y * config::SNAKE_WIDTH) as f64,
                    config::SNAKE_WIDTH as f64,
                )
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            let mut color = *self.color.get();
            squares.into_iter().for_each(|square| {
                graphics::rectangle(color, square, transform, gl);

                // Let's have some fun!
                color[0] += 0.012;
                color[1] += 0.012;
                color[2] += 0.012;
            });
        })
    }

    /// Check collisions with itself and other snake
    /// Return true if any collisions occur
    pub fn check_any_collisons(&self, other_snake: &Snake) -> bool {
        let mut self_iter = self.body.iter();
        let front = self_iter.next().unwrap();

        for snake_part in self_iter {
            if snake_part == front {
                return true;
            }
        }

        other_snake
            .body
            .iter()
            .any(|snake_part| snake_part == front)
    }

    /// Return new snake location
    pub fn update(&mut self, food: &Food) -> Pos {
        let mut front = *self.body.front().expect("Snake has nobody :-)");

        self.update_location(&mut front);

        // Well, not hungry anymore!
        if front != food.pos() {
            self.body.pop_back();
        }

        self.body.push_front(front);
        front
    }

    fn update_location(&mut self, front: &mut Pos) {
        const GRID_DOWN: u32 = config::COLS - 1;
        const GRID_UP: u32 = 0;
        const GRID_RIGHT: u32 = config::ROWS - 1;
        const GRID_LEFT: u32 = 0;

        match self.dir {
            Direction::Left => {
                front.x = match front.x == GRID_LEFT {
                    true => GRID_RIGHT,
                    _ => front.x - 1,
                };
            }
            Direction::Right => {
                front.x = match front.x == GRID_RIGHT {
                    true => GRID_LEFT,
                    _ => front.x + 1,
                };
            }
            Direction::Down => {
                front.y = match front.y == GRID_DOWN {
                    true => GRID_UP,
                    _ => front.y + 1,
                };
            }
            Direction::Up => {
                front.y = match front.y == GRID_UP {
                    true => GRID_DOWN,
                    _ => front.y - 1,
                };
            }
        }

        self.prev_dir = self.dir;
    }
}

impl fmt::Display for Snake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} snake", self.color)
    }
}
