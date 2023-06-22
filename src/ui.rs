use crate::app::App;
use crate::utils::centered_rect;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

const VIEW_MODE_WIDTH_PERCENT: u16 = 40;
const VIEW_MODE_HEIGHT_PERCENT: u16 = 40;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/tui-rs-revival/ratatui/tree/master/examples
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(86), Constraint::Percentage(14)])
        .split(frame.size());

    let image_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[0]);

    let image_list_chunk = image_chunks[0];
    // let image_preview_chunk = image_chunks[1];
    let footer_chunk = chunks[1];

    if app.view_mode {
        render_image(app, frame);
    } else {
        render_image_list(app, image_list_chunk, frame);
        render_footer(footer_chunk, frame);
    }
}

fn new_block(title: &'static str) -> Block {
    Block::default()
        .title(title)
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
}

pub fn render_image_list<B: Backend>(app: &mut App, chunk: Rect, frame: &mut Frame<B>) {
    let images = {
        let path_info: Vec<_> = app
            .images
            .items
            .iter()
            .map(|a| ListItem::new(a.to_str().unwrap()))
            .collect();
        List::new(path_info)
            .block(new_block("Images"))
            .highlight_symbol(">>")
            .highlight_style(Style::new().fg(Color::Green))
    };
    frame.render_stateful_widget(images, chunk, &mut app.images.state);
}
pub fn render_image<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    frame.render_widget(Clear, frame.size());
    let center_chunk = centered_rect(
        VIEW_MODE_WIDTH_PERCENT,
        VIEW_MODE_HEIGHT_PERCENT,
        frame.size(),
    );

    if app.images.items.is_empty() || app.images.state.selected().is_none() {
        let tips = Paragraph::new("There is no available images or you have't select it.")
            .block(new_block("Tips"));
        frame.render_widget(tips, center_chunk);
    }

    let conf = viuer::Config {
        x: center_chunk.x,
        y: center_chunk.y as i16,
        width: Some(center_chunk.width as u32),
        height: Some(center_chunk.height as u32),
        ..Default::default()
    };
    // let conf = viuer::Config::default();

    match app.images.state.selected() {
        None => (),
        Some(i) => {
            let path = &app.images.items[i];
            viuer::print_from_file(path, &conf).unwrap();
        }
    }
}
pub fn render_footer<B: Backend>(chunk: Rect, frame: &mut Frame<B>) {
    let text: Vec<Line> = vec![
        format!("{:<15} -> exit", "ctrl-c"),
        format!("{:<15} -> toggle images", "tab/shiftab"),
        format!("{:<15} -> view-mode for images", "v"),
        format!("{:<15} -> unselect or quit view-mode", "esc"),
        format!("{:<15} -> unselect or quit view-mode", "esc"),
        format!("{:<15} -> unselect or quit view-mode", "esc"),
        format!("{:<15} -> unselect or quit view-mode", "esc"),
    ]
    .into_iter()
    .map(Line::from)
    .collect();
    let footer = Paragraph::new(text).block(new_block("Footer"));
    frame.render_widget(footer, chunk);
}
