use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, multispace0},
    combinator::{map, opt},
    multi::{separated_list0, many1},
    sequence::{delimited, preceded, tuple},
    IResult,
};
use crate::ast::{abstraction::Abstraction, application::Application, agda_expr::AgdaExpr, record_projection::RecordProjection};
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::term;

pub fn parse_agda(expr: String) -> AgdaExpr {
    match parse_agda_expr(&expr) {
        Ok((_, parsed)) => parsed,
        Err(_) => panic!("Failed to parse Agda expression"),
    }
}


fn parse_term(input: &str) -> IResult<&str, String> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
        .map(|(next_input, term)| (next_input, term.to_string()))
}

fn parse_proj(input: &str) -> IResult<&str, RecordProjection> {
    let (input, (obj, _, field)) = tuple((parse_term, char('.'), parse_term))(input)?;
    Ok((input, RecordProjection { lhs: Box::from(term!(obj)), rhs: Box::from(term!(field)) }))
}

fn parse_abstraction(input: &str) -> IResult<&str, Abstraction> {
    let (input, _) = char('λ')(input)?;
    let (input, params) = separated_list0(multispace0, parse_term)(input)?;
    let (input, _) = tag("→")(input)?;
    let (input, body) = parse_agda_expr(input)?;

    // Convert `λ a b c → body` into nested abstractions: λ a -> λ b -> λ c -> body
    let abstraction = params.into_iter().rev().fold(body, |acc, param| {
        AgdaExpr::Abs(Abstraction {
            var: param,
            expr: Box::new(acc),
        })
    });

    if let AgdaExpr::Abs(abs) = abstraction {
        Ok((input, abs))
    } else {
        unreachable!()
    }
}
fn parse_application(input: &str) -> IResult<&str, AgdaExpr> {
    let (input, terms) = many1(preceded(multispace0, parse_term))(input)?;

    let mut iter = terms.into_iter();
    let first = iter.next().unwrap();

    let application = iter.fold(AgdaExpr::Term(first), |acc, term| {
        AgdaExpr::App(Application {
            lhs: Box::new(acc),
            rhs: Box::new(AgdaExpr::Term(term)),
        })
    });

    Ok((input, application))
}


fn parse_agda_expr(input: &str) -> IResult<&str, AgdaExpr> {
    alt((
        map(parse_abstraction, AgdaExpr::Abs), // Ensure abstraction is parsed first
        map(parse_proj, AgdaExpr::RecProj),    // Then projections
        parse_application,                     // Then applications
        map(parse_term, AgdaExpr::Term),       // Finally, standalone terms
    ))(input)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_complex_abstraction() {
        let input = "λ z → MortalSocrates꜀ (z .KnowledgeBaseᵣ.j₁ .ManSocratesᵣ.e₁) (z .KnowledgeBaseᵣ.j₁ .ManSocratesᵣ.p₁) (z .KnowledgeBaseᵣ.j₂ .IsManMortalᵣ.p (Man꜀ (z .KnowledgeBaseᵣ.j₁ .ManSocratesᵣ.e₁) (z .KnowledgeBaseᵣ.j₁ .ManSocratesᵣ.p₀)))".to_string();

        let parsed = parse_agda(input);

        println!("parsed: {:?}", parsed);

    }
}
