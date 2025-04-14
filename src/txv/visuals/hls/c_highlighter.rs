use crate::txv::colors::Colors;
use colored::{ColoredString, Colorize};
use std::collections::HashMap;
use std::io::Error;

/// The CHighlighter module is used internally to individually color
/// input tokens before they are pushed into the function queue. 
#[derive(Default)]
pub struct CHighlighter {
    pub hash: HashMap<String, ColoredString>
}

impl CHighlighter {
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
            "|", ".", "=", ";", "!"
        ];
        let blue: Vec<&str> = vec![
            "alignas", "alignof", "and", "asm",
            "bitand", "bitor", "bool", "break",
            "case", "catch", "char", "class", "const",
            "continue", "do", "double", "else", "enum",
            "float", "goto", "if", "inline", "int", "long",
            "namespace", "new", "not", "or", "private",
            "protected", "public", "return", "short", "signed",
            "sizeof", "static", "static_cast", "struct",
            "switch", "this", "throw", "typedef", "union",
            "unsigned", "using", "void", "volatile", "while",
            "#include",
        ];
        let yellow: Vec<&str> = vec![
            "std"
        ];
        let red: Vec<&str> = vec![
            "0", "1", "2", "3", "4", 
            "5", "6", "7", "8", "9",
            "%d", "true", "false"
        ];

        for _i in 1..10 {}

        for i in gray { self.add(i, Colors::to_grey(i)); }
        for i in blue { self.add(i, Colors::to_blue(i)); }
        for i in yellow { self.add(i, Colors::to_yellow(i)); }
        for i in red { self.add(i, Colors::to_red(i)); }

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

        // Stores the running characters that are not individual tokens.
        let mut running: String = String::from("");

        // Holds indiviudal characters for coloration purposes.
        let mut indiv: String = String::from("");

        let mut comment: bool = false; // If the current token is going to be part of a comment.
        let mut string: bool = false; // If the current token is going to be part of a string.
        let mut include_statement: bool = false; // If the line contains an #include statement. Used for <> quotes.

        let mut prev_period: bool = false; // If the previous individual character token was a period.

        // Stores the two previous tokens to see if they compose an arrow.
        let mut indiv_two: char = ' ';
        let mut indiv_one: char = ' ';

        for _i in 0..l.len() {
            // Pushes the current character to the running string.
            let c: char = l_chars.next().unwrap();

            // Any characters following a slash must compose a comment.
            if c == '/' && !string { comment = true; }
            if comment {
                running.push(c);
                continue;
            }

            // Add the individual character to the indiv string.
            indiv.push(c);

            if c == '<' && include_statement {
                string = !string;
                if string == false {
                    token_vec.push(Colors::to_green(&indiv));
                    indiv = String::from("");
                    continue;
                }
            }

            if c == '"' { 
                string = !string;
                if string == false {
                    token_vec.push(Colors::to_green(&indiv));
                    indiv = String::from("");
                    continue;
                }
            }
            
            if string {
                token_vec.push(Colors::to_green(&indiv));
                indiv = String::from("");
                continue;
            }

            // If the individual character matches a stored token,
            // output its colored version and dump the running string.
            if h.contains_key(&indiv) {
                if h.contains_key(&running) {
                    if running == "#include" { include_statement = true; }

                    token_vec.push(
                        h.get(&running).unwrap().clone()
                    );
                    token_vec.push(h.get(&indiv).unwrap().clone());

                    running = String::from("");
                    indiv = String::from("");
                    continue;
                }

                // If this character is a space, then we color the running 
                // token and add it to the token list.
                if c == ' ' {
                    if h.contains_key(&running) {
                        token_vec.push(
                            h.get(&running).unwrap().clone()
                        );
        
                        token_vec.push(Colors::to_default(&indiv));
                        running = String::from("");
                        indiv = String::from("");
                        continue;
                    } else {
                        if running.chars().count() > 0 {
                            if running.chars().nth(0).unwrap().is_ascii_uppercase() {
                                token_vec.push(Colors::to_yellow(&running));
                            } else {
                                token_vec.push(Colors::to_default(&running));
                            }
                        }
                    }
                    
                    token_vec.push(Colors::to_default(&indiv));
                    running = String::from("");
                    indiv = String::from("");
                    continue;
                }

                // If the previous token was a period, highlight the next token as light blue (function).
                if prev_period {
                    // Add the new light blue token.
                    prev_period = false;
                    token_vec.push(Colors::to_light_blue(&running));
                    
                    // If this character is ALSO a period, then apply the same rule.
                    if c == '.' { prev_period = true }
                    token_vec.push(h.get(&indiv).unwrap().clone());

                    // Reset the string contents.
                    running = String::from("");
                    indiv = String::from("");
                    continue;
                } 

                // token_vec.push(h.get(&String::from(indiv_two)).unwrap().clone().blue());
                // token_vec.push(h.get(&String::from(indiv_one)).unwrap().clone().blue());

                // If the two previous individual characters compose an arrow, highlight the next
                // token as light blue (function).
                if indiv_two == '-' && indiv_one == '>' {
                    token_vec.push(Colors::to_light_blue(&running));
                    token_vec.push(h.get(&indiv).unwrap().clone());

                    // Reset the string contents.
                    running = String::from("");
                    indiv = String::from("");
                    continue;
                }

                indiv_two = indiv_one;
                indiv_one = c;

                // If this character is a token, set the prev_period flag.
                if c == '.' { prev_period = true }
                
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
        }
    
        // If the last token is a comment, add the comment to the token list.
        if comment {
            token_vec.push(Colors::to_comment(&running));
            return Ok(token_vec);
        }

        // Push the final uncolored token contents to the token vector.
        token_vec.push(Colors::to_default(&running));
        Ok(token_vec)
    }
}