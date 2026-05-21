use crate::features::menu::state::Menu;
use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub quit: bool,
    pub startup: bool,
    pub current: Option<String>,
    #[serde(skip)]
    pub menu: Option<Menu>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            quit: false,
            startup: true,
            current: None,
            menu: Some(Menu::new()),
        }
    }

    pub fn save(state: &Self) -> Result<(), io::Error> {
        let save_name = match &state.current {
            None => unreachable!(),
            Some(name) => name,
        };
        let file_name = format!("{save_name}.json");

        let serialized = serde_json::to_string(state)?;
        fs::write(file_name, serialized)?;

        Ok(())
    }

    pub fn load(state: &mut Self, save_name: String) -> Result<(), io::Error> {
        let file_name = format!("{save_name}.json");
        let serialized = fs::read_to_string(file_name)?;
        let loaded_state: GameState = serde_json::from_str(serialized.as_str())?;
        *state = loaded_state;

        Ok(())
    }
}
