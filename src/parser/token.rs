#![allow(dead_code)]

use std::ops::Range;

use logos::{Lexer, Logos};

pub struct LqTokenizer<'a> {
    source: &'a str,
    lexer: Lexer<'a, TokenKind>,
}

impl<'a> LqTokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            lexer: TokenKind::lexer(source),
        }
    }
}

impl<'a> Iterator for LqTokenizer<'a> {
    type Item = LqToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next() {
            Some(Ok(kind)) => Some(LqToken {
                kind,
                source: self.source,
                at: self.lexer.slice(),
                span: self.lexer.span(),
            }),
            _ => None,
        }
    }
}

#[derive(PartialEq)]
pub struct LqToken<'a> {
    pub source: &'a str,
    pub kind: TokenKind,
    pub at: &'a str,
    pub span: Range<usize>,
}

impl<'a> LqToken<'a> {
    pub fn text(&self) -> &'a str {
        &self.source[self.span.clone()]
    }
}

impl std::fmt::Debug for LqToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} at {}..{}",
            self.kind, self.span.start, self.span.end
        )
    }
}

#[derive(Debug, PartialEq, Logos, Clone)]
pub enum TokenKind {
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,

    #[token("map", ignore(ascii_case))]
    Mapper,
    #[token("as", ignore(ascii_case))]
    As,
    #[token("|")]
    Pipe,

    #[token("select", ignore(ascii_case))]
    Select,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex("[0-9]+")]
    Number,

    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Whitespace => write!(f, "Whitespace"),
            TokenKind::Mapper => write!(f, "Mapper"),
            TokenKind::As => write!(f, "As"),
            TokenKind::Pipe => write!(f, "Pipe"),
            TokenKind::Select => write!(f, "Select"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::Number => write!(f, "Number"),
            TokenKind::OpenParen => write!(f, "OpenParen"),
            TokenKind::CloseParen => write!(f, "CloseParen"),
            TokenKind::OpenBrace => write!(f, "OpenBrace"),
            TokenKind::CloseBrace => write!(f, "CloseBrace"),
            TokenKind::OpenBracket => write!(f, "OpenBracket"),
            TokenKind::CloseBracket => write!(f, "CloseBracket"),
            TokenKind::Dot => write!(f, "Dot"),
            TokenKind::Comma => write!(f, "Comma"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let source = "map { .1 as a1, .3 as b1 } | select count_over_time(__line__[1s])";
        let res = LqTokenizer::new(source).collect::<Vec<_>>();

        for token in &res {
            println!("{:#?}", token);
        }
    }
}
