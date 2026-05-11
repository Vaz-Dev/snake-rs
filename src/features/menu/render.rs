use ratatui::{
    layout::{Constraint, Layout},
    text::Line,
    Frame,
};

use crate::{features::menu::state::Menu, output::Render};

impl Render for Menu {
    fn render(&self, frame: &mut Frame) {
        let constraints = [
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ];
        let layout = Layout::vertical(constraints).spacing(1);
        let areas = Layout::vertical(constraints).split(frame.area());
        let top = areas[0];
        let title = Line::from("Placeholder");
        frame.render_widget(title.centered(), top);
    }
}
