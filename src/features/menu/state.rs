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
