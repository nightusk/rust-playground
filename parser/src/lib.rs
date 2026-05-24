#![allow(unused)]

use std::collections::VecDeque;
use std::num::ParseIntError;

pub mod unicode;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Empty,
    NotSatisfied,
    NotFound(String),
    ParseIntError(ParseIntError),
}
type ParseResult<I, O> = Result<(O, I), (ParseError, I)>;
pub trait Parser<I> {
    type O;
    fn parse(&self, input: I) -> ParseResult<I, Self::O>;
    fn or<P>(&self, other: P) -> impl Parser<I, O = Self::O>
    where
        I: Clone,
        P: Parser<I, O = Self::O>,
    {
        move |input: I| self.parse(input.clone()).or(other.parse(input))
    }
    fn map<F, O>(self, f: F) -> impl Parser<I, O = O>
    where
        Self: Sized,
        F: Fn(Self::O) -> O,
    {
        move |input| {
            let (output, rest) = self.parse(input)?;
            Ok((f(output), rest))
        }
    }
}
impl<F, I, O> Parser<I> for F
where
    F: Fn(I) -> ParseResult<I, O>,
{
    type O = O;
    fn parse(&self, input: I) -> ParseResult<I, Self::O> {
        self(input)
    }
}
fn item<I>() -> impl Parser<I, O = I::Item>
where
    I: Iterator,
{
    |mut input: I| match input.next() {
        Some(item) => Ok((item, input)),
        None => Err((ParseError::Empty, input)),
    }
}
fn sat<F, I>(p: F) -> impl Parser<I, O = I::Item>
where
    F: Fn(&I::Item) -> bool,
    I: Iterator + Clone,
{
    move |input: I| {
        let (item, rest) = item().parse(input.clone())?;
        if p(&item) {
            Ok((item, rest))
        } else {
            Err((ParseError::NotSatisfied, input))
        }
    }
}
pub fn many<I, P>(p: P) -> impl Parser<I, O = Vec<P::O>>
where
    I: Clone,
    P: Parser<I>,
{
    move |mut input: I| {
        let mut xs = Vec::new();
        while let Ok((x, rest)) = p.parse(input.clone()) {
            xs.push(x);
            input = rest;
        }
        Ok((xs, input))
    }
}
pub fn some<I, P>(p: P) -> impl Parser<I, O = Vec<P::O>>
where
    I: Clone,
    P: Parser<I>,
{
    move |input: I| {
        let (x, mut input) = p.parse(input)?;
        let mut xs = Vec::new();
        xs.push(x);
        while let Ok((x, rest)) = p.parse(input.clone()) {
            xs.push(x);
            input = rest;
        }
        Ok((xs, input))
    }
}
