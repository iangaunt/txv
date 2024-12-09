use crate::editor::Location;

use core::cmp::max;
use std::io::Error;

#[derive(Default)]
pub struct Buffer {
    pub vector: Vec<String>,
    pub location: Location
}

/// A buffer for storing the contents of each line.
impl Buffer {
    /// Checks to see if the internal vector is empty.
    pub fn is_empty(&self) -> bool {
        self.vector.len() == 0
    }

    /// Adds a character at a specific point in the terminal depending on 
    /// where the current location of the buffer is.
    pub fn add_char(&mut self, key_code: char) -> Result<(), Error> {
        let b_vec = &mut self.vector;
        let b_line = b_vec.get_mut(self.location.y).unwrap();
        let mut b_chars = b_line.chars();
        
        // If the character is not at the beginning, remove one from the index.
        let mut add_index: usize = self.location.x;
        if self.location.x > 1 {
            add_index = self.location.x - 1;
        }

        // Parse the new line to add to the buffer.
        let mut new_line = String::from("");
        let mut added: bool = false;

        // Continue adding characters until the add index is hit.
        for i in 0..b_line.len() {
            if i == add_index {
                new_line.push(key_code);
                added = true;
            }
            new_line.push(b_chars.next().unwrap());
        }

        // If the add index has not been hit, add the new character.
        if !added { new_line.push(key_code); }

        // Edit the new line in the internal buffer.
        b_vec[self.location.y] = new_line;
        self.location.x = self.location.x.saturating_add(1);

        Ok(())
    }

    /// Deletes a char at the current location. Will not remove a character
    /// if the length of the current line is `0`.
    pub fn delete_char(&mut self) -> Result<(), Error> {
        let b_vec = &mut self.vector;
        let b_line = b_vec.get_mut(self.location.y).unwrap();

        // If the line is empty, there is nothing to remove.
        if b_line.len() == 0 {
            return Ok(())
        }

        let mut b_chars = b_line.chars();
        let mut new_line = String::from("");
        
        // If the iterator hits the delete location, ignore the next character.
        for i in 0..b_line.len() - 1 {
            if i == self.location.x - 2 {
                b_chars.next().unwrap();
            }
            new_line.push(b_chars.next().unwrap());
        }

        // Edit the new line in the internal buffer.
        b_vec[self.location.y] = new_line;
        self.location.x = max(2, self.location.x.saturating_sub(1));

        Ok(())
    }
}
