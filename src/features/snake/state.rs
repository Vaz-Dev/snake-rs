use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::features::coord::state::Coord;

pub struct Snake {
    pub position: Coord,
    direction: Direction,
    prev: Option<Weak<RefCell<Snake>>>,
    next: Option<Rc<RefCell<Snake>>>,
}

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    fn new(length: u16) -> Rc<RefCell<Self>> {
        let snake = Snake {
            position: Coord {
                x: 10,
                y: 10,
                char: Some('@'),
                color: Some(ratatui::style::Color::Green),
            },
            direction: Direction::Up,
            prev: None,
            next: None,
        };
        let ref_cell = RefCell::new(snake);
        let rc = Rc::new(ref_cell);
        rc.borrow_mut().next = Some(Snake::new_parts(rc.clone(), length - 1));
        rc
    }

    fn new_parts(this: Rc<RefCell<Self>>, quantity: u16) -> Rc<RefCell<Snake>> {
        let (position, direction) = {
            let current = this.borrow();
            (current.position.clone(), current.direction.clone())
        };
        let [x, y] = match direction {
            Direction::Up => [position.x, position.y - 1],
            Direction::Down => [position.x, position.y + 1],
            Direction::Left => [position.x + 1, position.y],
            Direction::Right => [position.x - 1, position.y],
        };
        let part: Snake = Snake {
            position: Coord {
                x,
                y,
                char: position.char,
                color: position.color,
            },
            direction,
            prev: Some(Rc::downgrade(&this)),
            next: None,
        };
        let ref_cell = RefCell::new(part);
        let rc = Rc::new(ref_cell);
        if quantity != 0 {
            rc.clone().borrow_mut().next = Some(Snake::new_parts(rc.clone(), quantity - 1));
        }
        rc
    }

    fn turn_and_move(this: Rc<RefCell<Self>>, direction: Option<Direction>) {
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
                snake.position.y += 1;
            }
            Direction::Down => {
                snake.position.y -= 1;
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
