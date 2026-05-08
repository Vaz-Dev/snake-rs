use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

pub fn input() -> KeyCode {
    loop {
        if event::poll(Duration::from_millis(16)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    return key_event.code
                }
                _ => continue,
            }
        }
    }
}
