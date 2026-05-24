use parser::Parser;
use parser::unicode::{natural, symbol};
use std::io;
use std::io::prelude::*;
use std::str::Chars;

fn expr<'a>() -> impl Parser<Chars<'a>, O = i64> {
    |input: Chars<'a>| {
        let (t, rest) = term().parse(input)?;
        let Ok((_, rest)) = symbol("+").parse(rest.clone()) else {
            return Ok((t, rest));
        };
        let (e, rest) = expr().parse(rest)?;
        Ok((t + e, rest))
    }
}
fn term<'a>() -> impl Parser<Chars<'a>, O = i64> {
    |input: Chars<'a>| {
        let (f, rest) = factor().parse(input)?;
        let Ok((_, rest)) = symbol("*").parse(rest.clone()) else {
            return Ok((f, rest));
        };
        let (t, rest) = term().parse(rest)?;
        Ok((f * t, rest))
    }
}
fn factor<'a>() -> impl Parser<Chars<'a>, O = i64> {
    (|input: Chars<'a>| {
        let (_, rest) = symbol("(").parse(input)?;
        let (e, rest) = expr().parse(rest)?;
        let (_, rest) = symbol(")").parse(rest)?;
        Ok((e, rest))
    })
    .or(natural())
}
fn eval<'a>(xs: String) -> Result<i64, String> {
    match expr().parse(xs.chars()) {
        Ok((n, rest)) => match rest.clone().peekable().peek() {
            None => Ok(n),
            _ => Err(format!("Unused input: {}", rest.as_str())),
        },
        Err((_, input)) => Err(format!("Invalid input: {}", input.as_str())),
    }
}

fn real_eval_print<R, W>(r: R, mut w: W) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    for line in r.lines() {
        writeln!(w, "{:?}", eval(line?))?
        // match eval(line?) {
        //     Ok(n) => writeln!(w, "{}", n)?,
        //     Err(e) => writeln!(w, "{}", e)?,
        // };
    }
    Ok(())
}

fn main() -> io::Result<()> {
    real_eval_print(io::stdin().lock(), io::stdout().lock())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eval_test() {
        for (input, expected) in [
            ("2*3+4", Ok(10)),
            ("2*(3+4)", Ok(14)),
            ("2*3^4", Err(String::from("Unused input: ^4"))),
            (
                "one plus two",
                Err(String::from("Invalid input: one plus two")),
            ),
        ] {
            assert_eq!(expected, eval(input.to_string()))
        }
    }
}
