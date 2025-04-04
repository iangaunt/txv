use crate::txv::colors::Colors;
use colored::ColoredString;
use std::io::Error;

/// The TextHighlighter module is used internally to individually color
/// input tokens before they are pushed into the function queue. 
#[derive(Default)]
pub struct TextHighlighter {}

impl TextHighlighter {
    /// Tokenizes a string and creates a series of colorized strings which will be
    /// displayed in the terminal when the buffer is rendered. The `Terminal` object
    /// can display the contents of the returned vector with `Terminal::print_vec`.
    pub fn tokenize(&self, l: &str) -> Result<Vec<ColoredString>, Error> {
        // A running list of the colored tokens to be displayed.
        let mut token_vec: Vec<ColoredString> = Vec::new();
        token_vec.push(Colors::to_default(&l));
        Ok(token_vec)
    }
}