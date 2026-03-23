# Battleship Frontend (SvelteKit)

Frontend for the Battleship multiplayer game, built with **SvelteKit** and **TailwindCSS v4**.

## Overview

This frontend connects to the Rust backend via:

- REST API (initial actions)
- WebSockets (real-time game updates)

Provides a clean, responsive UI for:

- Game creation and joining
- Fleet placement
- Turn-based gameplay
- Real-time updates

## Development

### Install dependencies

```bash
bun install
```

### Run dev server

```bash
bun dev
```

App runs on:

```
http://localhost:5173
```

## Backend Requirement

Make sure backend is running:

```
http://localhost:3000
```

Frontend expects the backend API to be available at this address.

## Structure

```
src/
├── routes/          # Pages (home, game)
├── lib/
│   ├── api/         # REST + WebSocket clients
│   ├── stores/      # Global state
│   └── components/  # UI components
```

## Tech Stack

- SvelteKit (latest)
- TailwindCSS v4
- TypeScript
- WebSocket API

## Notes

- Uses URL-based session:

  ```
  /game/{code}?player_token=...
  ```

- WebSocket provides the game state
- UI is designed to be minimal and responsive

## Status

In active development — UI/UX improvements ongoing
