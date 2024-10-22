use nom::{
    bytes::complete::{tag, take_until, take_while},
    sequence::delimited,
    IResult,
};

// Block comment: /* ... */
pub fn _parse_block_comment(input: &str) -> IResult<&str, &str> {
    delimited(tag("/*"), take_until("*/"), tag("*/"))(input)
}

// Line comment: // ...
pub fn _parse_line_comment(input: &str) -> IResult<&str, &str> {
    delimited(tag("//"), take_while(|c| c != '\n'), tag("\n"))(input)
}

// Function instead of `let (input, _) = skip_whitespace(input)?;`
pub fn skip_whitespace(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_whitespace())(input)
}
