#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::io;

fn read_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    return Ok(input);
}

#[derive(Debug, Clone)]
enum Token {
    ADD,
    NUMBER,
    WS,
}

fn get_match(str: &String) -> Option<Token> {
    lazy_static! {
        static ref ADD: Regex = Regex::new(r"^\+$").unwrap();
        static ref NUMBER: Regex = Regex::new("^[0-9]+$").unwrap();
        static ref WS: Regex = Regex::new("^[\t\n ]+$").unwrap();
    }
    if ADD.is_match(str) {
        Some(Token::ADD)
    } else if NUMBER.is_match(str) {
        Some(Token::NUMBER)
    } else if WS.is_match(str) {
        Some(Token::WS)
    } else {
        None
    }
}

fn lex(str: &String) -> () {
    let mut tokens: Vec<(Token, String)> = Vec::new();
    let mut partial: String = "".to_string();
    let mut last_token: Option<Token> = None;
    for ch in str.chars() {
        let candidate = partial.clone() + &ch.to_string();
        let candidate_match = get_match(&candidate);
        // println!("{:?} {:?}", candidate, candidate_match);
        match candidate_match {
            None => {
                tokens.push((
                    last_token
                        .clone()
                        .expect(&("unable to lex: ".to_string() + &candidate)),
                    partial,
                ));
                partial = ch.to_string();
                last_token = get_match(&partial);
            }
            Some(tok) => {
                partial.push(ch);
                last_token = Some(tok);
            }
        }
    }
    tokens.push((last_token.unwrap(), partial));
    println!("{}", tokens.len());
    println!("{:?}", tokens);
}

fn main() {
    let input = read_input().unwrap();
    lex(&input.to_string());
}
