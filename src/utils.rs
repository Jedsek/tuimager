use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use path_absolutize::Absolutize;
use std::{
    io,
    path::{Path, PathBuf},
};
use tui::layout::{Constraint, Direction, Layout, Rect};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn clear_screen() {
    execute!(io::stdout(), Clear(ClearType::All)).expect("failed to clear screen.");
}

#[rustfmt::skip]
pub fn find_images(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut new_paths = vec![];

    for path in paths.into_iter().filter(|p| p.exists()) {
        let path = path
            .absolutize().expect("failed to parse args to valid path")
            .into_owned();

        if path.is_dir() {
            for sub_path in path.read_dir().expect("read_dir call failed").flatten().map(|a| a.path()).filter(|p| is_image(p)) {
                new_paths.push(sub_path);
            }
        } else if is_image(&path) {
            new_paths.push(path);
        }
    }

    new_paths
}

pub fn is_image(path: &Path) -> bool {
    let image_suffix = ["jpg", "jpeg", "gif", "png", "bmp", "svg", "ico"];
    path.is_file() && image_suffix.contains(&path.extension().unwrap().to_str().unwrap())
}

// pub fn to_sixel_image(path: &Path) -> String {}
