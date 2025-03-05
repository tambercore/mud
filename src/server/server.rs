use serde::{Deserialize, Serialize};
use warp::Filter;
use crate::english_to_agda;

#[derive(Debug, Deserialize)]
struct SentenceInput {
    sentence: String,
}

#[derive(Debug, Serialize)]
struct AgdaResponse {
    agda: String,
}

pub async fn create_endpoint() {
    let route = warp::path("agda")
        .and(warp::post())
        .and(warp::body::json::<SentenceInput>())
        .map(|input: SentenceInput| {
            let agda_file = english_to_agda(input.sentence).agdaify();
            warp::reply::json(&AgdaResponse { agda: agda_file })
        });

    println!("Server running on port 12345...");
    warp::serve(route).run(([127, 0, 0, 1], 12345)).await;
}