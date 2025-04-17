use crate::ast::agda_expr::AgdaExpr;
use crate::ast::program::Program;
use crate::ast::program::DefinitionInserter;
use crate::ast::program::PostulateInserter;
use crate::ast::theorem_decl::Theorem;
use crate::ast::dependent_function::DependentFunction;
use crate::ast::application::Application;
use crate::ast::binary_op::BinOperator;
use crate::ast::function_type::FunctionType;
use crate::ast::abstraction::Abstraction;
use crate::ast::var_declaration::VarDecl;
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::ast::operator::Operator;
use crate::ast::top_decl::TDeclaration;
use crate::composer::case_converter::{convert_case, CaseStyle};
use crate::wordnet::interface::get_meanings;
use crate::{abstraction, app, bin_op, dependent_function, function_type, term, theorem, var_decl, WORDS_IN_EXISTENCE};
use crate::ast::top_decl::TDeclaration::VariableDecl;
use crate::interpreter::interpretation_map::insert_interpretation;



pub enum SynsetStrategy {
    Ignore, BestMatch, AllMeanings
}

pub enum SynsetRelevancyStrategy {
    Ignore, Relevant, All
}



/// Function to build agda code representing a synonymous relation between two properties, builds a
/// propositional equality to postulate, then derives an explicit pointwise identity function, allowing
/// Agsy Auto to interact with the equality.
pub fn build_agda_synonym(property: &str, synonym: &str, f: &mut Program) {

    /* Conversion to `is` notation to match existing properties */
    let is_property = convert_case(format!("is_{}", property).as_str(), CaseStyle::CamelCase);
    let is_synonym = convert_case(format!("is_{}", synonym).as_str(), CaseStyle::CamelCase);

    /* Add a term of the identity type to the postulate */
    let equality_identifier: String = format!("{}_syn_{}", property, synonym);

    let _type = bin_op!(term!(is_property.clone()), term!(is_synonym.clone()), Operator::PropEq);
    let entry = var_decl!(equality_identifier.clone(), _type);
    f.insert_postulate(VariableDecl(entry.clone()));

    insert_interpretation(TDeclaration::VariableDecl(entry), format!("{} is synonymous with {}", property, synonym));

    /*
     * The following code dervies a pointwise equality function from the above declared
     * identity type. The function body for this adopts a general form as follows.
     *
     * `λ (e) → λ (m) → subst (λ (X) → X e) identity_proof m`
     */
    let ast = Box::from(abstraction!(
        "e",
        abstraction!(
            "m",
        app!(
                app!(
                    term!("subst"),
                    abstraction!(
                        "X",
                        app!(
                            term!("X"),
                            term!("e")
                        )
                    )
                ),
                app!(
                    term!(equality_identifier.clone()),
                    term!("m")
                )
            )
        )
    ));

    /* Next, the type header for this, following `(e : Entity) → is_p1 e → is_p2 e` */
    let type_header = dependent_function!(var_decl!("e", term!("Entity")),
        function_type!(app!(term!(is_synonym.clone()), term!("e")), app!(term!(is_property.clone()), term!("e"))));

    let theorem = theorem!(format!("{}_syn_{}_pointwise", property, synonym), type_header, Some(ast), None);
    let function_def = theorem;

    /* These definitions are bundled as the full function, and inserted into the file */
    f.insert_definition(function_def);
}



/// Function to handle synonyms as Propositional Equalities (identity types), additionally derives a
/// pointwise equality function to enable Agsy Auto Compatability. Relies on the [`wordnet`] module to
/// find synonyms, then builds agda code & postulate using [`build_agda_synonym`].
pub fn handle_synonyms(property: &str, f: &mut Program) {

    /* todo: Extend these to the CLI interface. */
    let SYNSTRAT: SynsetStrategy = SynsetStrategy::AllMeanings;
    let SYNRELEVANT: SynsetRelevancyStrategy = SynsetRelevancyStrategy::Relevant;
    if let SynsetStrategy::Ignore = SYNSTRAT { return; }

    /* Get wordnet matches for interpretations of the word, if none then assign as empty vec */
    let wordnet_matches = get_meanings(property).unwrap_or_default();

    /* Get the current list of words in existence, this is used to prune the neighbours */
    let words_in_existence_snapshot = {
        let words_in_existence = WORDS_IN_EXISTENCE.lock().unwrap();
        words_in_existence.clone()
    };

    /* Iterate over each interpretation, processing its syonyms.  */
    for wordnode in wordnet_matches {
        for synonym in wordnode.synonyms {

            /* Often, it's possible to prune the search by only looking for other words used in the sentence */
            if matches!(SYNRELEVANT, SynsetRelevancyStrategy::Relevant) && !words_in_existence_snapshot.contains(&synonym) {
                continue;
            } else {
                /* Build the agda synonym to the file */
                build_agda_synonym(property, &*synonym, f);
            }
        }

        /* If best match is enabled, then we don't search every interpretation */
        if let SynsetStrategy::BestMatch = SYNSTRAT { return; }
    }
}