use std::io::Error;

use super::terminal::{Position, Size, Terminal};
mod buffer;
use buffer::Buffer;

#[derive(Default)]
pub struct View {
    pub buffer: Buffer
}

impl View {
    pub fn render(&self) -> Result<(), Error> {
        let Size {height, width} = Terminal::size()?;
        // Terminal::clear_line()?;

        for current_row in 0..height {
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(current_row as usize) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
                continue;
            }
            if current_row == height / 3 {
                Self::draw_welcome_message(width, current_row)?;
            } else{
                Self::draw_empty_row()?;
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message(width: u16, current_row: u16) -> Result<(), Error> {
        let welcome_msg = "Welcome to the Terminal Text Editor";
        let length = welcome_msg.len();
        Terminal::move_cursor_to(Position {x: width/2 - (length as u16/2), y: current_row})?;
        Terminal::print(&welcome_msg)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
}