use crate::towers::Tower;

use std::io::{Write, Stdout};

use crossterm::terminal::{Clear, ClearType};

use crossterm::{queue, QueueableCommand, cursor};
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor, Print};

use std::io::Result as IOResult;

#[derive(Debug)]
pub struct Map {
    size: (u16, u16),
    path: Vec<(u16, u16)>,
    towers: Vec<(u16, u16, Tower)>,
}

impl Map {
    pub fn new_empty() -> Self {
        Self {
            size: (6, 12),
            path: Vec::new(),
            towers: Vec::new(),
        }
    }

    pub fn new_map1() -> Self {
        let path = vec![
            (1, 1),
            (2, 2)
        ];

        Self {
            size: (6, 12),
            path,
            towers: Vec::new(),
        }
    }

    pub fn new_map2() -> Self {
        let path = vec![
            (2, 2),
            (3, 3)
        ];

        Self {
            size: (6, 12),
            path,
            towers: Vec::new(),
        }
    }


    /// Draws the map to a terminal
    /// 
    pub fn draw(&self, stdout: &mut Stdout) -> IOResult<()> {
        // Set background and foreground color to green
        let background = Color::Black;
        let foreground = Color::Green;
        queue!(
            stdout,
            SetBackgroundColor(background),
            SetForegroundColor(foreground),
        )?;

        // Loop over the entire map and set it to green
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                queue!(
                    stdout, 
                    cursor::MoveTo(x*2, y),
                    Print("██"),
                )?;
            }
        }


        // Loop over all path placements and draw them
        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
        for (x, y) in &self.path {
            queue!(
                stdout,
                cursor::MoveTo(*x*2, *y),
                Print("██"),
            )?;
        }

        stdout.flush()?;
        Ok(())
    }

    pub fn set_path(&mut self, path: Vec<(u16, u16)>) {
        self.path = path;
    }
}