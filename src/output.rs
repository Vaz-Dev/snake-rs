use std::io::stdout;

use ratatui::DefaultTerminal;

use crate::state::GameState;

pub fn output(state: &GameState, terminal: &DefaultTerminal) {
    if state.quit {
        println!("Thanks for playing!");
    }
}
