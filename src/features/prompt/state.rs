use crossterm::event::KeyCode;
use ratatui::Frame;

use crate::{input::input, output::Render, terminal::get_terminal};

pub struct Prompt {
    quesion: &'static str,
    input: String,
    done: bool,
}

impl Prompt {
    fn new(question: &'static str) -> Self {
        Prompt {
            input: String::from(""),
            done: false,
            quesion: question,
        }
    }
}

pub fn prompt(question: &'static str) -> String {
    let mut new_prompt = Prompt::new(question);
    let mut terminal = get_terminal();
    while !new_prompt.done {
        terminal.draw(|frame| new_prompt.render(frame)).unwrap();
        let input = input();
        if input == KeyCode::Enter {
            new_prompt.done = true;
        } else if let KeyCode::Char(char) = input {
            new_prompt.input.push(char);
        }
    }

    new_prompt.input
}
