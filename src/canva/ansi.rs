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
