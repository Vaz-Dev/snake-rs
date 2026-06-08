use crate::features::{menu::state::Menu, snake::state::Snake};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, fs, io, rc::Rc};

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub quit: bool,
    #[serde(skip)]
    pub current: Option<GameData>,
    #[serde(skip)]
    pub menu: Option<Menu>,
    pub score: u16,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            quit: false,
            current: None,
            menu: Some(Menu::new()),
            score: 3,
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
        let mut loaded_state: GameState = serde_json::from_str(serialized.as_str())?;
        loaded_state.current = Some(GameData::new(loaded_state.score));
        *self = loaded_state;

        Ok(())
    }

    pub fn start(&mut self) {
        self.menu = None;
        self.current = Some(GameData::default());
    }
}

pub struct GameData {
    pub snake: Rc<RefCell<Snake>>,
}

impl GameData {
    fn new(length: u16) -> Self {
        Self {
            snake: Snake::new(length),
        }
    }
}

impl Default for GameData {
    fn default() -> Self {
        GameData::new(3)
    }
}
