# To-Do App Tech Stack

## Backend
- Language: Rust
- Web Framework: Axum
- Database: PostgreSQL
- ORM: Sea-ORM
- Authentication: jsonwebtoken
- API Documentation: OpenAPI (Swagger) with utoipa

## Frontend
- Rendering: Server-Side Rendering (SSR)
- Templating: Askama
- Styling: 
  - Tailwind CSS
  - DaisyUI (component library on top of Tailwind)

## Development & Tooling
- Asset Bundling: trunk
- Database Migrations: sea-orm-cli
- Testing:
  - Unit Testing: Built-in Rust test framework
  - Integration Testing: Built-in Rust test framework
  - Async Testing: tokio-test
  - Property-Based Testing: proptest
  - Continuous Testing: cargo-watch
- Linting: clippy
- Formatting: rustfmt
- Documentation: rustdoc
- Code Coverage: tarpaulin

## Deployment
- Containerization: Docker
- Orchestration: docker-compose

## CI/CD
- CI/CD Platform: GitHub Actions or GitLab CI
- Automated Testing: cargo test
- Automated Linting: cargo clippy
- Automated Formatting Check: cargo fmt --check

## Monitoring & Logging
- Logging: log crate with env_logger
- Metrics: prometheus
- Tracing: opentelemetry
