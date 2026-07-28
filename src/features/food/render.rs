use crate::{features::food::state::Food, output::Render};

impl Render for Food {
    fn render(&self, frame: &mut ratatui::Frame) {
        self.coord.render(frame);
    }
}
