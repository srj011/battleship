# 🚢 Battleship Game (Full-Stack Multiplayer)

![Rust](https://img.shields.io/badge/backend-Rust-orange?logo=rust)
![SvelteKit](https://img.shields.io/badge/frontend-SvelteKit-black?logo=svelte)
![Status](https://img.shields.io/badge/status-in%20progress-yellow)
![WebSocket](https://img.shields.io/badge/realtime-WebSockets-blue)
![License](https://img.shields.io/badge/license-MIT-green)

A **full-stack real-time Battleship web application** built with:

*  **Rust (Axum)** — backend + game engine
*  **WebSockets** — real-time multiplayer updates
*  **SvelteKit** — modern frontend UI

Designed with a strong focus on **clean architecture, scalability, and real-time gameplay**.


## Architecture Overview

The project is structured into two main layers:

```
/battleship
├── backend/   # Rust + Axum (game engine + API + WS)
└── frontend/  # SvelteKit (UI + real-time client)
```

### Backend

* Game engine (core game logic)
* Session management
* REST API
* WebSocket server (real-time sync)

### Frontend

* Reactive UI (SvelteKit)
* WebSocket-driven state updates
* User interaction layer


## Features

### Core Game Engine

* Strongly-typed board and coordinate system
* Ship placement validation (including adjacency rules)
* Turn-based gameplay
* Hit / Miss / Sunk detection
* AI opponent with smart targeting

### Backend API (Axum)

* Game creation (AI / Multiplayer)
* Join via 6 character alphanumeric game codes
* Snapshot-based game state
* WebSocket real-time updates
* Structured error handling

### Frontend (SvelteKit)

* Clean, minimal UI
* Dual-board layout (player vs opponent)
* Real-time updates via WebSocket
* Fleet placement (manual + random)
* Turn-based interaction controls


## Getting Started

### 1. Run Backend

```bash
cd backend
cargo run
```

Server runs on:

```
http://localhost:3000
```


### 2. Run Frontend

```bash
cd frontend
bun install
bun dev
```

Frontend runs on:

```
http://localhost:5173
```


## API Overview

### REST Endpoints

| Method  | Endpoint                           | Description                    |
| ------- | ---------------------------------- | ------------------------------ |
| GET     | `/api/health`                      | Health check                   |
| POST    | `/api/v1/game`                     | Create game (AI / Multiplayer) |
| POST    | `/api/v1/game/{code}/join`         | Join game                      |
| GET     | `/api/v1/game/{code}`              | Get game state                 |
| GET(WS) | `/api/v1/game/{code}/ws`           | Connect via Websocket          |
| GET     | `/api/v1/random-fleet`             | Generates a random fleet       |
| POST    | `/api/v1/game/{code}/place-fleet`  | Place player fleet             |
| POST    | `/api/v1/game/{code}/fire`         | Fires at a coordinate and returns the resulting turn events. |

> Most endpoints require a `player_token` (returned during game creation/join) to identify the player.

### WebSocket

```
ws://localhost:3000/api/v1/game/{code}/ws?player_token=...
```

#### Incoming messages

* `game_state` → initial snapshot
* `game_update` → real-time updates
* `random_fleet` → generated fleet
* `error` → error messages

#### Outgoing messages

* `fire`
* `place_fleet`
* `random_fleet`


## Game Flow

1. Create or join a game
2. Place fleet (manual or random)
3. Wait for opponent
4. Take turns firing
5. First player to sink all ships wins


## Roadmap

### ✔️ Phase 1 – Core Engine

* [x] Game domain modeling
* [x] Ship placement validation
* [x] Turn handling
* [x] AI opponent

---

### ✔️ Phase 2 – Backend (Axum)

* [x] REST API
* [x] Game sessions
* [x] Snapshot system
* [x] WebSocket integration

---

### 🎨 Phase 3 – Frontend (SvelteKit)

* [x] Project setup
* [x] Routing
* [ ] Game UI polish
* [ ] Fleet placement UX improvements
* [ ] Turn indicators & animations

---

### 👥 Phase 4 – Multiplayer Enhancements

* [ ] Lobby / matchmaking
* [ ] Reconnection support
* [ ] Persistent sessions
* [ ] Spectator mode


## Tech Stack

### Backend

* Rust
* Axum
* Tokio
* WebSockets
* Serde

### Frontend

* SvelteKit (latest)
* TailwindCSS (v4)
* WebSocket API


## Key Design Principles

* Separation of concerns (engine vs API vs UI)
* Real-time first architecture
* Deterministic game logic
* Frontend-agnostic backend
* Scalable session management


## Contributing

Contributions are welcome.

Good areas to contribute:

* UI/UX improvements
* Multiplayer robustness
* AI enhancements
* Testing & edge cases
* Performance optimization


## Project Vision

To build a **production-quality, real-time multiplayer Battleship game** while showcasing:

* Clean Rust backend architecture
* Modern frontend engineering with SvelteKit
* WebSocket-driven real-time systems


## License

MIT License — see `LICENSE` file.
