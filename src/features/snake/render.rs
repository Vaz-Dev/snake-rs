use ratatui::Frame;
use std::{cell::RefCell, rc::Rc};

use crate::{features::snake::state::Snake, output::Render};

impl Render for Rc<RefCell<Snake>> {
    fn render(&self, frame: &mut Frame) {
        let current = self.borrow();
        current.position.render(frame);
        if let Some(next) = &current.next {
            next.render(frame);
        }
    }
}
