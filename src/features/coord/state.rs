use ratatui::style::Color;

pub struct Coord {
    pub x: u16,
    pub y: u16,
    pub char: Option<char>,
    pub color: Option<Color>,
}
