use crossterm::event::{
    read, Event::{self, Key}, KeyCode, KeyEvent, KeyEventKind, KeyModifiers
};
use std::io::Error;
use std::path::Path;

use crate::terminal::{Position, Terminal};
use crate::view::View;

#[derive(Copy, Clone, Debug, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,

    pub filename: String,
    pub viewer: View
}

impl Editor {
    pub fn run(&mut self) {
        let viewer: &mut View = &mut self.viewer;
        
        Terminal::initialize().unwrap();
        viewer.init().unwrap();
        if viewer.is_buffer_empty() {
            viewer.default(
                &String::from("txt-editor :: v1.0.0")
            ).unwrap();
        }
        let result: Result<(), Error> = self.repl();

        Terminal::terminate().unwrap();
        result.unwrap();
    }

    pub fn load(&mut self, filename: &str) -> Result<(), Error> {
        self.viewer.load(filename)?;
        self.filename = String::from(filename);

        Ok(())
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh()?;
            if self.should_quit {
                break;
            }

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

                KeyCode::Backspace => {
                    self.viewer.buffer.delete_char()?;
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
