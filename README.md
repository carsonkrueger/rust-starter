
# Project Overview

This project is a http server web application built with a focus on clarity, maintainability, and long-term sustainability.

## Tech Stack

- [**Axum**](https://github.com/tokio-rs/axum) — HTTP web framework
- **PostgreSQL** — Relational database
- [**Diesel**](https://diesel.rs) — Type-safe ORM and query builder
- [**Templr**](https://github.com/PizzasBear/templr) — Server-side rendering (SSR) templating (similar to Go’s `templ`)
- [**Datastar**](https://data-star.dev) — Lightweight frontend reactivity

## Architecture

The project follows a **Controller – Service – Repository** architecture:

- **Controllers** handle HTTP concerns only and may access **services** exclusively.
- **Services** encapsulate business logic and orchestrate workflows.
- **Repositories** manage all data persistence and database access.

This strict separation enforces clean boundaries, improves testability, and supports sustainable code evolution.


### Prerequisites

- Rust
- Docker

## Installation

Rename & modify the `example.env` to `.env`

I use cargo watch for live development: `cargo install cargo-watch --locked`

Install Diesel CLI to handle schema and migration management: `cargo install diesel_cli`

`make tw` to watch your tailwind class styles.

in another terminal:

`make app` to watch your rust files. This runs the server locally but runs the postgres instance within a docker container.

# Project Structure
```
├── app/                # Only binary; handles orchestration and app_db
├── DEVOPS/             # Deployment scripts and environment configurations
├── libs/               # Project-agnostic shared Rust crates
│   ├── datastar/       # Utilities and hypermedia templates for Datastar
│   ├── models/         # Core structures for Database and API layers
│   ├── schemas/        # Database models (Diesel-generated)
│   └── templates/      # Framework-agnostic Templr templates
└── README.md           # Project documentation
```
