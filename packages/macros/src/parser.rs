use std::{collections::HashSet, fs, io};

use cssparser::{Parser, ParserInput, Token};

pub fn class_names(filename: &str) -> Result<impl Iterator<Item = String>, io::Error> {
    let css = fs::read_to_string(filename)?;

    let mut parser_input = ParserInput::new(&css);
    let mut input = Parser::new(&mut parser_input);
    let mut classes = HashSet::new();
    let mut prev_dot = false;

    while let Ok(token) = input.next_including_whitespace_and_comments() {
        if prev_dot {
            if let Token::Ident(class) = token {
                classes.insert(class.to_string());
            }
        }

        prev_dot = matches!(token, Token::Delim('.'));
    }

    Ok(classes.into_iter())
}
