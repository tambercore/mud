use serde::{Deserialize, Serialize};
use warp::{http::Method, Filter};
use crate::english_to_agda;
use warp::reply::Reply;
use warp::cors;



/// Function to create an endpoint that accepts POST requests with a JSON body and returns a JSON response.
/// It enables CORS, processes the input, converts it using `english_to_agda`, and responds with the Agda file.
pub async fn create_endpoint() {
    /* Enable CORS */
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::POST, Method::GET])
        .allow_headers(vec!["Content-Type"]);

    let route = warp::path("agda")
        .and(warp::post())
        .and(warp::body::json::<SentenceInput>())
        .map(|input: SentenceInput| {
            let agda_file = english_to_agda(input.knowledge, input.conclusions).agdaify();
            warp::reply::json(&AgdaResponse { agda: agda_file })
        })
        .with(cors);  // Apply CORS to the route

    println!("Server running on port 12345...");
    warp::serve(route).run(([127, 0, 0, 1], 12345)).await;
}



/// Structure to define the input data received in the request.
/// This structure expects a vector of knowledge strings and a vector of conclusions.
#[derive(Debug, Deserialize)]
struct SentenceInput {
    knowledge: Vec<String>,
    conclusions: Vec<String>
}



/// Structure to define the response that will be returned to the client.
/// The response contains a single string field for the Agda file.
#[derive(Debug, Serialize)]
struct AgdaResponse {
    agda: String,
}
