use crate::txv::hls::{
    cpp_highlighter::CppHighlighter,
    dart_highlighter::DartHighlighter,
    rs_highlighter::RustHighlighter, 
    txt_highlighter::TextHighlighter
};
use colored::ColoredString;
use std::io::Error;

/// The Highlighter module is used internally to individually color
/// input tokens before they are pushed into the function queue. 
#[derive(Default)]
pub struct Highlighter {
    pub extension: String,

    pub cpp: CppHighlighter,
    pub dart: DartHighlighter,
    pub rs: RustHighlighter,
    pub txt: TextHighlighter
}

impl Highlighter {
    /// Initializes the internal `hash` value with a combination of
    /// strings to color and the colors they will be output in.
    /// 
    /// This will be modified into another separate class later as it is
    /// currently implemented like shit, and looks like trash.
    pub fn init(&mut self) -> Result<(), Error> {
        self.cpp.init()?;
        self.dart.init()?;
        self.rs.init()?;
        Ok(())
    }

    /// Tokenizes a string and creates a series of colorized strings which will be
    /// displayed in the terminal when the buffer is rendered. The `Terminal` object
    /// can display the contents of the returned vector with `Terminal::print_vec`.
    pub fn tokenize(&self, l: &str) -> Result<Vec<ColoredString>, Error> {
        // Fetches the internal hash map.
        let mut token_vec: Vec<ColoredString> = Vec::new(); 
        
        if self.extension == "cpp" { token_vec = self.cpp.tokenize(l)?; }
        if self.extension == "dart" { token_vec = self.dart.tokenize(l)?; }
        if self.extension == "rs" { token_vec = self.rs.tokenize(l)?; }
        if self.extension == "txt" { token_vec = self.txt.tokenize(l)?; }

        if token_vec.len() == 0 { token_vec = self.txt.tokenize(l)?; }

        Ok(token_vec)
    }
}