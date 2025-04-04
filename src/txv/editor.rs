use crossterm::event::{
    read, Event::{self, Key}, KeyCode, KeyEvent, KeyEventKind, KeyModifiers
};
use std::io::Error;
use std::path::Path;

use crate::txv::terminal::{Position, Terminal};
use crate::txv::view::View;

/// An `(x, y)` coordinate for the caret in the viewport.
/// Used for adjusting the positioning of any new characters
/// or caret movements across the board.
#[derive(Copy, Clone, Debug, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize
}

/// Each editor contains the quit flag, the current stored
/// filename, and the viewport used to modify its contents.
#[derive(Default)]
pub struct Editor {
    should_quit: bool,

    pub filename: String,
    pub viewer: View
}

impl Editor {
    pub fn set_extension(&mut self, extension: String) {
        self.viewer.highlighter.extension = extension;
    }

    /// Starts the main editor loop. 
    pub fn run(&mut self) {
        // Initializes the viewport of the terminal editor.
        let viewer: &mut View = &mut self.viewer;
        
        // Initializes the terminal.
        Terminal::initialize().unwrap();
        viewer.init().unwrap();

        // If the viewer buffer is empty (no file is closed), show the default screen.
        if viewer.is_buffer_empty() {
            viewer.default(
                &String::from("txv :: v1.0.0")
            ).unwrap();
        }

        // Enter the editor loop.
        let result: Result<(), Error> = self.repl();

        // Close the editor once the loop is terminated.
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    /// Loads the contents of the file specified at `filename` into
    /// the viewer buffer for colorization / display.
    pub fn load(&mut self, filename: &str) -> Result<(), Error> {
        self.viewer.load(filename)?;
        self.filename = String::from(filename);

        Ok(())
    }

    /// The main editor loop for the terminal. Handles any input
    /// from the keyboard and closes once the end keycode is entered.
    fn repl(&mut self) -> Result<(), Error> {
        loop {
            // Refreshes the contents of the viewport.
            self.refresh()?;
            if self.should_quit {
                break; // End the loop if forcekilled.
            }

            // Read in any keyboard events and change buffer accordingly.
            let event = read()?;
            self.evaluate(&event)?;
        }

        Ok(())
    }


    fn refresh(&mut self) -> Result<(), Error> {
        let viewer: &mut View = &mut self.viewer;
        Terminal::hide_caret()?;
        
        if self.should_quit {
            Terminal::clear_screen()?;
            viewer.default(&String::from("Goodbye."))?;
        } else {
            viewer.render()?;
            Terminal::move_caret(Position{ 
                col: viewer.buffer.location.x, 
                row: viewer.buffer.location.y 
            })?;
        }

        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    fn evaluate(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code, 
            modifiers, 
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('b') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }

                KeyCode::Char('s') if *modifiers == KeyModifiers::CONTROL => {
                    self.save();
                }

                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.viewer.move_to(*code)?;
                }

                KeyCode::Char(c) => {
                    self.viewer.buffer.add_char(*c)?;
                }

                KeyCode::Enter => { self.viewer.buffer.add_line(); }
                KeyCode::Backspace => {
                    let buf = &self.viewer.buffer;
                    if buf.location.x == 2 && buf.location.y != 0 {
                        self.viewer.buffer.delete_line();
                        return Ok(());
                    }

                    self.viewer.buffer.delete_char()?;
                }

                KeyCode::Tab => {
                    for _i in 0..4 {
                        self.viewer.buffer.add_char(' ')?;
                    }
                }

                _ => (),
            }
        }

        Ok(())
    }

    pub fn save(&mut self) {
        let path = Path::new(&self.filename);
        let vec: &Vec<String> = &self.viewer.buffer.vector;

        std::fs::write(path, vec.join("\n")).expect("");
    }
}
