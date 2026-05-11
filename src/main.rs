use crate::{engine::engine, input::input, output::output, state::GameState};
mod engine;
mod features {
    pub mod menu;
}
mod input;
mod output;
mod state;

fn main() {
    let mut terminal = ratatui::init();
    let mut prev_state = GameState::new();
    {
        output(&prev_state, &mut terminal);
    }
    loop {
        let input = input();
        let state = engine(input, prev_state);
        output(&state, &mut terminal);
        if state.quit {
            break;
        }
        prev_state = state;
    }
    ratatui::restore();
}
