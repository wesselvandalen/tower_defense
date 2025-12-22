use std::io::{Write, Stdout};
use std::io::Result as IOResult;

use crossterm::{queue, QueueableCommand};
use crossterm::terminal::{size};
use crossterm::cursor::MoveTo;
use crossterm::style::{
    Print,
    Color,
    SetBackgroundColor,
    SetForegroundColor
};

use crate::towers::Tower;

/// The information margin at the right of the screen
/// 
pub struct Margin {
    /// Width of the margin
    /// 
    width: u16,


    /// Remaining health of the user
    /// 
    user_health: usize,


    /// Remaining money of the the user
    /// 
    user_money: usize
}


impl Margin {
    /// Gives a new margin with given width and user
    /// 
    pub fn new(width: u16) -> Self {
        let user_health = 200;
        let user_money = 350;

        Self { width, user_health, user_money }
    }


    /// Draws the margin at the right of the screen.
    /// The margin contains useful info to the user such as:
    /// Money, Health, Round and a selection of towers to buy
    /// 
    pub fn draw(&self, stdout: &mut Stdout) -> IOResult<()> {
        // Set background and foreground color of text
        let background = Color::Black;
        let foreground = Color::White;
        
        queue!(
            stdout,
            SetBackgroundColor(background),
            SetForegroundColor(foreground),
        )?;
        

        // Print the margin
        let (stdout_x, stdout_y) = size()?;
        let x = stdout_x - self.width;
        for y in 0..stdout_y {
            queue!(
                stdout,
                MoveTo(x, y),
                Print("│"),
            )?;
        }

        let y = 2;
        for x in x+1..stdout_x {
            queue!(
                stdout,
                MoveTo(x, y),
                Print("─"),
            )?;
        }


        // Print the user information
        queue!(
            stdout,
            MoveTo(x+3, 0),
            SetForegroundColor(Color::Red),
            Print(format!("Health: {}", self.user_health)),

            MoveTo(x+3, 1),
            SetForegroundColor(Color::Yellow),
            Print(format!("Money : {}", self.user_money)),
        )?;

        
        // Draw all towers
        self.draw_towers(stdout)?;

        stdout.flush()?;
        Ok(())
    }


    /// Draws all the tower types in the margin
    /// 
    fn draw_towers(&self, stdout: &mut Stdout) -> IOResult<()> {
        let (stdout_x, _) = size()?;
        let x = stdout_x - self.width + 7;
        stdout.queue(MoveTo(x, 4))?;

        for tower in Tower::iter_all_towers() {
            tower.draw(stdout)?;
        }

        Ok(())
    }
    
}