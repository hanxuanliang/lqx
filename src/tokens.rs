#![allow(dead_code)]

use std::ops::Range;

use logos::{Lexer, Logos};

pub struct LqTokenizer<'a> {
    source: &'a str,
    lexer: Lexer<'a, TokenKind>,
}

impl<'a> LqTokenizer<'a> {
    fn new(source: &'a str) -> Self {
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
                at: self.lexer.slice(),
                span: self.lexer.span(),
            }),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LqToken<'a> {
    kind: TokenKind,
    pub at: &'a str,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Logos)]
pub enum TokenKind {
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,

    #[token("map")]
    Mapper,
    #[token("as")]
    As,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex("[0-9]+")]
    Number,

    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Whitespace => write!(f, "whitespace"),
            TokenKind::Mapper => write!(f, "map"),
            TokenKind::As => write!(f, "as"),
            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::Number => write!(f, "number"),
            TokenKind::OpenBrace => write!(f, "{{"),
            TokenKind::CloseBrace => write!(f, "}}"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Comma => write!(f, ","),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let source = "map { .1 as a1, .3 as b1 }";
        let res = LqTokenizer::new(source).collect::<Vec<_>>();

        for token in &res {
            println!("{:#?}", token);
        }
    }
}
