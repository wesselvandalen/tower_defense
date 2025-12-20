mod map;
mod towers;
mod margin;

use towers::*;
use map::Map;
use margin::Margin;

use std::io::Result as IOResult;
use std::io::{self, Write, stdout};

use std::time::{Instant, Duration};

use crossterm::{QueueableCommand};
use crossterm::terminal::{enable_raw_mode, Clear, ClearType};
use crossterm::cursor::Hide;

fn main() -> IOResult<()> {
    let mut map = Map::new_map1();
    let mut margin = Margin::new(18);

    let mut terminal = stdout();

    // Hide cursor and enable raw_mode
    enable_raw_mode()?;
    terminal.queue(Hide)?;

    loop {
        let start_time = Instant::now();

        // Clear screen
        terminal.queue(Clear(ClearType::All))?;

        // Draw map on the screen
        map.draw(&mut terminal)?;

        // Draw margin on screen
        margin.draw(&mut terminal)?;


        // do nothing to keep fps capped at {insert desired fps here}
        let et = start_time.elapsed();
        while et < Duration::from_millis(1000) / 60 {}
    }

    Ok(())
}
