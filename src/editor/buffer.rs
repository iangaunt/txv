use crate::editor::Location;

use core::cmp::max;
use std::io::Error;

#[derive(Default)]
pub struct Buffer {
    pub vector: Vec<String>,
    pub location: Location
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.vector.len() == 0
    }

    pub fn add_char(&mut self, key_code: char) -> Result<(), Error> {
        let b_vec = &mut self.vector;
        let b_line = b_vec.get_mut(self.location.y).unwrap();
        let mut b_chars = b_line.chars();
        
        let mut add_index: usize = self.location.x;
        if self.location.x > 1 {
            add_index = self.location.x - 1;
        }

        let mut new_line = String::from("");
        let mut added: bool = false;

        for i in 0..b_line.len() {
            if i == add_index {
                new_line.push(key_code);
                added = true;
            }
            new_line.push(b_chars.next().unwrap());
        }
        if !added { new_line.push(key_code); }

        b_vec[self.location.y] = new_line;
        self.location.x = self.location.x.saturating_add(1);

        Ok(())
    }

    pub fn delete_char(&mut self) -> Result<(), Error> {
        let b_vec = &mut self.vector;
        let b_line = b_vec.get_mut(self.location.y).unwrap();

        if b_line.len() == 0 {
            return Ok(())
        }

        let mut b_chars = b_line.chars();
        let mut new_line = String::from("");
        
        for i in 0..b_line.len() - 1 {
            if i == self.location.x - 2 {
                b_chars.next().unwrap();
            }
            new_line.push(b_chars.next().unwrap());
        }
        b_vec[self.location.y] = new_line;
        self.location.x = max(2, self.location.x.saturating_sub(1));

        Ok(())
    }
}
