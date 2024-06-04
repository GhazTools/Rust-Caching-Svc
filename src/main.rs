// EXTERNAL IMPORTS START HERE
use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::FmtSubscriber;
// EXTERNAL IMPORTS END HERE

// LOCAL IMPORTS START HERE
mod requests;
mod wrappers;

use requests::{
    get_variable::get_variable_request, service_status::service_status_request,
    set_variable::set_variable_request,
};
// LOCAL IMPORTS END HERE

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily("./logs", "rust-caching-svc.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .with_timer(ChronoLocal::rfc_3339())
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Set up the middleware
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .into_inner();

    let app: Router = Router::new()
        .route("/", get(service_status_request))
        .route("/get_variable", post(get_variable_request))
        .route("/set_variable", post(set_variable_request))
        .layer(middleware);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
