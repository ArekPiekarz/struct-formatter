use crate::token::{Token, Tokens};

use std::iter::Peekable;
use std::str::Chars;


pub(crate) fn lexText(text: &str) -> Vec<Token>
{
    let mut tokens = vec![];
    let mut iter = text.chars().peekable();
    while let Some(character) = iter.next() {
        match character {
            '{' => lexBraceStart(&mut tokens),
            '}' => lexBraceEnd(&mut tokens),
            ',' => lexComma(&mut tokens),
             _  => lexOtherText(character, &mut iter, &mut tokens)
        }
    }
    tokens
}

fn lexBraceStart(tokens: &mut Tokens)
{
    tokens.push(Token::CurlyBraceOpen);
}

fn lexBraceEnd(tokens: &mut Tokens)
{
    tokens.push(Token::CurlyBraceClose);
}

fn lexComma(tokens: &mut Tokens)
{
    tokens.push(Token::Comma);
}

fn lexOtherText(character: char, iter: &mut Peekable<Chars>, tokens: &mut Tokens)
{
    let mut text = String::from(character);
    while let Some(character) = iter.peek() {
        match character {
            '{' | '}' | ',' => break,
            character => {
                text.push(*character);
                iter.next();
            }
        }
    }
    tokens.push(Token::Text(text));
}
