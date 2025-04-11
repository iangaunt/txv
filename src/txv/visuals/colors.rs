use colored::{ColoredString, Colorize};

#[derive(Default)]
pub struct Colors {}

/// Converts an input string to their respective color.
impl Colors {
    pub fn to_blue(string: &str) -> ColoredString { string.truecolor(84, 158, 255) }
    pub fn to_light_blue(string: &str) -> ColoredString { string.truecolor(97, 175, 239) }
    pub fn to_grey(string: &str) -> ColoredString { string.truecolor(107, 133, 152) }
    pub fn to_yellow(string: &str) -> ColoredString { string.truecolor(229, 187, 129) }
    pub fn to_red(string: &str) -> ColoredString { string.truecolor(229, 129, 146) }
    pub fn to_green(string: &str) -> ColoredString { string.truecolor(142, 216, 160) }

    pub fn to_comment(string: &str) -> ColoredString { string.truecolor(127, 132, 142).italic() }
    pub fn to_default(string: &str) -> ColoredString { string.truecolor(155, 172, 185) }
}