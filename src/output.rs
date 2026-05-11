use std::io::stdout;

use ratatui::{DefaultTerminal, Frame};

use crate::state::GameState;

pub fn output(state: &GameState, terminal: &mut DefaultTerminal) {
    if let Some(menu) = &state.menu {
        terminal.draw(|frame| menu.render(frame)).unwrap();
    }
}

pub trait Render {
    fn render(&self, frame: &mut Frame);
}
