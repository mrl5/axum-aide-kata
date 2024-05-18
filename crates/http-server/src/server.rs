use crate::tracing::MyOnResponse;
use axum::{serve, Extension, Router};
use common::db::DB;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub async fn run_server(
    address: SocketAddr,
    router: Router,
    docs_path: Option<&str>,
    db: DB,
) -> anyhow::Result<()> {
    let middleware_stack = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .on_request(())
                .on_response(MyOnResponse {}),
        )
        .layer(Extension(db.clone()));

    let app = router.layer(middleware_stack);

    tracing::info!("Starting server ...");
    let listener = TcpListener::bind(&address).await?;
    let server = serve(listener, app.into_make_service());

    println!("Server running at http://{}", address);
    if let Some(dp) = docs_path {
        println!("Docs at http://{}{}", address, dp);
    }
    server.await?;
    Ok(())
}
