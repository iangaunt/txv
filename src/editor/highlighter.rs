use colored::{ColoredString, Colorize};
use std::collections::HashMap;
use std::io::Error;

#[derive(Default)]
pub struct Highlighter {
    pub hash: HashMap<String, ColoredString>
}

impl Highlighter {
    pub fn init(&mut self) -> Result<(), Error> {
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

        self.hash.insert(String::from("use"), "use".truecolor(5, 175, 242));
        self.hash.insert(String::from("pub"), "pub".truecolor(5, 175, 242));
        self.hash.insert(String::from("struct"), "struct".truecolor(5, 175, 242));
        self.hash.insert(String::from("let"), "let".truecolor(5, 175, 242));
        self.hash.insert(String::from("mut"), "mut".truecolor(5, 175, 242));
        self.hash.insert(String::from("derive"), "derive".truecolor(5, 175, 242));
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

    pub fn tokenize(&self, l: &str) -> Result<Vec<ColoredString>, Error> {
        let h: &HashMap<String, ColoredString> = &self.hash;
        let mut token_vec: Vec<ColoredString> = Vec::new();

        let mut l_chars = l.chars();
        let mut running: String = String::from("");

        for _i in 0..l.len() {
            let c: char = l_chars.next().unwrap();
            let c_str = String::from(c);

            if c == ' ' {
                token_vec.push(running.white());
                token_vec.push(" ".white());

                running = String::from("");
                continue;
            }

            if h.contains_key(&c_str) {
                token_vec.push(running.white());
                token_vec.push(
                    h.get(&c_str).unwrap().clone()
                );

                running = String::from("");
                continue;
            }
            
            running.push(c);
            if h.contains_key(&running) {
                token_vec.push(
                    h.get(&running).unwrap().clone()
                );

                running = String::from("");
                continue;
            }
        }
        
        token_vec.push(running.white());

        Ok(token_vec)
    }
}

