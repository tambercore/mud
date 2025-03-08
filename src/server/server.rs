use serde::{Deserialize, Serialize};
use warp::{http::Method, Filter};
use crate::english_to_agda;
use warp::reply::Reply;
use warp::cors;

#[derive(Debug, Deserialize)]
struct SentenceInput {
    knowledge: Vec<String>,
    conclusions: Vec<String>
}

#[derive(Debug, Serialize)]
struct AgdaResponse {
    agda: String,
}


pub async fn create_endpoint(filepath: String) {
    /* Enable CORS */
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::POST, Method::GET])
        .allow_headers(vec!["Content-Type"]);

    let route = warp::path("agda")
        .and(warp::post())
        .and(warp::body::json::<SentenceInput>())
        .map(move |input: SentenceInput| {
            let filepath = filepath.clone();
            let response = english_to_agda(input.knowledge, input.conclusions, filepath);
            warp::reply::json(&AgdaResponse { agda: response })
        })
        .with(cors);  // Apply CORS to the route

    println!("Server running on port 12345...");
    warp::serve(route).run(([127, 0, 0, 1], 12345)).await;
}

