use tokio::{net::TcpListener, runtime::Builder as RuntimeBuilder};

mod auth;
mod routes;

fn main() {
    RuntimeBuilder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to initialise async runtime")
        .block_on(async { app_root().await })
}

async fn app_root() {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let router = routes::router();

    axum::serve(tcp_listener, router.await).await.unwrap();
}
