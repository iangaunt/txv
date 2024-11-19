use crate::terminal::{Position, Size, Terminal};
use std::io::Error;

#[derive(Default)]
pub struct Buffer {
    pub vector: Vec<String>
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.vector.len() == 0
    }
}

#[derive(Default)]
pub struct View {
    pub buffer: Buffer
}

impl View {
    pub fn init(&mut self) -> Result<(), Error> { Ok(()) }

    fn center(msg: &str) -> Result<String, Error> {
        let mut run = format!("{}", msg);
        
        let width: usize = Terminal::size()?.width as usize;
        let spaces = " ".repeat((width - msg.len()) / 2 - 1);

        run = format!("~{spaces}{run}");
        run.truncate(width);
        Ok(run)
    }

    pub fn load_default(&self, msg: &String) -> Result<(), Error> {
        let Size{height, ..}: Size = Terminal::size()?;

        for i in 0..height {
            Terminal::clear_line()?;
            Terminal::move_caret(Position { col: 0, row: i })?;

            if i == height / 3 {
                let gb = Self::center(&String::from(msg))?;
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

    pub fn load(&mut self, filename: &str) -> Result<(), Error> {
        let contents = std::fs::read_to_string(filename)?;
        let buff_vec = &mut self.buffer.vector;
    
        for l in contents.lines() {
            buff_vec.push(String::from(l));
        }
        Ok(())
    }

    pub fn is_buffer_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}