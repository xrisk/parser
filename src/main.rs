#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::io;
use std::io::Write;

fn read_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    return Ok(input);
}

#[derive(Debug, Clone)]
enum Token {
    ADD(String),
    NUMBER(String),
    WS(String),
}

fn get_match(str: &String) -> Option<Token> {
    lazy_static! {
        static ref ADD: Regex = Regex::new(r"^\+$").unwrap();
        static ref NUMBER: Regex = Regex::new("^[0-9]+$").unwrap();
        static ref WS: Regex = Regex::new("^[\t\n ]+$").unwrap();
    }
    if ADD.is_match(str) {
        Some(Token::ADD((*str).clone()))
    } else if NUMBER.is_match(str) {
        Some(Token::NUMBER((*str).clone()))
    } else if WS.is_match(str) {
        Some(Token::WS((*str).clone()))
    } else {
        None
    }
}

fn lex(str: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut partial: String = "".to_string();
    let mut last_token: Option<Token> = None;
    for ch in str.chars() {
        let candidate = partial.clone() + &ch.to_string();
        let candidate_match = get_match(&candidate);
        // println!("{:?} {:?}", candidate, candidate_match);
        match candidate_match {
            None => {
                tokens.push(last_token.clone().unwrap());
                partial = ch.to_string();
                last_token = get_match(&partial);
            }
            Some(tok) => {
                partial.push(ch);
                last_token = Some(tok);
            }
        }
    }
    tokens.push(last_token.unwrap());
    tokens
}

// E := T + E | eps
// T := number
//
//
#[derive(Debug)]
struct T {
    n: i32,
}

#[derive(Debug)]
struct Op {
    op: Token,
}

#[derive(Debug)]
enum E {
    E1 { t: Box<T>, o: Box<Op>, e: Box<E> },
    E2 { t: Box<T> },
    EPS,
}

fn is_ws(t: &Token) -> bool {
    match t {
        Token::WS(_) => true,
        _ => false,
    }
}

fn parse_t(toks: &Vec<Token>, idx: usize) -> (Option<T>, usize) {
    if idx >= toks.len() {
        return (None, idx);
    }
    match &toks[idx] {
        Token::NUMBER(n) => {
            return (
                Some(T {
                    n: n.parse::<i32>().unwrap(),
                }),
                idx + 1,
            );
        }
        _ => return (None, idx),
    }
}

fn parse_op(toks: &Vec<Token>, idx: usize) -> (Option<Op>, usize) {
    if idx >= toks.len() {
        return (None, idx);
    }
    match &toks[idx] {
        Token::ADD(op) => (
            Some(Op {
                op: Token::ADD((*op).clone()),
            }),
            idx + 1,
        ),
        _ => return (None, idx),
    }
}

fn parse_e(toks: &Vec<Token>, idx: usize) -> (Option<E>, usize) {
    if idx >= toks.len() {
        return (None, idx);
    }
    // let rollback = idx;
    let (parse_one, new_idx) = parse_t(toks, idx);
    if let Some(one) = parse_one {
        let idx = new_idx;
        let (parse_two, new_idx) = parse_op(toks, idx);
        if let Some(two) = parse_two {
            let idx = new_idx;
            let (parse_three, new_idx) = parse_e(toks, idx);
            if let Some(three) = parse_three {
                return (
                    Some(E::E1 {
                        t: Box::new(one),
                        o: Box::new(two),
                        e: Box::new(three),
                    }),
                    new_idx,
                );
            }
        }
    }
    let (parse, idx) = parse_t(toks, idx);
    if let Some(one) = parse {
        return (Some(E::E2 { t: Box::new(one) }), idx);
    }
    return (None, idx);
}

fn main() {
    print!("Enter input expression: ");
    io::stdout().flush();
    let input = read_input().unwrap();
    let tokens = lex(&input.to_string());
    let tokens = tokens.into_iter().filter(|i| !is_ws(i)).collect();
    // println!("{:?}", tokens);
    let (parsed, idx) = parse_e(&tokens, 0);
    if idx != tokens.len() {
        // println!("{}", idx);
        println!("failed to parse full input {} {}!", idx, tokens.len());
    } else {
        match parsed {
            Some(e) => println!("{:?}", e),
            None => println!("failed to parse!"),
        }
    }
}
