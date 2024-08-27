use error::ServerError;
use tokio::{net::TcpListener, runtime::Builder as RuntimeBuilder};

mod auth;
mod error;
mod routes;

fn main() -> Result<(), ServerError> {
    RuntimeBuilder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to initialise async runtime")
        .block_on(async { app_root().await })
}

async fn app_root() -> Result<(), ServerError> {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .map_err(|e| ServerError::Io(e))?;

    let router = routes::router().await;

    axum::serve(tcp_listener, router)
        .await
        .map_err(|e| ServerError::Io(e))?;

    Ok(())
}
