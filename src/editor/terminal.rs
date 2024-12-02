use colored::ColoredString;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{Error, stdout, Write};

use crate::highlighter::Highlighter;

#[derive(Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize
}

#[derive(Default)]
pub struct Terminal {
    pub highlighter: Highlighter
}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn move_caret(pos: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(pos.col as u16, pos.row as u16))
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn _print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }

    pub fn vec_print(v: &Vec<ColoredString>) -> Result<(), Error> {
        for i in 0..v.len() {
            queue!(stdout(), Print(
                format!("{}", v[i])
            ))?;
        }
        
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        let w_usize = width as usize;
        let h_usize = height as usize;

        Ok(Size { width: w_usize, height: h_usize })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}