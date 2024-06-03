// EXTERNAL IMPORTS START HERE
use axum::{routing::post, Router};
// EXTERNAL IMPORTS END HERE

// LOCAL IMPORTS START HERE
mod requests;
mod wrappers;

use requests::{get_variable::get_variable_request, set_variable::set_variable_request};
// LOCAL IMPORTS END HERE

#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/get_variable", post(get_variable_request))
        .route("/set_variable", post(set_variable_request));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
