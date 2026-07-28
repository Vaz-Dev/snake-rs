use crate::{engine::engine, input::input, output::output, state::GameState};
mod engine;
mod terminal;
mod features {
    pub mod coord;
    pub mod food;
    pub mod menu;
    pub mod prompt;
    pub mod snake;
}
mod input;
mod output;
mod state;

fn main() {
    let mut prev_state = GameState::new();
    {
        output(&prev_state);
    }
    loop {
        let input = input();
        let state = engine(input, prev_state);
        output(&state);
        if state.quit {
            break;
        }
        prev_state = state;
    }
    ratatui::restore();
}
