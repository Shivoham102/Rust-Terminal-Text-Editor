use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::cursor::{Hide, MoveDown, MoveLeft, MoveRight, MoveTo, MoveToColumn, MoveToRow, MoveUp, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Error, Write};#[derive(Copy, Clone)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}
#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}
pub struct Terminal;
impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{x: 0,y: 0})?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn move_cursor_to(position: Position) -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }
    pub fn move_cursor(action: KeyCode) -> Result<(), std::io::Error> {
        match action {
            KeyCode::Left => {
                execute!(stdout(), MoveLeft(1))?;
            }
            KeyCode::Right => {
                execute!(stdout(), MoveRight(1))?;
            }
            KeyCode::Up => {
                execute!(stdout(), MoveUp(1))?;
            }
            KeyCode::Down => {
                execute!(stdout(), MoveDown(1))?;
            }
            KeyCode::Home => {
                execute!(stdout(), MoveToColumn(0))?;
            }
            KeyCode::End => {
                execute!(stdout(), MoveToColumn(Terminal::size()?.width))?;
            }
            KeyCode::PageUp => {
                execute!(stdout(), MoveToRow(0))?;
            }
            KeyCode::PageDown => {
                execute!(stdout(), MoveToRow(Terminal::size()?.height))?;
            }
            _ => ()
        }
        Ok(())
    }
    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        Ok(Size {height, width})
    }
    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }
    pub fn print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}