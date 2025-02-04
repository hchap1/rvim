mod application;
mod filesystem;
mod editor;

use crate::editor::{Cursor, EditorState};
use crate::filesystem::File;

fn main() {
    let mut terminal = ratatui::init();
    ratatui::restore();
}
