use ratatui::Frame;

use crate::{state::GameState, terminal::get_terminal};

pub fn output(state: &GameState) {
    let mut terminal = get_terminal();
    if let Some(menu) = &state.menu {
        terminal.draw(|frame| menu.render(frame)).unwrap();
    } else if let Some(current_game) = &state.current {
        terminal
            .draw(|frame| {
                current_game.snake.render(frame);
                current_game.food.render(frame);
            })
            .unwrap();
    }
}

pub trait Render {
    fn render(&self, frame: &mut Frame);
}
