# 🚢 Battleship Game (Rust)

![Rust](https://img.shields.io/badge/Rust-1.93%2B-orange?logo=rust)
![Status](https://img.shields.io/badge/status-in%20progress-yellow)
![Frontend](https://img.shields.io/badge/frontend-planned-blue)
![Multiplayer](https://img.shields.io/badge/multiplayer-upcoming-purple)
![License](https://img.shields.io/badge/license-MIT-green)

A modular, extensible **Battleship game engine** built in Rust with a strong focus on clean architecture, testability, and future real-time multiplayer integration.

The long-term goal is to evolve this into a full-stack, real-time multiplayer web application powered by **Axum (WebSockets)** and **React + TypeScript**.


## 🧠 Architecture Overview

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

The project emphasizes domain modeling and minimal shared mutable state.


## ✨ Implemented Features

- Strongly-typed board and coordinate system
- Ship modeling with placement validation
- Turn-based shot resolution engine
- Hit / Miss / Sunk detection logic
- AI opponent with state-aware targeting
- Modular codebase structured for backend extensibility
- Unit-testable core game state


## 🚀 Getting Started

### Run the project

```bash
cargo run
```

### Run tests

```bash
cargo test
```

Tested with **Rust 1.93+**


## 🗺️ Roadmap

### ✅ Phase 1 – Core Engine (Completed)

- [x] Domain modeling (Board, Ship, Coord, GameState)
- [x] Deterministic game state management
- [x] AI opponent
- [x] Turn-based resolution
- [x] Modular structure

---

### 🚧 Phase 2 – Engine Hardening (In Progress)

- [ ] Expanded unit tests
- [ ] Edge-case validation
- [ ] Public API stabilization
- [ ] Improved AI targeting strategy
- [ ] Error handling refinement

---

### 🌐 Phase 3 – Web Backend (Axum)

- [ ] Axum server setup
- [ ] WebSocket integration
- [ ] Game session management
- [ ] Player matchmaking logic
- [ ] State synchronization
- [ ] Serde-based serialization

---

### 🎨 Phase 4 – React Frontend

- [ ] Interactive board rendering
- [ ] Ship placement UI
- [ ] Real-time game updates via WebSocket
- [ ] Visual feedback for hits/misses
- [ ] Responsive layout
- [ ] TypeScript integration

---

### 👥 Phase 5 – Multiplayer

- [ ] Online PvP mode
- [ ] Matchmaking & Room/lobby system
- [ ] Reconnection handling
- [ ] Match lifecycle management



## 🛠 Planned Tech Stack

**Core Engine**
- Rust

**Backend**
- Axum
- Tokio
- WebSockets
- Serde

**Frontend**
- React
- TypeScript


## 🤝 Contributing

Contributions, suggestions, and discussions are welcome.

Areas that are especially helpful:

- Improving test coverage
- Refining API design
- Enhancing AI strategy
- Reviewing architecture decisions
- Preparing the engine for networking integration

If you're interested, feel free to open an issue or start a discussion.


## 🎯 Project Vision

To build a clean, reusable Battleship engine that can serve as a foundation for a full-stack, real-time multiplayer application while demonstrating sound Rust architecture principles.


## 📜 License

This project is licensed under the MIT License. See the LICENSE file for details.
