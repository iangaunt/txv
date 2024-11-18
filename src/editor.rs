use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;

mod terminal { include!("terminal/terminal.rs"); }
use terminal::{Terminal, Size, Position};

pub struct Editor {
    should_quit: bool
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false}
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();

        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn draw_rows() -> Result<(), Error>  {
        let Size{height, ..}: Size = Terminal::size()?;

        for i in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;

            if i + 1 < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh()?;
            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate(&event);
        }

        Ok(())
    }

    fn refresh(&mut self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor(Position{ x: 0, y: 0 })?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn evaluate(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            println!("{code}");
            match code {
                Char('b') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
}
