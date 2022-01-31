#![allow(non_snake_case)]

mod indent;
mod lexer;
mod parser;
mod token;

use crate::indent::Indent;
use crate::lexer::lexText;
use crate::parser::parseTokens;

use clap::Parser;


fn main()
{
    let args = Args::parse();
    let tokens = lexText(&args.text);
    let formattedStruct = parseTokens(&tokens, args.indent);
    println!("{}", formattedStruct);
}

#[derive(Debug, Parser)]
struct Args
{
    text: String,
    #[clap(short, long, default_value_t = 4)]
    indent: Indent
}
