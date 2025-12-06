# Project Status

> **Current Version**: 0.1.0 (Alpha)
> **Stability**: Experimental / Active Development

Oxidite is currently in the **Alpha** phase. The core HTTP server, routing, and middleware systems are stable, but APIs may change as we refine the developer experience.

## Feature Completeness

| Component | Status | Notes |
|-----------|--------|-------|
| **Core Server** | 🟢 Stable | High performance, HTTP/1.1 & HTTP/2 |
| **Router** | 🟢 Stable | Path params, query params, extraction |
| **Middleware** | 🟢 Stable | CORS, CSRF, Rate Limit, Compression |
| **CLI** | 🟡 Beta | Scaffolding, Dev Server, Code Gen |
| **Database** | 🟡 Beta | Basic ORM, Migrations (SQL only) |
| **Auth** | 🟡 Beta | JWT, Sessions, OAuth2 |
| **Templates** | 🟢 Stable | Tera-based, Inheritance, Static Files |
| **Realtime** | 🟡 Beta | WebSockets, Pub/Sub |
| **Queues** | 🟡 Beta | Redis-backed background jobs |
| **Email** | 🟡 Beta | SMTP support |
| **Storage** | 🟡 Beta | Local & S3 support |

## Known Issues

- NoSQL database support is currently limited.
- Admin dashboard is not yet implemented.
- Plugin system is in design phase.

## Next Milestones

- **Sprint 1**: CLI Enhancements (Completed)
- **Sprint 2**: Database & ORM Maturity
- **Sprint 3**: Advanced Security & Auth
