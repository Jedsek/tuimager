use std::io;
use std::path::PathBuf;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use tuimager::app::{App, AppResult};
use tuimager::event::{Event, EventHandler};
use tuimager::handler::handle_key_events;
use tuimager::tui::Tui;
use tuimager::utils::find_images;

fn main() -> AppResult<()> {
    // Create an application.
    let image_paths = {
        let paths = std::env::args_os().skip(1).map(PathBuf::from).collect();
        find_images(paths)
    };
    let mut app = App::new(image_paths);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(230);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
