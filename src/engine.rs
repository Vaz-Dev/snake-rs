use crossterm::event::KeyCode;

use crate::{
    features::{
        menu::state::{Menu, MenuOptions},
        prompt::state::prompt,
        snake::state::{Direction, Snake},
    },
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
    } else if let Some(current_game) = &mut state.current {
        let snake = current_game.snake.clone();
        match input {
            KeyCode::Esc | KeyCode::Char('q') => state.menu = Some(Menu::new()),
            KeyCode::Down | KeyCode::Char('j') => {
                Snake::turn_and_move(snake, Some(Direction::Down))
            }
            KeyCode::Up | KeyCode::Char('k') => Snake::turn_and_move(snake, Some(Direction::Up)),
            KeyCode::Left | KeyCode::Char('h') => {
                Snake::turn_and_move(snake, Some(Direction::Left))
            }
            KeyCode::Right | KeyCode::Char('l') => {
                Snake::turn_and_move(snake, Some(Direction::Right))
            }
            _ => Snake::turn_and_move(snake, None),
        }
    }

    state
}
