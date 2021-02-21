use std::io::{self, Write};
use termion::color;

/// Prompts user to enter a number
pub fn prompt_number() -> usize {
    let mut number = String::new();

    print!(
        "{}\nItem: {}",
        color::Fg(color::Yellow),
        color::Fg(color::Reset)
    );
    io::stdout().flush().expect("Could not flush output");
    io::stdin()
        .read_line(&mut number)
        .expect("Could not read input");

    number
        .trim()
        .parse::<usize>()
        .expect("Could not parse input") - 1
}
