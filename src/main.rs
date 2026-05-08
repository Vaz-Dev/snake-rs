use crate::{engine::engine, input::input, output::output, state::GameState};
mod engine;
mod features {
    pub mod menu;
}
mod input;
mod output;
mod state;

fn main() {
    let terminal = ratatui::init();
    let mut prev_state = GameState::new();
    loop {
        let input = input();
        let state = engine(input, prev_state);
        output(&state, &terminal);
        if state.quit {
            break;
        }
        prev_state = state;
    }
    ratatui::restore();
}
