use crate::features::menu::state::Menu;
use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub quit: bool,
    pub current: Option<GameData>,
    #[serde(skip)]
    pub menu: Option<Menu>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            quit: false,
            current: None,
            menu: Some(Menu::new()),
        }
    }

    pub fn save(&self, save_name: String) -> Result<(), io::Error> {
        let file_name = format!("{save_name}.json");

        let serialized = serde_json::to_string(&self)?;
        fs::write(file_name, serialized)?;

        Ok(())
    }

    pub fn load(&mut self, save_name: String) -> Result<(), io::Error> {
        let file_name = format!("{save_name}.json");
        let serialized = fs::read_to_string(file_name)?;
        let loaded_state: GameState = serde_json::from_str(serialized.as_str())?;
        *self = loaded_state;

        Ok(())
    }

    pub fn start(&mut self) {
        self.menu = None;
        self.current = Some(GameData::new());
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameData {
    length: u16,
}

impl GameData {
    fn new() -> Self {
        Self { length: 3 }
    }
}
