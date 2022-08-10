#![allow(dead_code)]
use std::iter::Peekable;
use std::str::Chars;

use crate::collection_utils::*;
use im_rc::vector;
use im_rc::vector::*;
use tailcall::tailcall;

#[derive(Debug, Clone)]
pub enum TokenType {
    Number,
    Identifier,
    LeftBracket,
    RightBracket,
    Dot,
    Comma,
    LeftArrow,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String) -> Self {
        Self {
            token_type,
            lexeme,
        }
    }
}

pub fn tokenize(program: String) -> Vector<Token> {
    #[tailcall]
    fn tokenize_inner(mut p: Peekable<Chars>, tokens: Vector<Token>) -> Vector<Token> {
        match p.next() {
            None => tokens,
            Some(curr) => match curr {
                '[' => tokenize_inner(
                    p,
                    tokens.conj(Token::new(TokenType::LeftBracket, String::from(curr))),
                ),
                ']' => tokenize_inner(
                    p,
                    tokens.conj(Token::new(TokenType::RightBracket, String::from(curr))),
                ),
                '.' => tokenize_inner(
                    p,
                    tokens.conj(Token::new(TokenType::Dot, String::from(curr))),
                ),
                ',' => tokenize_inner(
                    p,
                    tokens.conj(Token::new(TokenType::Comma, String::from(curr))),
                ),
                c if c.is_alphabetic() => tokenize_identifier(p, tokens, String::from(curr)),
                c if c.is_ascii_digit() => tokenize_number(p, tokens, String::from(curr), false),
                _ => tokenize_inner(p, tokens),
            },
        }
    }

    #[tailcall]
    fn tokenize_identifier(
        mut p: Peekable<Chars>,
        tokens: Vector<Token>,
        accumulated: String,
    ) -> Vector<Token> {
        match p.peek() {
            None => tokens.conj(Token::new(TokenType::Identifier, accumulated)),
            Some(nxt) => match nxt {
                c if c.is_alphabetic() => {
                    let accumulated = accumulated.conj(p.next().unwrap());
                    tokenize_identifier(p, tokens, accumulated)
                }
                _ => tokenize_inner(
                    p,
                    tokens.conj(Token::new(TokenType::Identifier, accumulated)),
                ),
            },
        }
    }

    #[tailcall]
    fn tokenize_number(
        mut p: Peekable<Chars>,
        tokens: Vector<Token>,
        accumulated: String,
        dot_appeard: bool,
    ) -> Vector<Token> {
        match p.peek() {
            None => tokens.conj(Token::new(TokenType::Number, accumulated)),
            Some(nxt) => match nxt {
                c if c.is_ascii_digit() => {
                    let accumulated = accumulated.conj(p.next().unwrap());
                    tokenize_number(p, tokens, accumulated, dot_appeard)
                }
                '.' if !dot_appeard => {
                    let accumulated = accumulated.conj(p.next().unwrap());
                    tokenize_number(p, tokens, accumulated, true)
                }
                _ => tokenize_inner(p, tokens.conj(Token::new(TokenType::Number, accumulated))),
            },
        }
    }

    tokenize_inner(program.chars().peekable(), vector![])
}
