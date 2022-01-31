use crate::indent::Indent;
use crate::token::{Token, Tokens};

use to_trait::To;


pub(crate) fn parseTokens(tokens: &Tokens, indent: Indent) -> String
{
    let mut output = String::new();
    let mut depth = 0;
    for token in tokens {
        match token {
            Token::CurlyBraceOpen => {
                output.push_str(&format!("\n{}{{\n", makeWhitespace(depth, indent)));
                depth += 1;
            },
            Token::CurlyBraceClose => {
                depth = depth.saturating_sub(1);
                output.push_str(&format!("\n{}}}", makeWhitespace(depth, indent)));
            },
            Token::Comma => {
                output.push_str(",\n");
            }
            Token::Text(text) => {
                output.push_str(&format!("{}{}", makeWhitespace(depth, indent), text))
            }
        }
    }
    output
}

fn makeWhitespace(depth: u32, indent: Indent) -> String
{
    let size = depth * indent;
    let mut output = String::with_capacity(size.try_to::<usize>().unwrap());
    for _ in 0..size {
        output.push(' ');
    }
    output
}
