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
    /// Initializes the internal `hash` value with a combination of
    /// strings to color and the colors they will be output in.
    /// 
    /// This will be modified into another separate class later as it is
    /// currently implemented like shit, and looks like trash.
    pub fn init(&mut self) -> Result<(), Error> {
        self.hash.insert(String::from(" "), " ".white());

        self.hash.insert(String::from(":"), ":".truecolor(34, 37, 43));
        self.hash.insert(String::from(","), ",".truecolor(34, 37, 43));
        self.hash.insert(String::from("{"), "{".truecolor(34, 37, 43));
        self.hash.insert(String::from("}"), "}".truecolor(34, 37, 43));
        self.hash.insert(String::from("~"), "~".truecolor(34, 37, 43));
        self.hash.insert(String::from(";"), ";".truecolor(34, 37, 43));
        self.hash.insert(String::from("&"), "&".truecolor(34, 37, 43));
        self.hash.insert(String::from("["), "[".truecolor(34, 37, 43));
        self.hash.insert(String::from("]"), "]".truecolor(34, 37, 43));
        self.hash.insert(String::from("("), "(".truecolor(34, 37, 43));
        self.hash.insert(String::from(")"), ")".truecolor(34, 37, 43));
        self.hash.insert(String::from("?"), "?".truecolor(34, 37, 43));
        self.hash.insert(String::from("<"), "<".truecolor(34, 37, 43));
        self.hash.insert(String::from(">"), ">".truecolor(34, 37, 43));
        self.hash.insert(String::from("-"), "-".truecolor(34, 37, 43));
        self.hash.insert(String::from("+"), "+".truecolor(34, 37, 43));
        self.hash.insert(String::from("="), "=".truecolor(34, 37, 43));
        self.hash.insert(String::from("|"), "|".truecolor(34, 37, 43));
        self.hash.insert(String::from("."), ".".truecolor(34, 37, 43));
        self.hash.insert(String::from("#"), "#".truecolor(34, 37, 43));

        self.hash.insert(String::from("use"), "use".truecolor(5, 175, 242).bold());
        self.hash.insert(String::from("pub"), "pub".truecolor(5, 175, 242).bold());
        self.hash.insert(String::from("struct"), "struct".truecolor(5, 175, 242).bold());
        self.hash.insert(String::from("let"), "let".truecolor(5, 175, 242).bold());
        self.hash.insert(String::from("mut"), "mut".truecolor(5, 175, 242).bold());
        self.hash.insert(String::from("derive"), "derive".truecolor(5, 175, 242).bold());
        self.hash.insert(String::from("fn"), "fn".truecolor(5, 175, 242));
        self.hash.insert(String::from("impl"), "impl".truecolor(5, 175, 242));
        self.hash.insert(String::from("self"), "self".truecolor(5, 175, 242));

        self.hash.insert(String::from("bool"), "bool".truecolor(232, 213, 9));
        self.hash.insert(String::from("if"), "if".truecolor(232, 213, 9));
        self.hash.insert(String::from("loop"), "loop".truecolor(232, 213, 9));
        self.hash.insert(String::from("Result"), "Result".truecolor(232, 213, 9));
        self.hash.insert(String::from("Error"), "Error".truecolor(232, 213, 9));
        self.hash.insert(String::from("std"), "std".truecolor(232, 213, 9));
        self.hash.insert(String::from("Ok"), "Ok".truecolor(232, 213, 9));
        self.hash.insert(String::from("unwrap"), "unwrap".truecolor(232, 213, 9));
        self.hash.insert(String::from("break"), "break".truecolor(232, 213, 9));

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

        for _i in 0..l.len() {
            // Pushes the current character to the running string.
            let c: char = l_chars.next().unwrap();            
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

