#[cfg(test)]
use super::*;
use crate::*;

#[test]
fn item_test() {
    for (input, parser, expected) in [
        ("", item::<Chars>(), Err((ParseError::Empty, ""))),
        ("abc", item(), Ok(('a', "bc"))),
    ] {
        parse_item_test(input, parser, expected);
    }
}
#[test]
fn map_test() {
    let to_upper = |c: char| c.to_ascii_uppercase();
    for (input, parser, expected) in [
        ("", item().map(to_upper), Err((ParseError::Empty, ""))),
        ("abc", item().map(to_upper), Ok(('A', "bc"))),
    ] {
        parse_item_test(input, parser, expected);
    }
}
#[test]
fn char_test() {
    for (input, parser, expected) in [("abc", char('a'), Ok(('a', "bc")))] {
        parse_item_test(input, parser, expected);
    }
}
#[test]
fn string_test() {
    for (input, parser, expected) in [
        ("abcdef", string("abc"), Ok((String::from("abc"), "def"))),
        (
            "ab1234",
            string("abc"),
            Err((ParseError::NotFound(String::from("abc")), "ab1234")),
        ),
    ] {
        parse_string_test(input, parser, expected)
    }
}
#[test]
fn many_digit_test() {
    for (input, parser, expected) in [
        ("123abc", many(digit()), Ok((String::from("123"), "abc"))),
        ("abc", many(digit()), Ok((String::new(), "abc"))),
    ] {
        parse_char_sequence_test(input, parser, expected)
    }
}
#[test]
fn some_digit_test() {
    for (input, parser, expected) in [
        ("123abc", some(digit()), Ok((String::from("123"), "abc"))),
        ("abc", some(digit()), Err((ParseError::NotSatisfied, "abc"))),
    ] {
        parse_char_sequence_test(input, parser, expected)
    }
}
#[test]
fn nat_test() {
    for (input, parser, expected) in [("123 abc", nat(), Ok((123, " abc")))] {
        parse_item_test(input, parser, expected);
    }
}
#[test]
fn space_test() {
    for (input, parser, expected) in [(" abc", space(), Ok(((), "abc")))] {
        parse_item_test(input, parser, expected);
    }
}
#[test]
fn int_test() {
    for (input, parser, expected) in [("-123 abc", int(), Ok((-123, " abc")))] {
        parse_item_test(input, parser, expected);
    }
}
fn parse_item_test<'a, P, T>(input: &'a str, parser: P, expected: ParseResult<&'a str, T>)
where
    P: Parser<Chars<'a>, O = T>,
    T: std::fmt::Debug + PartialEq,
{
    assert_eq!(
        expected,
        parser
            .parse(input.chars())
            .map(|(output, rest)| (output, rest.as_str()))
            .map_err(|(err, input)| (err, input.as_str()))
    )
}
fn parse_string_test<'a, P>(input: &'a str, parser: P, expected: ParseResult<&'a str, String>)
where
    P: Parser<Chars<'a>, O = String>,
{
    assert_eq!(
        expected,
        parser
            .parse(input.chars())
            .map(|(output, rest)| (output, rest.as_str()))
            .map_err(|(err, input)| (err, input.as_str()))
    )
}
fn parse_char_sequence_test<'a, P, S>(
    input: &'a str,
    parser: P,
    expected: ParseResult<&'a str, String>,
) where
    P: Parser<Chars<'a>, O = S>,
    S: IntoIterator<Item = char>,
{
    assert_eq!(
        expected,
        parser
            .parse(input.chars())
            .map(|(output, rest)| (output.into_iter().collect(), rest.as_str()))
            .map_err(|(err, input)| (err, input.as_str()))
    )
}
