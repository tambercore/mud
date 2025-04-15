use crate::Literate;
use crate::TDeclaration;
use std::fs;
use serde::{Deserialize, Serialize};
use warp::{http::Method, Filter};
use crate::{english_to_agda, interpret_holes, literate};
use crate::command_line::output_handler::{create_task, update_task};
use crate::resolver::fill_holes::fill_holes;

#[derive(Debug, Deserialize)]
struct SentenceInput {
    knowledge: Vec<String>,
    conclusions: Vec<String>
}
    
#[derive(Debug, Serialize)]
struct AgdaResponse {
    agda: String,
    premises: Vec<AgdaPremise>,
    conclusions: Vec<AgdaConclusion>
}

#[derive(Debug, Serialize)]
pub struct AgdaPremise {
    pub(crate) text: String,
    pub(crate) ccg_tree: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct AgdaConclusion {
    pub(crate) text: String,
    pub(crate) ccg_tree: String,
    pub(crate) filled: bool
}


pub async fn create_endpoint(output_location: String) {
    /* Enable CORS */
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::POST, Method::GET])
        .allow_headers(vec!["Content-Type"]);

    let output_location_clone = output_location.clone(); // Clone here before passing into closure

    let route = warp::path("agda")
        .and(warp::post())
        .and(warp::body::json::<SentenceInput>())
        .map(move |input: SentenceInput| {
            let output_loc = output_location_clone.clone();

            let (mut agda_file, premises, mut conclusions) = english_to_agda(input.knowledge.clone(), input.conclusions.clone());

            let mut intro_literate = format!("\\section{{Premises (Assumptions)}}\n\n\\begin{{itemize}}");
            for (idx, assumption) in input.knowledge.iter().enumerate() {
                intro_literate.push_str(format!("\\item A{}: {}\n", idx, assumption).as_str());
            }
            intro_literate.push_str(format!("\\end{{itemize}}").as_str());
            agda_file.declarations.push(literate!(intro_literate));

            agda_file.write_to_file(output_loc.clone());

            let (hole_contents, agda_file_str) = fill_holes(output_loc.clone(), &mut conclusions);

            let new_contents = interpret_holes(hole_contents.clone(), &agda_file, conclusions.clone(), agda_file_str);
            /* Read the file as a string. */
            let agda_str = fs::read_to_string(output_loc).expect("Failed to read file");

            warp::reply::json(&AgdaResponse { agda: agda_str, premises, conclusions })
        })
        .with(cors);  // Apply CORS to the route

    println!("Server running on port 12345...");
    warp::serve(route).run(([127, 0, 0, 1], 12345)).await;
}
