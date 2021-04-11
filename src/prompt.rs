use std::error::Error;
use std::io::{self, Write};
use termion::color;

/// Prompts user to enter a command and number
// TODO: return result, rerun in main if not Ok
pub fn prompt() -> Result<(char, usize), Box<dyn Error>> {
    let mut input = String::new();

    print!("{}> {}", color::Fg(color::Yellow), color::Fg(color::Reset));

    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    Ok(parse_cmd(&input).unwrap())
}

/// Interpret user input
fn parse_cmd(input: &str) -> Result<(char, usize), Box<dyn Error>> {
    // Clean up input
    let lowercased = &input.to_lowercase();
    let cleaned_input = &lowercased.trim();
    let chars: Vec<char> = cleaned_input.chars().collect();
    let index: usize;

    // Get command, default to play
    let mut cmd = 'p';
    if !chars[0].is_numeric() {
        cmd = chars[0];

        index = cleaned_input[1..].parse::<usize>().unwrap();
    } else {
        index = cleaned_input.parse::<usize>().unwrap();
    }

    // Get index
    Ok((cmd, index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cmd_nocmd() {
        assert_eq!(parse_cmd("12").unwrap(), ('p', 12 as usize));
    }

    #[test]
    fn test_parse_cmd_play() {
        assert_eq!(parse_cmd("p4").unwrap(), ('p', 4 as usize));
    }

    #[test]
    fn test_parse_cmd_dl() {
        assert_eq!(parse_cmd("d10").unwrap(), ('d', 10 as usize));
    }

    #[test]
    fn test_parse_cmd_sub() {
        assert_eq!(parse_cmd("s1").unwrap(), ('s', 1 as usize));
    }
}
