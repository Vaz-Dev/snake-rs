use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier},
    text::Line,
    widgets::{Block, List, ListItem, Padding},
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
        let areas = Layout::vertical(constraints).split(frame.area());

        let top = areas[0];
        let title = Line::from("Placeholder");
        frame.render_widget(title.centered(), top);

        let block = Block::default().padding(Padding::new(2, 2, 1, 1));
        let middle = areas[1];
        let items: Vec<ListItem> = self
            .options
            .iter()
            .map(|option| ListItem::new(option.to_string()))
            .collect();
        let list = List::new(items)
            .block(block)
            .style(Color::White)
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, middle, &mut self.state.borrow_mut());
    }
}
