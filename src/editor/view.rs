mod terminal { include!("terminal.rs"); }
use terminal::{Position, Size, Terminal};

use std::io::Error;

#[derive(Default)]
pub struct Buffer {
    pub vector: Vec<String>
}

impl Buffer {}

#[derive(Default)]
pub struct View {
    buffer: Buffer
}

impl View {
    pub fn init(&mut self) -> Result<(), Error> {
        self.buffer.vector.push(String::from("Hello, world!"));
        self.buffer.vector.push(String::from("txt-editor :: v1.0.0"));
        Ok(())
    }

    fn center(msg: &str) -> Result<String, Error> {
        let mut run = format!("{}", msg);
        
        let width: usize = Terminal::size()?.width as usize;
        let spaces = " ".repeat((width - msg.len()) / 2 - 1);

        run = format!("~{spaces}{run}");
        run.truncate(width);
        Ok(run)
    }

    pub fn goodbye(&self) -> Result<(), Error> {
        let Size{height, ..}: Size = Terminal::size()?;

        for i in 0..height {
            Terminal::clear_line()?;
            Terminal::move_caret(Position { col: 0, row: i })?;

            if i == height / 3 {
                let gb = Self::center(&String::from("Goodbye."))?;
                Terminal::print(&gb)?;
            } else {
                Terminal::print("~")?;
            }

            if i + 1 < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error>  {
        let Size{height, ..}: Size = Terminal::size()?;
        let buff_vec = &self.buffer.vector;

        for i in 0..height {
            Terminal::clear_line()?;
            Terminal::move_caret(Position { col: 0, row: i })?;

            if buff_vec.len() > i {
                let line = &buff_vec[i];
                let line_format = format!("~ {line}",);
                Terminal::print(&line_format)?;
            } else {
                Terminal::print("~")?;
            }

            if i + 1 < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }
}