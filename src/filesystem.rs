use std::path::PathBuf;
use std::fs::read_to_string;

pub struct File {
    path: PathBuf,
    lines: Vec<Vec<char>>
}

impl File {
    pub fn load(other: PathBuf) -> Option<File> {
        let lines = match read_to_string(&other) {
            Ok(value) => value.lines().map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>(),
            Err(_) => return None
        };

        Some(File{
            path: other,
            lines
        })
    }

    pub fn get_lines_mut(&mut self) -> &mut Vec<Vec<char>> {
        &mut self.lines
    }

    pub fn get_lines(&self) -> &Vec<Vec<char>> {
        &self.lines
    }
}
