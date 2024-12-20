use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    multi::many0,
    sequence::delimited,
    IResult,
};

// Block comment: /* ... */
fn parse_block_comment(input: &str) -> IResult<&str, &str> {
    delimited(tag("/*"), take_until("*/"), tag("*/"))(input)
}

// Line comment: // ...
fn parse_line_comment(input: &str) -> IResult<&str, &str> {
    delimited(tag("//"), take_while(|c| c != '\n'), tag("\n"))(input)
}

/// Skip whitespace and comments (which are treated as whitespace for the parser)
pub fn skip_whitespace(input: &str) -> IResult<&str, &str> {
    let (input, _) = _skip_whitespace(input)?;
    let (input, _) = many0(alt((parse_block_comment, parse_line_comment)))(input).unwrap_or((input, Vec::new()));
    let (input, _) = _skip_whitespace(input)?;
    Ok((input, ""))
}

// Function instead of `let (input, _) = skip_whitespace(input)?;`
fn _skip_whitespace(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_whitespace())(input)
}
