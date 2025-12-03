# Forge

Forge is a batteries-included Rust backend web framework inspired by FastAPI, Laravel, Express, and Django.
It provides:

* High-performance HTTP server supporting all HTTP versions (1.0, 1.1, 2, 3)
* Advanced routing and typed request/response handling
* Middleware system
* CLI tooling for scaffolding, testing, linting, migrations
* Alembic-style models with SQL + NoSQL support
* Auth and security (JWT, sessions, role-based access)
* CORS and protocol policies
* Background job queues, caching, and configuration system
* Real-time support (WebSockets, SSE, pub/sub)
* Logging, observability, and plugin architecture
* Developer-first features (linting, testing, scaffolding)
* Extensible for future AI/analytics modules

## 🎯 Objectives

* Build a full-featured, production-ready Rust web framework
* Ensure developer ergonomics: easy-to-use CLI, scaffolding, and modular API
* Include all major features inspired by FastAPI, Laravel, Express, and Django
* Provide security-first defaults for backend developers
* Support multiple databases: SQL (Postgres, MySQL, SQLite) and NoSQL (MongoDB, Redis)
* Full support for CORS, protocol headers, HTTPS, and all HTTP versions
* Provide extensible architecture for plugins and future modules

## Repo Structure
```
rust-forge/
  forge-core/          # Router, extractors, HTTP handling
  forge-middleware/    # Middleware pipeline
  forge-auth/          # JWT, sessions, role-based access
  forge-db/            # SQL + NoSQL models, Alembic-style migrations
  forge-cli/           # Scaffolding, dev server, lint, test
  forge-queue/         # Background jobs, scheduling
  forge-cache/         # In-memory & Redis caching
  forge-config/        # Environment & config system
  forge-realtime/      # WebSockets, SSE, pub/sub
  forge-security/      # Rate limiting, CORS, headers, CSRF
  forge-utils/         # Logging, tracing, helpers
  examples/            # Sample apps
  docs/
    architecture/
    guides/
    reference/
```
