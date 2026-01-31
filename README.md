# Volt

**Real-time messaging, built from the ground up.**

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-4169E1?style=flat&logo=postgresql&logoColor=white)
![WebSocket](https://img.shields.io/badge/WebSocket-010101?style=flat&logo=websocket&logoColor=white)
![React](https://img.shields.io/badge/React-61DAFB?style=flat&logo=react&logoColor=black)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=flat&logo=typescript&logoColor=white)
![TailwindCSS](https://img.shields.io/badge/TailwindCSS-06B6D4?style=flat&logo=tailwindcss&logoColor=white)

Volt is a full-stack real-time chat application with a React frontend and a Rust backend. It supports direct and group conversations over WebSockets, with Clerk-powered authentication and a clean, themeable UI.

## Features

- **Real-time messaging** — Instant delivery via WebSockets with per-conversation fan-out
- **Direct & group conversations** — One-on-one chats and multi-participant groups
- **Clerk authentication** — Secure sign-in with session management and middleware verification
- **Dark / light theme** — Toggleable UI theme with TailwindCSS
- **Type-safe end-to-end** — TypeScript on the client, compile-time checked SQL on the server

## Tech Stack

| Frontend    | Backend    |
| ----------- | ---------- |
| React       | Rust       |
| TypeScript  | Axum       |
| TailwindCSS | PostgreSQL |
| shadcn-ui   |            |

**Real-time:** WebSockets via Axum + Tokio broadcast channels

## Architecture

- **Repository pattern** — Trait-based abstractions with database and in-memory implementations
- **Dependency injection** — Axum state extractors for clean handler signatures
- **WebSocket management** — Per-conversation broadcast channels with connection fan-out
- **Compile-time SQL** — SQLx macros verify queries against the database schema at build time (WIP)
- **Custom error types** — Automatic HTTP status code mapping via `IntoResponse` implementations
