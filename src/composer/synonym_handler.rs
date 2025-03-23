use crate::composer::case_converter::{convert_case, CaseStyle};
use crate::composer::postulate::{DefinitionInserter, PostulateEntry, PostulateInserter};
use crate::wordnet::interface::get_meanings;
use crate::composer::structures::{AgdaType};
use crate::composer::structures::AgdaType::{Application, PropEq, Simple};
use crate::{astApply, astLambda, astTerm, tToken, λPred, λVar, τApp, τDepFunc, τFunc, τProduct, τPropEq, τRecProj, τSimp, WORDS_IN_EXISTENCE};
use crate::composer::function_def::FunctionDefinition;
use crate::composer::ast::AgdaAst;
use crate::composer::postulate::AgdaStructure::FunctionDef;



/* todo: move these to a higher level when integrating with CLI */
pub enum SynsetStrategy {
    Ignore, BestMatch, AllMeanings
}

pub enum SynsetRelevancyStrategy {
    Ignore, Relevant, All
}



/// Function to build agda code representing a synonymous relation between two properties, builds a
/// propositional equality to postulate, then derives an explicit pointwise identity function, allowing
/// Agsy Auto to interact with the equality.
pub fn build_agda_synonym(property: &str, synonym: &str, f: &mut AgdaFile) {

    /* Conversion to `is` notation to match existing properties */
    let is_property = convert_case(format!("is_{}", property).as_str(), CaseStyle::CamelCase);
    let is_synonym = convert_case(format!("is_{}", synonym).as_str(), CaseStyle::CamelCase);

    /* Add a term of the identity type to the postulate */
    let equality_identifier: String = format!("{}_syn_{}", property, synonym);
    f.insert_postulate(PostulateEntry(
        equality_identifier.clone(),
        *τPropEq!(τSimp!(is_property.clone()), τSimp!(is_synonym.clone())),
    ));

    /*
     * The following code dervies a pointwise equality function from the above declared
     * identity type. The function body for this adopts a general form as follows.
     *
     * `λ (e) → λ (m) → subst (λ (X) → X e) identity_proof m`
     */
    let ast = astLambda!(
        String::from("e"),
        astLambda!(
            String::from("m"),
            astApply!(
                astApply!(
                    astTerm!(String::from("subst")),
                    astLambda!(
                        String::from("X"),
                        astApply!(
                            astTerm!(String::from("X")),
                            astTerm!(String::from("e"))
                        )
                    )
                ),
                astApply!(
                    astTerm!(equality_identifier.clone()),
                    astTerm!(String::from("m"))
                )
            )
        )
    );

    /* Next, the type header for this, following `(e : Entity) → is_p1 e → is_p2 e` */
    let type_header = τDepFunc!(
        "e".parse().unwrap(), τSimp!("Entity".parse().unwrap()),
        τFunc!(τApp!(τSimp!(is_property), τSimp!("e".parse().unwrap())),
            τApp!(τSimp!(is_synonym),  τSimp!("e".parse().unwrap())))
    );

    /* These definitions are bundled as the full function, and inserted into the file */
    f.insert_definition(FunctionDef(FunctionDefinition {
        function_name: (*format!("{}_syn_{}_pointwise", property, synonym)).parse().unwrap(),
        function_type: *type_header,
        function_body: *ast,
    }));
}



/// Function to handle synonyms as Propositional Equalities (identity types), additionally derives a
/// pointwise equality function to enable Agsy Auto Compatability. Relies on the [`wordnet`] module to
/// find synonyms, then builds agda code & postulate using [`build_agda_synonym`].
pub fn handle_synonyms(property: &str, f: &mut AgdaFile) {

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