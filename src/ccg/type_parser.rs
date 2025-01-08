
use super::category::CCGType;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0},
    combinator::map,
    sequence::{delimited, pair, preceded},
    IResult,
};
use nom::multi::many0;



// Helper function to parse atomic categories
fn parse_atomic(input: &str) -> IResult<&str, CCGType> {
    alt((
        map(tag("s"), |_| CCGType::Sentence),
        map(tag("np"), |_| CCGType::NounPhrase),
        map(tag("n"), |_| CCGType::Noun),
        map(tag("[conj]"), |_| CCGType::ConjunctionTag),
        map(tag("conj"), |_| CCGType::Conjunction),
        map(tag("punc"), |_| CCGType::Punctuation),
        map(tag("p"), |_| CCGType::PrepositionalPhrase),
        map(tag("empty"), |_| CCGType::Empty),
    ))(input)
}



// Parser for the backslash operator '\'
pub fn parse_backwards_functor(left: CCGType, right: CCGType) -> CCGType {
    CCGType::BackwardsFunctor(Box::new(left), Box::new(right))
}



// Parser for the forward slash operator '/'
pub fn parse_forward_functor(left: CCGType, right: CCGType) -> CCGType {
    CCGType::ForwardsFunctor(Box::new(left), Box::new(right))
}



// The main parser that handles operators and precedence
pub fn parse_category(input: &str) -> IResult<&str, CCGType> {
    let mut atom = alt((
        delimited(
            preceded(multispace0, char('(')),
            parse_category,
            preceded(multispace0, char(')')),
        ),
        parse_atomic,
    ));

    // Parse an operator and the next atom
    let operator = preceded(multispace0, alt((char('\\'), char('/'))));

    // First, parse the initial atom
    let (input, initial) = atom(input)?;

    // Then, parse zero or more (operator, atom) pairs
    let (input, ops) = many0(pair(operator, atom))(input)?;

    // Now, fold the operations manually into the AST
    let acc = ops.into_iter().fold(initial, |acc, (op, right)| {
        match op {
            '\\' => parse_backwards_functor(acc, right),
            '/' => parse_forward_functor(acc, right),
            _ => acc,
        }
    });

    Ok((input, acc))
}