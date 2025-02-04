use ratatui::symbols::border;
use ratatui::widgets::ListState;
use ratatui::DefaultTerminal;
use ratatui::widgets::List;
use ratatui::widgets::Block;

use crate::{editor::{Cursor, EditorState}, filesystem::File};

struct Application {
    file: Option<File>,
    list_state: ListState,

    // Application status
    running: bool
}

impl Application {
    fn new() -> Self {
        Self {
            file: None,
            list_state: ListState::default(),
            running: true
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal, file: File) {
        self.file = Some(file);
        let mut cursor: Cursor = Cursor::new();
        let mut editor_state: EditorState = EditorState::Normal;

        while self.running {
            
        }
    }

    fn render(&self, terminal: &mut DefaultTerminal, cursor: &Cursor, state: &EditorState, file: Option<&File>) {
        let frame = terminal.get_frame();

        let file_list = match &self.file {
            Some(file) => {
                file.get_lines().iter().map(|x|
                    x.into_iter().collect::<String>()
                ).collect()
            }
            None => List::default()
        }.block(Block::bordered().border_set(border::THICK));

    }
}
