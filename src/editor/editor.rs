use core::cmp::min;
use crossterm::event::{
    read, Event::{self, Key}, KeyCode, KeyEvent, KeyEventKind, KeyModifiers
};

use std::io::Error;

mod terminal { include!("terminal.rs"); }
use terminal::{Position, Size, Terminal};

mod view { include!("view.rs"); }
use view::View;

#[derive(Copy, Clone, Debug, Default)]
struct Location {
    x: usize,
    y: usize
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    viewer: View
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.viewer.init().unwrap();
        let result = self.repl();

        Terminal::terminate().unwrap();
        result.unwrap();
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
        Terminal::hide_caret()?;
        
        if self.should_quit {
            Terminal::clear_screen()?;
            self.viewer.goodbye()?;
        } else {
            self.viewer.render()?;
            Terminal::move_caret(Position{ 
                col: self.location.x, 
                row: self.location.y 
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

                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_to(*code)?;
                }
                _ => (),
            }
        }

        Ok(())
    }

    fn move_to(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y} = self.location;
        let Size { width, height } = Terminal::size()?;

        match key_code {
            KeyCode::Up => { y = y.saturating_sub(1); }
            KeyCode::Down => { y = min(
                height.saturating_sub(1), 
                y.saturating_add(1)); 
            }
            
            KeyCode::Left => { x = x.saturating_sub(1); }
            KeyCode::Right => { x = min(
                width.saturating_sub(1), 
                x.saturating_add(1)); 
            }

            KeyCode::PageUp => { y = 0; }
            KeyCode::PageDown => { y = height.saturating_sub(1); }

            KeyCode::Home => { x = 0; }
            KeyCode::End => { y = width.saturating_sub(1); }

            _ => (),
        }

        self.location = Location { x, y };
        Ok(())
    }
}
