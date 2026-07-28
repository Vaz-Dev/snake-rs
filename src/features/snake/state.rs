use std::{cell::RefCell, rc::Rc};

use crate::features::coord::state::Coord;

pub struct Snake {
    pub position: Coord,
    pub direction: Direction,
    pub next: Option<Rc<RefCell<Snake>>>,
}

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    pub fn new(length: u16) -> Rc<RefCell<Self>> {
        let snake = Snake {
            position: Coord {
                x: 10,
                y: 10,
                char: Some('@'),
                color: Some(ratatui::style::Color::Green),
            },
            direction: Direction::Up,
            next: None,
        };
        let ref_cell = RefCell::new(snake);
        let rc = Rc::new(ref_cell);
        Snake::new_parts(rc.clone(), length - 1);
        rc
    }

    pub fn new_parts(this: Rc<RefCell<Self>>, quantity: u16) {
        if quantity == 0 {
            return;
        }
        if let Some(next) = &this.borrow().next {
            Snake::new_parts(next.clone(), quantity);
        } else {
            let (position, direction) = {
                let current = this.borrow();
                (current.position.clone(), current.direction.clone())
            };
            let [x, y] = match direction {
                Direction::Up => [position.x, position.y + 1],
                Direction::Down => [position.x, position.y - 1],
                Direction::Left => [position.x - 1, position.y],
                Direction::Right => [position.x + 1, position.y],
            };
            let part: Snake = Snake {
                position: Coord {
                    x,
                    y,
                    char: position.char,
                    color: position.color,
                },
                direction,
                next: None,
            };
            let ref_cell = RefCell::new(part);
            let rc = Rc::new(ref_cell);
            Snake::new_parts(rc.clone(), quantity - 1);
            this.borrow_mut().next = Some(rc);
        }
    }

    pub fn turn_and_move(this: Rc<RefCell<Self>>, direction: Option<Direction>) {
        let mut snake = this.borrow_mut();
        if let Some(next) = snake.next.clone() {
            Snake::turn_and_move(next, Some(snake.direction.clone()));
        }
        if let Some(new_direction) = direction {
            let is_opposite = match (&snake.direction, &new_direction) {
                (Direction::Up, Direction::Down) => true,
                (Direction::Down, Direction::Up) => true,
                (Direction::Left, Direction::Right) => true,
                (Direction::Right, Direction::Left) => true,
                _ => false,
            };
            if !is_opposite {
                snake.direction = new_direction;
            }
        };
        match snake.direction {
            Direction::Up => {
                snake.position.y -= 1;
            }
            Direction::Down => {
                snake.position.y += 1;
            }
            Direction::Left => {
                snake.position.x -= 1;
            }
            Direction::Right => {
                snake.position.x += 1;
            }
        };
    }
}
