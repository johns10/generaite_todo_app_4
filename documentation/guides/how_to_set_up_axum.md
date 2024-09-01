# Guide: Setting up Axum for Your Project with Encapsulated Service and Separate Routes

- Set up your project structure:
  ```
  your_project/
  ├── Cargo.toml
  ├── src/
  │   ├── main.rs
  │   ├── services/
  │   │   └── axum_web_server.rs
  │   └── routes/
  │       └── mod.rs
  ```

- Add necessary dependencies using `cargo add`:
  ```sh
  cargo add axum
  cargo add tokio --features full
  cargo add tower
  ```

- Create `src/routes/mod.rs` to define your routes:
  ```rust
  use axum::{Router, routing::get};

  async fn hello_world() -> &'static str {
      "Hello, World!"
  }

  pub fn create_routes() -> Router {
      Router::new()
          .route("/", get(hello_world))
      // Add more routes here as needed
  }
  ```

- Implement the encapsulated Axum web application in `src/services/axum_web_server.rs`:
  ```rust
  use axum::Router;
  use std::net::SocketAddr;
  use crate::routes;

  pub struct AxumWebServer;

  impl AxumWebServer {
      pub fn new() -> Self {
          AxumWebServer
      }

      pub async fn run(&self) {
          let app = routes::create_routes();

          let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
          println!("Server listening on http://{}", addr);

          axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
      }
  }
  ```

- Update `src/main.rs` to use the encapsulated Axum web server service:
  ```rust
  mod services;
  mod routes;

  use services::axum_web_server::AxumWebServer;

  #[tokio::main]
  async fn main() {
      let server = AxumWebServer::new();
      server.run().await;
  }
  ```

- Run your Axum application:
  ```
  cargo run
  ```

- Additional notes:
  - Dependencies are now added using `cargo add`, which automatically updates your `Cargo.toml` file.
  - Routes are defined in `src/routes/mod.rs`, allowing for better organization and scalability.
  - The `AxumWebServer` struct in `axum_web_server.rs` uses the routes defined in the `routes` module.
  - `main.rs` remains simple, only creating and running the `AxumWebServer`.
  - To add more routes, extend the `create_routes` function in `src/routes/mod.rs`.
  - For larger applications, consider creating separate files for different route groups within the `routes` directory.
  - You can easily extend this structure to include:
    - Middleware
    - Error handling
    - Database connections
    - Authentication
  - Remember to handle errors appropriately in a production environment.
  - For sharing state between handlers, consider implementing it in the `AxumWebServer` and passing it to the `create_routes` function.