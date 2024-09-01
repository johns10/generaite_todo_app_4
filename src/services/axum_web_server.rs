use std::net::SocketAddr;
use crate::routes;
use crate::config::AppConfig;
use std::sync::Arc;

pub struct AxumWebServer {
    config: Arc<AppConfig>,
}

impl AxumWebServer {
    pub fn new(config: Arc<AppConfig>) -> Self {
        AxumWebServer { config }
    }

    pub async fn run(&self) {
        let app = routes::create_routes();

        let addr = SocketAddr::from((self.config.server.host.parse::<std::net::IpAddr>().unwrap(), self.config.server.port));
        tracing::info!("Server listening on http://{}", addr);

        axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
    }
}
