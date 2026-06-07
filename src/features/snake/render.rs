use std::{cell::RefCell, rc::Rc};

use crate::{features::snake::state::Snake, output::Render};

impl Render for Rc<RefCell<Snake>> {
    fn render(&self, frame: &mut ratatui::Frame) {}
}
