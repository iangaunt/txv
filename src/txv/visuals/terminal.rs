use colored::ColoredString;
use crossterm::{
    cursor::{Hide, MoveTo, SetCursorStyle::BlinkingBar, Show}, queue, style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}
};
use std::io::{Error, stdout, Write};

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
pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        queue!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()?;
        queue!(stdout(), BlinkingBar)?;

        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        queue!(stdout(), LeaveAlternateScreen)?;

        Ok(())
    }

    pub fn hide_caret() -> Result<(), Error> { queue!(stdout(), Hide)?; Ok(()) }
    pub fn show_caret() -> Result<(), Error> { queue!(stdout(), Show)?; Ok(()) }

    pub fn move_caret(pos: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(pos.col as u16, pos.row as u16))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> { queue!(stdout(), Clear(ClearType::CurrentLine))?; Ok(()) }
    pub fn clear_screen() -> Result<(), Error> { queue!(stdout(), Clear(ClearType::All))?; Ok(()) }

    pub fn print(str: String) -> Result<(), Error> {
        queue!(stdout(), Print(str.to_string()))?;
        Ok(())
    }

    pub fn vec_print(v: &[ColoredString]) -> Result<(), Error> {
        for i in v {
            queue!(stdout(), Print(
                format!("{}", i)
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

    pub fn execute() -> Result<(), Error> { stdout().flush()?; Ok(()) }
}