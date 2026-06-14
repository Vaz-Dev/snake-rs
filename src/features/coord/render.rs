use ratatui::{layout::Rect, style::Styled, text::Text, Frame};

use crate::{features::coord::state::Coord, output::Render};

impl Render for Coord {
    fn render(&self, frame: &mut Frame) {
        if let (Some(char), Some(color)) = (self.char, self.color) {
            let coord_rect = Rect::new(self.x, self.y, 1, 1);
            let styled_char = Text::from(format!("{}", char)).set_style(color);

            frame.render_widget(styled_char, coord_rect);
        } else {
            panic!("tried to render a raw coord");
        }
    }
}
