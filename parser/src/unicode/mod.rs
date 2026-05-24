use crate::sat;
use crate::{ParseError, Parser, many, some};
use std::collections::VecDeque;
use std::str::Chars;

pub fn digit<'a>() -> impl Parser<Chars<'a>, O = char> {
    sat(|c: &char| c.is_ascii_digit())
}
pub fn lower<'a>() -> impl Parser<Chars<'a>, O = char> {
    sat(|c: &char| c.is_ascii_lowercase())
}
pub fn upper<'a>() -> impl Parser<Chars<'a>, O = char> {
    sat(|c: &char| c.is_ascii_uppercase())
}
pub fn alpha<'a>() -> impl Parser<Chars<'a>, O = char> {
    sat(|c: &char| c.is_ascii_alphabetic())
}
pub fn alnum<'a>() -> impl Parser<Chars<'a>, O = char> {
    sat(|c: &char| c.is_ascii_alphanumeric())
}
pub fn char<'a>(x: char) -> impl Parser<Chars<'a>, O = char> {
    sat(move |c: &char| c == &x)
}
pub fn string<'a>(xs: &'a str) -> impl Parser<Chars<'a>, O = String> {
    |input: Chars<'a>| {
        str(xs.chars())
            .map(|xs| xs.into_iter().collect())
            .parse(input.clone())
            .or(Err((ParseError::NotFound(xs.to_string()), input)))
    }
}
fn str<'a>(mut xs: Chars<'a>) -> impl Parser<Chars<'a>, O = VecDeque<char>> {
    let x = xs.next();
    move |input: Chars<'a>| match x {
        Some(x) => {
            let (x, rest) = char(x).parse(input)?;
            let (mut xs, rest) = str(xs.clone()).parse(rest)?;
            xs.push_front(x);
            Ok((xs, rest))
        }
        None => Ok((VecDeque::new(), input)),
    }
}
fn nat<'a>() -> impl Parser<Chars<'a>, O = i64> {
    |input: Chars<'a>| {
        let (xs, rest) = some(digit()).parse(input.clone())?;
        match xs.into_iter().collect::<String>().parse() {
            Ok(xs) => Ok((xs, rest)),
            Err(err) => Err((ParseError::ParseIntError(err), input)),
        }
    }
}
fn int<'a>() -> impl Parser<Chars<'a>, O = i64> {
    (|input| {
        let (_, rest) = char('-').parse(input)?;
        let (n, rest) = nat().parse(rest)?;
        Ok((-n, rest))
    })
    .or(nat())
}
fn space<'a>() -> impl Parser<Chars<'a>, O = ()> {
    |input: Chars<'a>| {
        let (_, rest) = many(sat(|x: &char| x.is_ascii_whitespace())).parse(input)?;
        Ok(((), rest))
    }
}
pub fn token<'a, P>(p: P) -> impl Parser<Chars<'a>, O = P::O>
where
    P: Parser<Chars<'a>>,
{
    move |input| {
        let (_, rest) = space().parse(input)?;
        let (v, rest) = p.parse(rest)?;
        let (_, rest) = space().parse(rest)?;
        Ok((v, rest))
    }
}
pub fn natural<'a>() -> impl Parser<Chars<'a>, O = i64> {
    token(nat())
}
fn integer<'a>() -> impl Parser<Chars<'a>, O = i64> {
    token(int())
}
pub fn symbol<'a>(xs: &'a str) -> impl Parser<Chars<'a>, O = String> {
    token(string(xs))
}

#[cfg(test)]
mod tests;
