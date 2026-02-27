# rust-api-gateway-template

A reusable, production-ready Rust API Gateway template built with **Axum**, **Tokio**, and **code-first OpenAPI**.

---

## ✨ Current Features

- Axum HTTP server
- Tokio async runtime
- Environment-based configuration (`.env`)
- Structured logging via `tracing`
- Health endpoint (`/healthz`)
- Code-first OpenAPI generation with `utoipa`
- Swagger UI at `/docs`
- OpenAPI JSON at `/openapi.json`

---

## 📁 Project Structure

    src/
      main.rs        # Runtime bootstrap
      lib.rs         # App builder + tracing init
      config.rs      # Environment configuration
      api.rs         # API module root
      api/
        router.rs    # Route wiring
        openapi.rs   # OpenAPI definition
        routes.rs    # Route module definitions
        routes/
          health.rs  # Health endpoint

---

## ⚙️ Configuration

Configuration is loaded from environment variables.

A `.env.example` file is provided.

Example `.env`:

    HOST=127.0.0.1
    PORT=3000
    RUST_LOG=info

Copy it locally:

    cp .env.example .env

---

## 🚀 Running the Server

    cargo run

Server will start on:

    http://127.0.0.1:3000

---

## 🔎 Available Endpoints

### Health Check

    GET /healthz

Response:

```json
{
  "status": "ok"
}
