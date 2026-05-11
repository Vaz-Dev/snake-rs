use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Menu {
    current: MenuOptions,
    options: Vec<MenuOptions>,
}

#[derive(Serialize, Deserialize)]
pub enum MenuOptions {
    New,
    Save,
    Load,
    Quit,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            current: MenuOptions::New,
            options: vec![MenuOptions::New, MenuOptions::Load, MenuOptions::Quit],
        }
    }
}
