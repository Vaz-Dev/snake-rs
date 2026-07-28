use ratatui::style::Color;

use crate::features::coord::state::Coord;

pub struct Food {
    pub coord: Coord,
}

impl Food {
    pub fn new() -> Self {
        Food {
            coord: Coord {
                x: 10,
                y: 10,
                char: Some('*'),
                color: Some(Color::Yellow),
            },
        }
    }
}
