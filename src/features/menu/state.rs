use std::{
    cell::RefCell,
    fmt::{Display, Formatter, Result},
};

use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

pub struct Menu {
    pub current: MenuOptions,
    pub options: Vec<MenuOptions>,
    pub state: RefCell<ListState>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
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
            state: RefCell::new(ListState::default().with_selected(Some(0))),
        }
    }

    pub fn prev(&mut self) {
        self.current = {
            let index = self
                .options
                .iter()
                .position(|opt| opt == &self.current)
                .expect("Menu was supposed to be an Vec");
            if index == 0 {
                self.state.borrow_mut().select_last();
                self.options[self.options.len() - 1].clone()
            } else {
                self.state.borrow_mut().select_previous();
                self.options[index - 1].clone()
            }
        }
    }

    pub fn next(&mut self) {
        self.current = {
            let index = self
                .options
                .iter()
                .position(|opt| opt == &self.current)
                .expect("Menu was supposed to be an Vec");
            if index == self.options.len() - 1 {
                self.state.borrow_mut().select_first();
                self.options[0].clone()
            } else {
                self.state.borrow_mut().select_next();
                self.options[index + 1].clone()
            }
        }
    }
}

impl Display for MenuOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MenuOptions::New => write!(f, "New Game"),
            MenuOptions::Save => write!(f, "Save"),
            MenuOptions::Load => write!(f, "Load"),
            MenuOptions::Quit => write!(f, "Quit"),
        }
    }
}
