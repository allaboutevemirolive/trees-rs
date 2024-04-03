use crate::canva::buffer::*;
use std::io::{self, StdoutLock};

// https://gist.github.com/JBlond/2fea43a3049b38287e5e9cefc87b2124
struct Color {
    black: &'static str,
    // hidden
    red: &'static str,
    // folder
    green: &'static str,
    yellow: &'static str,
    // File
    blue: &'static str,
    purple: &'static str,
    cyan: &'static str,
    white: &'static str,
    reset: &'static str,
}

impl Color {
    fn new() -> Color {
        Color {
            black: "\x1b[0;30m",
            red: "\x1b[0;31m",
            green: "\x1b[0;32m",
            yellow: "\x1b[0;33m",
            blue: "\x1b[0;34m",
            purple: "\x1b[0;35m",
            cyan: "\x1b[0;36m",
            white: "\x1b[0;37m",
            reset: "\x1b[0m",
        }
    }

    fn print_black(&self, buffer: &mut Buffer<StdoutLock<'_>>, text: &str) -> io::Result<()> {
        buffer.paint_text(self.black, text, self.reset)
    }

    fn print_red(&self, text: &str) {
        println!("{}{}{}", self.red, text, self.reset);
    }

    fn print_green(&self, text: &str) {
        println!("{}{}{}", self.green, text, self.reset);
    }

    fn print_blue(&self, text: &str) {
        println!("{}{}{}", self.blue, text, self.reset);
    }

    fn print_yellow(&self, text: &str) {
        println!("{}{}{}", self.yellow, text, self.reset);
    }

    fn print_purple(&self, text: &str) {
        println!("{}{}{}", self.purple, text, self.reset);
    }

    fn print_cyan(&self, text: &str) {
        println!("{}{}{}", self.cyan, text, self.reset);
    }

    fn print_white(&self, text: &str) {
        println!("{}{}{}", self.white, text, self.reset);
    }
}

/// Our Ansi that will be use entire programme. The idea is that if user
/// want to print output in a file, we can throw Ansi by providing empty
/// string.
pub struct AnsiTy {
    paint: String,
    reset: String,
}

impl AnsiTy {
    pub fn new(paint: String, reset: String) -> Self {
        Self { paint, reset }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_initialization() {
        let colors = Color::new();
        assert_eq!(colors.black, "\x1b[0;30m");
        assert_eq!(colors.red, "\x1b[0;31m");
        assert_eq!(colors.green, "\x1b[0;32m");
        assert_eq!(colors.yellow, "\x1b[0;33m");
        assert_eq!(colors.blue, "\x1b[0;34m");
        assert_eq!(colors.purple, "\x1b[0;35m");
        assert_eq!(colors.cyan, "\x1b[0;36m");
        assert_eq!(colors.white, "\x1b[0;37m");
        assert_eq!(colors.reset, "\x1b[0m");
    }
}
