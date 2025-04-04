use crate::txv::editor::Location;
use crate::txv::terminal::Position;

use core::cmp::max;
use std::io::Error;

#[derive(Default)]
pub struct Buffer {
    pub vector: Vec<String>,
    pub location: Location,
    pub scroll_offset: Position,
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
        let b_line = b_vec.get_mut(self.location.y + self.scroll_offset.row).unwrap();
        let mut b_chars = b_line.chars();
        
        // If the character is not at the beginning, remove one from the index.
        let mut add_index: usize = self.location.x;
        if self.location.x > 2 {
            add_index = self.location.x - 2;
        }
        if self.location.x == 2 { add_index = 0; }

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
        b_vec[self.location.y + self.scroll_offset.row] = new_line;
        self.location.x = self.location.x.saturating_add(1);

        Ok(())
    }

    /// Adds a new line following the specific position of the caret. If there
    /// is text following the caret, it will be moved to a new line.
    pub fn add_line(&mut self) {
        let x: usize = self.location.x + self.scroll_offset.col;
        let y: usize = self.location.y + self.scroll_offset.row;
        
        let current_line: String = self.vector[y].clone();
        let mut b_vec: Vec<String> = Vec::new();

        for i in 0..self.vector.len() {
            if i == y {
                // If the new line is added on the current line, split the
                // contents of the line based on the caret position.
                let split_line_front: String = String::from(&current_line[0..(x - 2)]);
                let split_line_back: String = String::from(&current_line[(x - 2)..]);

                // Push the new cut lines back to back.
                b_vec.push(split_line_front);
                b_vec.push(split_line_back);
            } else {
                b_vec.push(self.vector[i].clone());
            }
        }

        // Move the caret to the start of the new line.
        self.vector = b_vec;
        self.location.y += 1;
        self.location.x = 2;
    }

    /// Deletes a char at the current location. Will not remove a character
    /// if the length of the current line is `0`.
    pub fn delete_char(&mut self) -> Result<(), Error> {
        let b_vec = &mut self.vector;
        let b_line = b_vec.get_mut(self.location.y + self.scroll_offset.row).unwrap();

        // If the cursor is at the first position, then skip deleting.
        if self.location.x == 2 { return Ok(()) }
        if b_line.len() == 0 { return Ok(()) }

        let mut b_chars = b_line.chars();
        let mut new_line = String::from("");
        
        // If the iterator hits the delete location, ignore the next character.
        for i in 0..b_line.len() - 1 {
            if i == self.location.x - 3 {
                b_chars.next().unwrap();
            }
            new_line.push(b_chars.next().unwrap());
        }

        // Edit the new line in the internal buffer.
        b_vec[self.location.y + self.scroll_offset.row] = new_line;
        self.location.x = max(2, self.location.x.saturating_sub(1));

        Ok(())
    }

    /// Deletes a line if the contents of the line are empty, moving all
    /// of the following lines in the buffer up one index and truncating the vector.
    pub fn delete_line(&mut self) {
        // The vertical location of the line to delete.
        let y: usize = self.location.y + self.scroll_offset.row;

        // The length of the line above, plus the added ~ character.
        let ind: usize = self.vector[y - 1].len() + 2;

        let mut b_vec: Vec<String> = Vec::new();
        for i in 0..self.vector.len() {
            // Skip the deleted line.
            if i == y { continue; }

            // Move the contents of the deleted line to the line above.
            let mut cl = self.vector[i].clone();
            if i == y - 1 {
                cl.push_str(&self.vector[i + 1]);
            }

            b_vec.push(cl);
        }

        // Move the position of the caret up one line and to the end of the 
        // length of the previous line BEFORE appending new contents.
        self.vector = b_vec;
        self.location.x = ind;
        self.location.y -= 1;
    }
}