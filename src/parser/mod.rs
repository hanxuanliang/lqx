use self::token::LqToken;

mod common;

pub mod error;
pub mod token;

pub type Input<'a> = &'a [LqToken<'a>];
pub type IResult<'a, Output> = nom::IResult<Input<'a>, Output, error::PError>;
