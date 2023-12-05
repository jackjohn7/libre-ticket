use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // create SQL connection pool

    // run migrations

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let app = Router::new()
        .route("/", get(root));

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world"
}
