use router::router;
// use tower_http::services::ServeFile;

mod router;

#[tokio::main]
async fn main() {
    //.route_service("/toml", ServeFile::new("Cargo.toml"));
    let listner = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listner, router()).await.unwrap()
}

