#![allow(dead_code)]

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
        Self { token_type, lexeme }
    }
}

pub fn tokenize(program: String) -> Vector<Token> {
    #[tailcall]
    fn tokenize_inner(p: List<char>, tokens: Vector<Token>) -> Vector<Token> {
        match p.clone().head() {
            None => tokens,
            Some(curr) => match curr {
                '[' => tokenize_inner(
                    p.tail(),
                    tokens.conj(Token::new(TokenType::LeftBracket, String::from(curr))),
                ),
                ']' => tokenize_inner(
                    p.tail(),
                    tokens.conj(Token::new(TokenType::RightBracket, String::from(curr))),
                ),
                '.' => tokenize_inner(
                    p.tail(),
                    tokens.conj(Token::new(TokenType::Dot, String::from(curr))),
                ),
                ',' => tokenize_inner(
                    p.tail(),
                    tokens.conj(Token::new(TokenType::Comma, String::from(curr))),
                ),
                '<' => match p.clone().tail().head() {
                    None => tokens,
                    Some('-') => tokenize_inner(
                        p.tail(),
                        tokens.conj(Token::new(TokenType::LeftArrow, String::from("<-"))),
                    ),
                    Some(_) => tokenize_inner(p.tail(), tokens),
                },
                c if c.is_alphabetic() => tokenize_identifier(p.tail(), tokens, String::from(curr)),
                c if c.is_ascii_digit() => {
                    tokenize_number(p.tail(), tokens, String::from(curr), false)
                }
                _ => tokenize_inner(p.tail(), tokens),
            },
        }
    }

    #[tailcall]
    fn tokenize_identifier(
        p: List<char>,
        tokens: Vector<Token>,
        accumulated: String,
    ) -> Vector<Token> {
        match p.clone().head() {
            None => tokens.conj(Token::new(TokenType::Identifier, accumulated)),
            Some(nxt) => match nxt {
                c if c.is_alphabetic() => {
                    tokenize_identifier(p.tail(), tokens, accumulated.conj(nxt))
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
        p: List<char>,
        tokens: Vector<Token>,
        accumulated: String,
        dot_appeard: bool,
    ) -> Vector<Token> {
        match p.clone().head() {
            None => tokens.conj(Token::new(TokenType::Number, accumulated)),
            Some(nxt) => match nxt {
                c if c.is_ascii_digit() => {
                    tokenize_number(p.tail(), tokens, accumulated.conj(nxt), dot_appeard)
                }
                '.' if !dot_appeard => {
                    tokenize_number(p.tail(), tokens, accumulated.conj(nxt), true)
                }
                _ => tokenize_inner(p, tokens.conj(Token::new(TokenType::Number, accumulated))),
            },
        }
    }

    tokenize_inner(program.chars().seq(), vector![])
}
