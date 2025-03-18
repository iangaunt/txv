use crate::colors::Colors;
use colored::{ColoredString, Colorize};
use std::collections::HashMap;
use std::io::Error;

/// The Highlighter module is used internally to individually color
/// input tokens before they are pushed into the function queue. 
#[derive(Default)]
pub struct Highlighter {
    pub hash: HashMap<String, ColoredString>
}

impl Highlighter {
    /// Adds a string and its colored version to the hash map.
    pub fn add(&mut self, string: &str, colstr: ColoredString) {
        self.hash.insert(String::from(string), colstr);
    }

    /// Initializes the internal `hash` value with a combination of
    /// strings to color and the colors they will be output in.
    /// 
    /// This will be modified into another separate class later as it is
    /// currently implemented like shit, and looks like trash.
    pub fn init(&mut self) -> Result<(), Error> {
        self.hash.insert(String::from(" "), " ".white());

        let gray: Vec<&str> = vec![
            ":", ",", "{", "}", "~",
            "&", "[", "]", "(", ")", 
            "?", "<", ">", "-", "+", 
            "|", ".", "#", "=", ";"
        ];
        let blue: Vec<&str> = vec![
            "use", "pub", "struct", "let", "mut", 
            "derive", "fn", "impl", "self", "if",
            "in", "from", "crate", "else"
        ];
        let yellow: Vec<&str> = vec![
            "bool", "loop", "Result", "Error", "std", 
            "Ok", "unwrap", "break", "KeyEvent", "KeyEventKind",
            "io", "path", "str"
        ];

        for _i in 1..10 {}

        for i in gray {  self.add(i, Colors::to_grey(i)); }
        for i in blue {  self.add(i, Colors::to_blue(i)); }
        for i in yellow {  self.add(i, Colors::to_yellow(i)); }

        Ok(())
    }

    /// Tokenizes a string and creates a series of colorized strings which will be
    /// displayed in the terminal when the buffer is rendered. The `Terminal` object
    /// can display the contents of the returned vector with `Terminal::print_vec`.
    pub fn tokenize(&self, l: &str) -> Result<Vec<ColoredString>, Error> {
        // Fetches the internal hash map.
        let h: &HashMap<String, ColoredString> = &self.hash;

        // A running list of the colored tokens to be displayed.
        let mut token_vec: Vec<ColoredString> = Vec::new();

        // An iterator for the `l` string and a running string for storing characters.
        let mut l_chars = l.chars();

        let mut running: String = String::from("");
        let mut indiv: String = String::from("");

        let mut comment: bool = false;

        for _i in 0..l.len() {
            // Pushes the current character to the running string.
            let c: char = l_chars.next().unwrap();

            if c == '/' { comment = true; }
            if comment {
                if c == '\n' { 
                    comment = false;
                    token_vec.push(Colors::to_blue(&running));
                    running = String::from("");
                    continue;
                } else {
                    running.push(c);
                    continue;
                }
            }

            indiv.push(c);

            // If the individual character matches a stored token,
            // output its colored version and dump the running string.
            if h.contains_key(&indiv) {
                if running.chars().count() > 0 {
                    if running.chars().nth(0).unwrap().is_ascii_uppercase() {
                        token_vec.push(Colors::to_yellow(&running));
                    } else {
                        token_vec.push(Colors::to_default(&running));
                    }
                }
                token_vec.push(
                    h.get(&indiv).unwrap().clone()
                );

                running = String::from("");
                indiv = String::from("");
                continue;
            }

            indiv = String::from("");
            running.push(c);

            // If the running string matches a stored token,
            // output its colored variant to the terminal.
            if h.contains_key(&running) {
                token_vec.push(
                    h.get(&running).unwrap().clone()
                );

                running = String::from("");
                continue;
            }
        }
        
        // Push the final uncolored token contents to the token vector.
        token_vec.push(running.white());

        // The string has now been colored and tokenized for output.
        Ok(token_vec)
    }
}

