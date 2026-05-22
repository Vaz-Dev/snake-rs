use std::sync::{LazyLock, Mutex};

use ratatui::DefaultTerminal;

static TERMINAL: LazyLock<Mutex<DefaultTerminal>> = LazyLock::new(|| Mutex::new(ratatui::init()));
pub fn get_terminal() -> std::sync::MutexGuard<'static, DefaultTerminal> {
    TERMINAL.lock().unwrap()
}
