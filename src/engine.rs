use crossterm::event::KeyCode;

use crate::state::GameState;

pub fn engine(input: KeyCode, mut state: GameState) -> GameState {
    if input == KeyCode::Char('q') {
        state.quit = true;
    }
    state
}
