use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
// use termwiz::input::{KeyCode, KeyEvent, Modifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `Ctrl-c`
        KeyCode::Char('c') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }

        KeyCode::Esc => app.on_esc(),

        KeyCode::Down | KeyCode::Tab => app.on_down(),

        KeyCode::Up | KeyCode::BackTab => app.on_up(),

        KeyCode::Char('v') => app.toggle_view(),

        _ => return Ok(()),
    }
    Ok(())
}
