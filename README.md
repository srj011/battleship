# 🚢 Battleship Game (Rust)

![Rust](https://img.shields.io/badge/Rust-1.93%2B-orange?logo=rust)
![Status](https://img.shields.io/badge/status-in%20progress-yellow)
![Frontend](https://img.shields.io/badge/frontend-planned-blue)
![Multiplayer](https://img.shields.io/badge/multiplayer-upcoming-purple)
![License](https://img.shields.io/badge/license-MIT-green)

A modular, extensible **Battleship game engine and backend API** written in Rust with a strong focus on clean architecture, testability, and future real-time multiplayer integration.

The project exposes the game engine through a REST API built with **Axum**, and is designed to evolve into a full-stack multiplayer web application powered by **React + TypeScript**.


## Architecture Overview

The engine is designed with strict separation between:

- Core game logic
- Player abstraction
- AI strategy
- Future networking layer
- Future presentation layer

This allows the engine to remain:

- Frontend-agnostic
- Deterministic and testable
- Easily integrable with WebSocket-based multiplayer systems
- Suitable for CLI, Web, or WASM environments


## Implemented Features

### Core Engine

- Strongly-typed board and coordinate system
- Ship placement validation
- Turn-based shot handling
- Hit / Miss / Sunk detection
- AI opponent with state-aware targeting

### Backend API

- Axum-based HTTP server
- Game session management
- Event-based turn timeline
- Snapshot endpoint for full game state
- Health check endpoint
- Input validation and structured API errors
- Unit and integration tests


## Getting Started

### Run the project

```bash
cargo run
```

### Run tests

```bash
cargo test
```

Tested with **Rust 1.93+**


## API Overview

### API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/health` | Health check endpoint to verify that the backend service is running. |
| POST | `/api/v1/game` | Creates a new game session and returns a unique `game_id`. |
| GET | `/api/v1/game/{id}` | Retrieves the current snapshot of the game, including turn, status, and event history. |
| POST | `/api/v1/game/{id}/fire` | Fires at a coordinate for the specified player and returns the resulting turn events. |


## Roadmap

### ✔️ Phase 1 – Core Engine

- [x] Domain modeling (Board, Ship, Coord, GameState)
- [x] Deterministic game state management
- [x] AI opponent
- [x] Turn-based shot handling
- [x] Modular structure

---

### ✔️ Phase 2 – Engine Hardening

- [x] Expanded unit tests
- [x] Edge-case validation
- [x] Improved AI targeting strategy
- [x] Error handling refinement

---

### ✔️ Phase 3 – Web Backend (Axum)

- [x] Axum server setup
- [x] Game session management
- [x] State synchronization
- [x] Serde-based serialization
- [x] Core REST API implementation

---

### 🎨 Phase 4 – React Frontend

- [ ] Interactive board rendering
- [ ] Ship placement UI
- [ ] Real-time game updates via WebSocket
- [ ] Visual feedback for hits/misses
- [ ] Responsive layout

---

### 👥 Phase 5 – Multiplayer

- [ ] Online PvP mode
- [ ] Matchmaking & Room/lobby system
- [ ] Reconnection handling
- [ ] Match lifecycle management



## Planned Tech Stack

**Backend + API**
- Rust
- Axum
- Tokio
- WebSockets
- Serde

**Frontend**
- React
- TypeScript


## Contributing

Contributions, suggestions, and discussions are welcome.

Areas that are especially helpful:

- Improving test coverage
- Refining API design
- Enhancing AI strategy
- Reviewing architecture decisions
- Preparing the engine for networking integration

If you're interested, feel free to open an issue or start a discussion.


## Project Vision

To build a clean, reusable Battleship engine that can serve as a foundation for a full-stack, real-time multiplayer application while demonstrating sound Rust architecture principles.


## License

This project is licensed under the MIT License. See the LICENSE file for details.
