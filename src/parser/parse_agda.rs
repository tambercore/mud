use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, multispace0},
    combinator::{map, opt},
    multi::{many1},
    sequence::{preceded, tuple},
    IResult,
};
use nom::bytes::complete::tag;
use crate::ast::{abstraction::Abstraction, application::Application, agda_expr::AgdaExpr, record_projection::RecordProjection};
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::term;



/// Function to parse an Agda expression from a string.
pub fn parse_agda(expr: String) -> AgdaExpr {
    match parse_agda_expr(&expr) {
        Ok((_, parsed)) => parsed,
        Err(_) => panic!("Failed to parse Agda expression"),
    }
}



/// Function to parse a term, accepting any non-whitespace and non-lambda characters.
fn parse_term(input: &str) -> IResult<&str, String> {
    take_while1(|c: char| !c.is_whitespace() && c != 'λ' && c != '.' && c != '→' && c != '(' && c != ')')(input)
        .map(|(next_input, term)| (next_input, term.to_string()))
}



/// Function to parse a record projection.
fn parse_proj(input: &str) -> IResult<&str, RecordProjection> {
    let (input, (_, obj, _, _, field)) = tuple((
        opt(multispace0),
        parse_term,
        opt(multispace0),
        char('.'),
        parse_agda_expr
    ))(input)?;

    Ok((input, RecordProjection {
        lhs: Box::from(term!(obj)),
        rhs: Box::from(field)
    }))
}



/// Function to parse a lambda abstraction.
fn parse_abstraction(input: &str) -> IResult<&str, Abstraction> {
    let (input, _) = char('λ')(input)?;

    let (input, params) = many1(preceded(opt(multispace0), parse_term))(input)?;

    let (input, _) = preceded(multispace0, tag("→"))(input)?;
    let (input, body) = preceded(opt(multispace0), parse_agda_expr)(input)?;

    /* Convert `λ a b c → body` into nested abstractions: λ a -> λ b -> λ c -> body */
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



/// Function to parse an expression within parentheses.
fn parse_parentheses_expr(input: &str) -> IResult<&str, AgdaExpr> {
    let (input, _) = opt(multispace0)(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = parse_agda_expr(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, expr))
}



/// Function to parse an application of expressions.
fn parse_application(input: &str) -> IResult<&str, AgdaExpr> {
    let (input, mut terms) = many1(preceded(multispace0, parse_base_expr))(input)?;
    let mut iter = terms.into_iter();
    let first = iter.next().unwrap();

    /* Chain applications left-associatively */
    let application = iter.fold(first, |acc, term| {
        AgdaExpr::App(Application {
            lhs: Box::new(acc),
            rhs: Box::new(term),
        })
    });

    Ok((input, application))
}



/// Function to parse a base-level expression.
fn parse_base_expr(input: &str) -> IResult<&str, AgdaExpr> {
    alt((
        parse_parentheses_expr,
        map(parse_abstraction, AgdaExpr::Abs),
        map(parse_proj, AgdaExpr::RecProj),
        map(parse_term, AgdaExpr::Term),
    ))(input)
}



/// Function to parse an Agda expression, handling applications and base expressions.
fn parse_agda_expr(input: &str) -> IResult<&str, AgdaExpr> {
    alt((parse_application, parse_base_expr))(input)
}



#[cfg(test)]
mod tests {
    use crate::{abstraction, app, record_projection};
    use super::*;

    #[test]
    fn test_lambda_parsing() {
        let input = "λ x y z → f x y z".to_string();
        let parsed = parse_agda(input);
        let expected = abstraction!("x", abstraction!("y", abstraction!("z", app!( app!(app!(term!("f") , term!("x")) , term!("y")), term!("z")))));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_assoc() {
        let input = "f a b".to_string();
        let parsed = parse_agda(input);
        let expected = app!(app!(term!("f"), term!("a")), term!("b"));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_sample() {
        let input = "Man꜀ (z .KnowledgeBaseᵣ.j₁ .ManSocratesᵣ.e₁) (z .KnowledgeBaseᵣ.j₁ .ManSocratesᵣ.p₀)".to_string();
        let parsed = parse_agda(input);
        let expected = app!(
                app!(
                    term!("Man꜀"),
                    record_projection!(
                        term!("z"),
                        record_projection!(
                            term!("KnowledgeBaseᵣ"),
                            record_projection!(
                                term!("j₁"),
                                record_projection!(term!("ManSocratesᵣ"), term!("e₁"))
                            )
                        )
                    )
                ),
                record_projection!(
                    term!("z"),
                    record_projection!(
                        term!("KnowledgeBaseᵣ"),
                        record_projection!(
                            term!("j₁"),
                            record_projection!(term!("ManSocratesᵣ"), term!("p₀"))
                        )
                    )
                )
            );
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_sample2() {
        let input = "λ z →
          AmberAmber꜀ (z .KnowledgeBaseᵣ.j₁ .TobyAmberᵣ.e₁)
          (z .KnowledgeBaseᵣ.j₁ .TobyAmberᵣ.p₁)
          (z .KnowledgeBaseᵣ.j₁ .TobyAmberᵣ.p₁)".to_string();

        let parsed = parse_agda(input);

        println!("{:?}", parsed);
    }


}