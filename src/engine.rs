use crossterm::event::KeyCode;

use crate::state::GameState;

pub fn engine(input: KeyCode, mut state: GameState) -> GameState {
    if input == KeyCode::Esc {
        state.quit = true;
    }
    let menu = &mut state.menu;
    if let Some(menu) = menu {
        match input {
            KeyCode::Up | KeyCode::Char('k') => menu.prev(),
            KeyCode::Down | KeyCode::Char('j') => menu.next(),
            KeyCode::Enter | KeyCode::Char('l') => match menu.current {
                crate::features::menu::state::MenuOptions::New => todo!(),
                crate::features::menu::state::MenuOptions::Save => todo!(),
                crate::features::menu::state::MenuOptions::Load => todo!(),
                crate::features::menu::state::MenuOptions::Quit => state.quit = true,
            },
            _ => (),
        }
    }

    state
}
