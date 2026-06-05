use ratatui::{
    layout::{Constraint, Layout},
    style::Stylize,
    text::Line,
    Frame,
};

use crate::{features::prompt::state::Prompt, output::Render};

impl Render for Prompt {
    fn render(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [top, main] = frame.area().layout(&vertical);

        let question = Line::from(self.question).bold().blue();
        let answer_prompt = Line::from(self.input.as_str());
        frame.render_widget(question, top);
        frame.render_widget(answer_prompt, main);
    }
}
