use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn input() -> KeyCode {
    loop {
        match event::read().unwrap() {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => key_event.code,
            _ => continue,
        };
    }
}
