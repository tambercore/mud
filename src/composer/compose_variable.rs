use crate::ast::record_projection::RecordProjection;
use crate::ast::function_type::FunctionType;
use crate::composer::case_converter::{convert_case, CaseStyle};
use crate::composer::postulate::{DefinitionInserter, PostulateInserter};
use crate::{app, function_type, record_projection, term};
use crate::ast::agda_expr::AgdaExpr;
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::ast::agda_expr::AgdaExpr::App;
use crate::ast::application::Application;
use crate::ast::postulate_decl::Postulate;
use crate::ast::program::Program;
use crate::ast::record_decl::Record;
use crate::ast::top_decl::TDeclaration::RecordDecl;
use crate::ast::var_declaration::VarDecl;
use crate::composer::compose_predicate::generate_predicate_output;
use crate::composer::lambda_to_types::generate_function_header;
use crate::composer::langtree::Token;
use crate::monty::fresh_variable::to_unicode_subscript;

pub fn compose_variable(token: Token, f: &mut Program, props: Vec<Token>) -> (String, AgdaExpr) {

    /* Handle negation layers and remove instances of negation from props */
    let negation_layers = props.iter().filter(|p| *p == "not").count() as i32;
    let props: Vec<Token> = props.into_iter().filter(|p| p != "not").collect();

    /* Generate Fields */
    let mut predicate_iden = convert_case(format!("is_{}", token).as_str(), CaseStyle::CamelCase);

    let field = VarDecl {
        iden: "e₁".to_string(),
        _type: term!("Entity"),
    };
    let mut fields: Vec<VarDecl> = vec![field ];
    let app: AgdaExpr = app!(*term!(predicate_iden.clone()), *term!("e₁"));
    let proj_field = VarDecl {
        iden: "p₁".to_string(),
        _type: Box::from(app),
    };

    fields.push(proj_field);

    /* Generate each property as a proof */
    let mut types: Vec<AgdaExpr> = vec![];
    for p in (props.clone()) {
        let mut c_predicate = convert_case(format!("is_{}", p).as_str(), CaseStyle::CamelCase);
        let __type = app!(*term!(c_predicate.clone()), *term!("e₁"));
        types.push(__type);

        let postulate_entry = VarDecl {
            iden: c_predicate,
            _type: Box::from(generate_function_header(1))
        };
        f.insert_postulate(postulate_entry);
    }

    /* Handle cases without negation */
    if negation_layers == 0 {
        let mut counter: usize = 0;
        for _type in types {
            let field = VarDecl {
                iden : format!("p{}", to_unicode_subscript(counter)),
                _type: Box::from(_type)
            };
            fields.push(field);
            counter = counter + 1;
        }
    }

    /* Handle cases with negation */
    else {
        let mut inner = generate_predicate_output(types.into_iter().map(|x| {Box::from(x)}).collect());
        for _ in (0..negation_layers) { inner = Box::from(function_type!(*inner, *term!("⊥"))); }

        let field = VarDecl {
            iden: format!("p{}", to_unicode_subscript(0)),
            _type: inner,
        };
        fields.push(field);
    }

    /* Now, we need to insert the record for it */
    let props_iden = format!("{}{}",
         props.iter().fold(String::new(), |mut acc, p| { acc.push_str(&p); acc.push('_'); acc }),
         token);

    let record_name = format!("{}ᵣ", convert_case(props_iden.clone().as_str(), CaseStyle::PascalCase));
    let constructor_name = format!("{}꜀", convert_case(props_iden.clone().as_str(), CaseStyle::PascalCase));

    let rec = Record {
        record_iden: record_name.clone(),
        constructor_iden: constructor_name,
        fields: fields,
        comment : None
    };

    let postulate_entry = VarDecl {
        iden: predicate_iden.clone(),
        _type: Box::from(generate_function_header(1)),
    };

    /* We need to also update the postulate to include the isType function */
    f.insert_postulate(postulate_entry);
    f.insert_definition(RecordDecl(rec));

    let proj = record_projection!(*term!(record_name.clone()), *term!("e₁"));
    let projection = app!(proj, *term!("e₁"));
    (record_name, projection)
}
