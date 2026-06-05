use crossterm::event::KeyCode;

use crate::{
    features::{menu::state::MenuOptions, prompt::state::prompt},
    state::GameState,
};

pub fn engine(input: KeyCode, mut state: GameState) -> GameState {
    if input == KeyCode::Esc {
        state.quit = true;
    }
    let menu = &mut state.menu;
    if let Some(menu_option) = menu {
        match input {
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Tab => menu_option.next(),
            KeyCode::Up | KeyCode::Char('k') => menu_option.prev(),
            KeyCode::Enter | KeyCode::Char(' ') => match menu_option.current {
                MenuOptions::Save => {
                    if let Some(save_name) = prompt("Save as?") {
                        state.save(save_name).expect("save failed");
                    } else {
                        return state;
                    }
                }
                MenuOptions::Load => {
                    if let Some(save_name) = prompt("Which save file would you like to load?") {
                        state.load(save_name).expect("load failed")
                    } else {
                        return state;
                    };
                }
                MenuOptions::Quit => state.quit = true,
                MenuOptions::New => {
                    if state.current.is_some() {
                        state = GameState::new()
                    }
                    state.start();
                }
                MenuOptions::Continue => state.start(),
            },
            _ => (),
        }
    }

    state
}
