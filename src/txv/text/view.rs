use crate::txv::buffer::Buffer;
use crate::txv::editor::Location;
use crate::txv::highlighter::Highlighter;
use crate::txv::terminal::{Position, Size, Terminal};

use core::cmp::{max, min};
use colored::ColoredString;
use crossterm::event::KeyCode;
use std::io::Error;

#[derive(Default)]
pub struct View {
    pub buffer: Buffer,
    pub highlighter: Highlighter,

    pub filename: String
}

impl View {
    pub fn init(&mut self) -> Result<(), Error> { 
        self.buffer.location = Location { x: 2, y: 0 };
        self.highlighter.init()?;

        Ok(())
    }

    fn center(msg: &str) -> Result<String, Error> {
        let mut run = msg.to_string();
        
        let width: usize = Terminal::size()?.width;
        let spaces = " ".repeat((width - msg.len()) / 2 - 1);

        run = format!("{spaces}{run}");
        run.truncate(width);
        Ok(run)
    }

    pub fn load(&mut self, filename: &str) -> Result<(), Error> {
        let contents = std::fs::read_to_string(filename)?;
        let buff_vec = &mut self.buffer.vector;
    
        for l in contents.lines() {
            buff_vec.push(String::from(l));
        }
        Ok(())
    }

    pub fn default(&mut self, msg: &String) -> Result<(), Error> {
        let Size{height, ..}: Size = Terminal::size()?;
        let buff_vec = &mut self.buffer.vector;

        let sx: usize = self.buffer.scroll_offset.col;
        let sy: usize = self.buffer.scroll_offset.row;

        for i in sy..height + sy {
            let mut st: String = String::from("");
            if i == height / 3 {
                st = Self::center(&String::from(msg))?;
            }

            if st.len() > (sx + 1) {
                let pushed = &st[sx..];
                buff_vec.push(String::from(pushed));
            } else {
                buff_vec.push(String::from(""));
            }
        }

        self.render()?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error>  {
        let Size{height, ..}: Size = Terminal::size()?;
        let buff_vec = &self.buffer.vector;

        let tilda_vec: Vec<ColoredString> = self.highlighter.tokenize("~").unwrap();

        let x: usize = self.buffer.location.x;
        let y: usize = self.buffer.location.y;

        let sx: usize = self.buffer.scroll_offset.col;
        let sy: usize = self.buffer.scroll_offset.row;

        for i in 0..(height - 1) {
            Terminal::move_caret(Position { col: 0, row: i })?;
            Terminal::clear_line()?;

            if buff_vec.len() > (i + sy) {
                let mut l: &str = &buff_vec[i + sy];
                if l.len() > sx {
                    l = &l[sx..];
                    let line_format = format!("~ {l}",);
                    Terminal::vec_print(
                        &self.highlighter.tokenize(&line_format).unwrap()
                    )?;
                } else {
                    Terminal::vec_print(&tilda_vec)?;
                }
            } else {
                Terminal::vec_print(&tilda_vec)?;
            }
        }

        self.stats_line(height)?;
        Terminal::move_caret(Position { col: x, row: y })?;

        Ok(())
    }

    pub fn refresh_line(&mut self) -> Result<(), Error> {
        let buff_vec = &self.buffer.vector;

        let y: usize = self.buffer.location.y;
        let sx: usize = self.buffer.scroll_offset.col;
        let sy: usize = self.buffer.scroll_offset.row;

        let mut l: &str = &buff_vec[y + sy];
        Terminal::move_caret(Position { col: 0, row: y })?;

        if l.len() > sx {
            l = &l[sx..];
            let line_format = format!("~ {l}",);
            Terminal::vec_print(
                &self.highlighter.tokenize(&line_format).unwrap()
            )?;
        }

        Ok(())
    }

    pub fn stats_line(&mut self, height: usize) -> Result<(), Error> {
        let pos_string: String = format!("{}, {}", 
            self.buffer.location.y + self.buffer.scroll_offset.row,
            self.buffer.location.x + self.buffer.scroll_offset.col - 2
        );

        let desc = String::from("~ txv :: ") + &self.filename + "    " + &pos_string;
        let mut spaces = String::from("");
        while (spaces.len() + desc.len()) < Terminal::size()?.width {
            spaces.push(' ');
        }

        Terminal::move_caret(Position { col: 0, row: height - 1})?;
        Terminal::clear_line()?;
        Terminal::print(spaces + &desc)?;

        Ok(())
    }
    
    pub fn move_to(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y} = self.buffer.location;
        let Position { mut col, mut row } = self.buffer.scroll_offset;

        let Size { width, height } = Terminal::size()?;

        match key_code {
            KeyCode::Up => { 
                y = y.saturating_sub(1); 

                if y == 0 {
                    row = max(0, row.saturating_sub(1));
                }

                if self.buffer.vector[y].len() < x && x != 2 {
                    x = self.buffer.vector[y].len() + 2;
                }
            }
            
            KeyCode::Down => { 
                y = min(
                height.saturating_sub(2), 
                y.saturating_add(1)
                ); 

                if y == height - 2 {
                    row = min(
                        self.buffer.vector.len() - height, 
                        row.saturating_add(1)
                    );
                }

                if y != height - 2 
                    && self.buffer.vector[y.saturating_add(row)].len() < x 
                    && x != 2 {
                    
                    x = self.buffer.vector[y.saturating_add(row)].len() + 2;
                }
            }
            
            KeyCode::Left => { 
                // If the cursor is in the top left corner, then skip left logic.
                if x == 2 && y == 0 { return Ok(()); }

                // If the cursor is on the left of the terminal, then 
                if x == 2 {
                    col = max(0, col.saturating_sub(1));

                    if y == 0 { row = row.saturating_sub(1); }
                    
                    if col == 0 {
                        y = max(0, y.saturating_sub(1));
                        x = self.buffer.vector[y.saturating_add(row)].len() + 3;
                        if self.buffer.vector[y.saturating_add(row)].is_empty() { x = 2; }
                    }
                }

                x = max(2, x.saturating_sub(1));
            }

            KeyCode::Right => { 
                if y == height - 1 { return Ok(()); }

                x = min(
                    width.saturating_sub(1), 
                    x.saturating_add(1)
                ); 

                if x == self.buffer.vector[y.saturating_add(row)].len() + 3 
                    || self.buffer.vector[y.saturating_add(row)].is_empty() {

                    x = 2;
                    y = min(height - 1, y.saturating_add(1));

                    if y == height - 1 {
                        row = row.saturating_add(1);
                    }
                }

                if x == width - 1 {
                    col = col.saturating_add(1);
                }
            },

            _ => (),
        }

        self.buffer.location = Location { x, y };
        self.buffer.scroll_offset = Position { col, row };

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}