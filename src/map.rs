use crate::towers::Tower;

#[derive(Debug)]
pub struct Map {
    size: (usize, usize),
    path: Vec<(usize, usize)>,
    towers: Vec<(usize, usize, Tower)>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            size: (6, 12),
            path: Vec::new(),
            towers: Vec::new(),
        }
    }

    pub fn set_path(&mut self, path: Vec<(usize, usize)>) {
        self.path = path;
    }
}