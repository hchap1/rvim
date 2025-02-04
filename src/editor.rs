
pub enum EditorState {
    Normal,
    Insert,
    Replace,
    Overwrite,
    Command
}

pub struct Cursor {
    x: usize,
    y: usize
}

impl Cursor {
    pub fn new() -> Self {
        Cursor { x: 0, y: 0 }
    }
}
