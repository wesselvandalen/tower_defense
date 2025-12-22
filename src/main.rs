mod map;
mod towers;
mod margin;

use towers::*;
use map::Map;
use margin::Margin;

use std::io::Result as IOResult;
use std::io::{self, Write, stdout, Stdout};

use std::time::{Instant, Duration};

use crossterm::{cursor, execute, queue, terminal, QueueableCommand};
use crossterm::terminal::{
    enable_raw_mode, 
    disable_raw_mode,
    Clear, 
    ClearType
};
use crossterm::cursor::{Hide, MoveTo};
use crossterm::event::{
    self, 
    Event,
    KeyCode,
    KeyModifiers,
};
use crossterm::style::{
    Print,
    SetBackgroundColor,
    SetForegroundColor,
    Color,
};

fn main() -> IOResult<()> {
    let mut map = Map::new_map1();
    let mut margin = Margin::new(17);

    let mut terminal = stdout();

    // Hide cursor and enable raw_mode
    enable_raw_mode()?;
    terminal.queue(Hide)?;

    // timestamp for last time screen was drawn to. Used to cap fps
    let mut last_draw = Instant::now();

    loop {
        // This while block runs while there is a user event happening
        while event::poll(Duration::ZERO).unwrap() {
            let event =  event::read()?;
            handle_input(event, &mut terminal)?;
        }

        // Only draw to screen every 60 fps
        let et = last_draw.elapsed();
        if et >= Duration::from_millis(1000) / 60 {
            execute!(terminal, terminal::BeginSynchronizedUpdate)?;
            // Clear screen
            terminal.queue(Clear(ClearType::All))?;

            // Draw map on the screen
            map.draw(&mut terminal)?;

            // Draw margin on screen
            margin.draw(&mut terminal)?;

            execute!(terminal, terminal::EndSynchronizedUpdate)?;
            last_draw = Instant::now();
        }
    }
}


/// Handles user input
/// 
fn handle_input(event: Event, terminal: &mut Stdout) -> IOResult<()> {
    match event {
        Event::Key(key_event) => handle_key_input(key_event, terminal),
        _ => Ok(())
    }
}


/// Handles key events
/// 
fn handle_key_input(event: event::KeyEvent, terminal: &mut Stdout) -> IOResult<()> {
    // Ctrl + c to stop program
    if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL {
        close_program(terminal)?;
    }

    Ok(())
}


/// Logic to handle stopping the program
/// 
fn close_program(terminal: &mut Stdout) -> IOResult<()> {
    disable_raw_mode()?;

    queue!(
        terminal,
        Clear(ClearType::All),
        SetBackgroundColor(Color::Black),
        SetForegroundColor(Color::White),
        cursor::Show,
    )?;

    panic!();
}


/// Queues the print command to print a string, but it handles newline (\n) better
/// 
pub struct PrintLines<'a>(&'a str);

use crossterm::Command;
impl<'a> Command for PrintLines<'a> {
    fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
        cursor::SavePosition.write_ansi(f)?;

        for (i, line) in self.0.lines().enumerate() {
            Print(line).write_ansi(f)?;
            cursor::RestorePosition.write_ansi(f)?;
            cursor::MoveDown((i+1) as u16).write_ansi(f)?;
        }
        
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> IOResult<()> {
        Ok(())
    }
}