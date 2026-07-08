# System Architecture

```text
Browser
      │
      ▼
Askama Templates
      │
      ▼
Axum Router
      │
      ▼
Handlers
      │
      ▼
Services
      │
      ▼
Repositories
      │
      ▼
SQLx
      │
      ▼
PostgreSQL
```

## Layers

### Routes

Maps URLs to handlers.

### Handlers

Process HTTP requests.

### Services

Contains business logic.

### Repository

Handles database access.

### Database

Stores application data.

## Principles

- Separation of concerns
- Dependency injection
- Repository pattern
- Modular design
- Error propagation using Result
