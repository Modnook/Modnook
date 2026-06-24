use std::{sync::Arc, time::Duration};

use poem::{EndpointExt, Route, Server, listener::TcpListener};
use poem_openapi::{OpenApiService, payload::PlainText};
use tokio::signal;

use crate::{cache::redis::RedisCache, config::server::ServerOptions};

#[derive(Debug, Clone)]
pub struct AppContext {
    pub database: toasty::Db,

    pub cache: crate::cache::redis::RedisCache,
}

struct Api;

#[poem_openapi::OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }
}

async fn shutdown_signal() {
    let ctrl_c = signal::ctrl_c();

    #[cfg(unix)]
    let terminate = {
        use tokio::signal::unix::{SignalKind, signal};

        let mut sigterm =
            signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");

        async move { sigterm.recv().await }
    };

    #[cfg(windows)]
    let terminate = {
        use tokio::signal::windows;

        let mut sigterm = windows::ctrl_c().expect("failed to install signal handler");

        async move { sigterm.recv().await }
    };

    tokio::select! {
        _ = ctrl_c => println!("ctrl_c received"),
        _ = terminate => println!("terminate received"),
    }
}

pub async fn run_server(options: ServerOptions) -> Result<(), Box<dyn std::error::Error>> {
    let database = toasty::Db::builder()
        .models(toasty::models!(crate::*))
        .connect(&options.database.database_url)
        .await?;

    let cache = RedisCache::new(options.redis)?;

    let state = Arc::new(AppContext { database, cache });

    let service = OpenApiService::new(Api, "Modnook Api", "0.1").server("http://localhost:3000");
    let ui = service.swagger_ui();

    let app = Route::new()
        .nest("/", service)
        .nest("/swagger-ui", ui)
        .data(state);

    let signal = shutdown_signal();

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run_with_graceful_shutdown(app, signal, Some(Duration::from_secs(10)))
        .await?;

    Ok(())
}
