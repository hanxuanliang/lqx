#![allow(dead_code)]

use std::fmt::Debug;

use super::{
    error::PError,
    token::{LqToken, TokenKind},
    IResult, Input,
};

use nom::{
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, tuple},
    Slice,
};

pub fn separated_list<'a, F, O>(
    opener: TokenKind,
    closer: TokenKind,
    sep: TokenKind,
    element_parser: F,
    _wherein: &'static str,
    _expected: &'static str,
) -> impl Fn(Input) -> IResult<Vec<O>>
where
    F: Clone + Copy + FnMut(Input) -> IResult<O>,
    O: Debug,
{
    move |input| {
        let (rest, elements) = tuple((
            match_token(opener.clone()),
            separated_list1(
                match_token(sep.clone()),
                delimited(
                    opt(match_token(TokenKind::Whitespace)),
                    element_parser,
                    opt(match_token(TokenKind::Whitespace)),
                ),
            ),
            opt(match_token(sep.clone())),
            match_token(closer.clone()),
        ))(input)
        .map(|(rest, (_, elements, _, _))| (rest, elements))?;

        Ok((rest, elements))
    }
}

pub fn match_token(kind: TokenKind) -> impl Fn(Input) -> IResult<&LqToken> {
    move |i: Input| match i.get(0) {
        Some(token) if token.kind == kind => Ok((i.slice(1..), token)),
        Some(token) => Err(nom::Err::Error(PError(format!(
            "Expected API Token {kind}, found {} at {}",
            token.kind, token.at
        )))),
        _ => Err(nom::Err::Error(PError(format!(
            "LoqQuery Token {kind} does not match",
        )))),
    }
}

pub fn match_text(text: &'static str) -> impl Fn(Input) -> IResult<&LqToken> {
    move |i| match i.get(0).filter(|token| token.text() == text) {
        Some(token) => Ok((i.slice(1..), token)),
        None => Err(nom::Err::Error(PError(format!(
            "LoqQuery Text {text} does not match",
        )))),
    }
}

pub fn label_identifier(input: Input) -> IResult<String> {
    map(match_token(TokenKind::Identifier), |token| {
        token.text().to_string()
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::token::{LqTokenizer, TokenKind};

    #[test]
    fn test_separated_list() {
        let input = LqTokenizer::new("(foo,bar,   baz, )").collect::<Vec<_>>();

        let (_, elements) = separated_list(
            TokenKind::OpenParen,
            TokenKind::CloseParen,
            TokenKind::Comma,
            label_identifier,
            "test_separated_list",
            "test_separated_list",
        )(&input)
        .unwrap();

        for (i, element) in elements.iter().enumerate() {
            println!("element {}: {:#?}", i, element);
        }
    }
}
