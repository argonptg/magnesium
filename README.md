# Magnesium

**A desktop telemetry companion for EA SPORTS F1 games.**

Magnesium is an open-source desktop application built to turn the telemetry data from EA SPORTS F1 games into a clean, accessible dashboard. It focuses on the information you need during a session, with room for more detailed views as the project grows.

> [!IMPORTANT]
> **Work in progress.** Magnesium is in early development, starting with **F1 25**. The interface and telemetry pipeline are being built, and the app is not yet ready for general use.

## Overview

F1 games can broadcast live session information through a UDP telemetry feed. Magnesium listens for those packets, decodes them in Rust, and sends the resulting data to its Svelte frontend.

The goal is a **one-page overview** for the most useful information at a glance, with dedicated pages for deeper analysis in the future.

### Current progress

- **F1 25 UDP protocol:** Rust data structures for the 16 packet types defined in the 2025 specification.
- **Telemetry receiver:** a background UDP listener that validates incoming packet headers and sizes before decoding and forwarding packets through Tauri events.
- **Reactive frontend:** Svelte 5 state for incoming lap, session, and participant data.
- **Dashboard foundation:** cards for lap number, position, lap times, and other session information. Some values are still placeholders.
- **Connection feedback:** events for incoming telemetry and receiver timeouts.

Not every decoded packet is consumed by the UI yet, and some protocol handling remains under development.

## Tech stack

| Layer | Technology |
| --- | --- |
| Desktop application | [Tauri 2](https://tauri.app/) |
| Telemetry backend | [Rust](https://www.rust-lang.org/) |
| Frontend | [Svelte 5](https://svelte.dev/) / [SvelteKit](https://svelte.dev/docs/kit/introduction) |
| Language | [TypeScript](https://www.typescriptlang.org/) |
| Styling | [Sass / SCSS](https://sass-lang.com/) |
| Build tooling | [Vite](https://vite.dev/) and [pnpm](https://pnpm.io/) |

Magnesium targets **Windows and Linux**, although platform support is still being developed and verified.

## Getting started

There are no stable builds available yet. To try the current development version, build it from source.

### Prerequisites

- Node.js and pnpm
- Rust and Cargo
- The system dependencies required by Tauri 2 for your operating system

See the [Tauri prerequisites guide](https://v2.tauri.app/start/prerequisites/) for platform-specific setup.

### Run locally

```bash
git clone https://github.com/argonptg/magnesium.git
cd magnesium
pnpm install
pnpm tauri dev
```

To create a local production bundle:

```bash
pnpm tauri build
```

These commands build the current development code; a successful build does not imply a stable release.

### Configure F1 25 telemetry

In **F1 25**, enable UDP telemetry and configure the game to send data to Magnesium:

| Setting | Value |
| --- | --- |
| UDP Telemetry | On |
| UDP IP Address | `127.0.0.1` (when running on the same PC) |
| UDP Port | `20777` |
| UDP Format | `2025` |

The current application listens on `0.0.0.0:20777`, meaning it binds to all local network interfaces. If the game is running on another device, configure its destination IP to point to the computer running Magnesium, and make sure local firewall rules allow incoming UDP packets on that port.

> Telemetry configuration is currently fixed in the application code; a settings screen is not available yet.

## Project structure

```text
magnesium/
├── src/
│   ├── lib/
│   │   ├── components/      # Dashboard and layout components
│   │   ├── services/        # Tauri event listeners
│   │   ├── stores/          # Reactive telemetry state
│   │   ├── types/           # TypeScript packet definitions
│   │   └── styles/          # Global SCSS
│   └── routes/              # SvelteKit pages
├── src-tauri/
│   └── src/
│       ├── udp/
│       │   ├── connection.rs # UDP receiver and packet dispatch
│       │   └── structs/     # F1 25 packet layouts and parsing
│       └── utils/           # Backend utilities
└── static/                   # Static assets
```

### How it works

```text
F1 25
  │ UDP telemetry (2025 format)
  ▼
Rust UDP listener
  │ Validate header → decode packet
  ▼
Tauri events
  │ Serialized telemetry data
  ▼
Svelte 5 reactive state
  │
  ▼
Magnesium dashboard
```

## Roadmap

Magnesium is evolving incrementally. Current areas of focus include:

- Expanding the dashboard beyond its initial telemetry cards.
- Connecting more packet types to the frontend.
- Developing more detailed session and telemetry views.
- Improving connection management and configuration.
- Preparing reliable builds for supported desktop platforms.

These are development goals, not a list of completed features or promised release dates.

## License

Magnesium is licensed under the **Mozilla Public License 2.0 (MPL-2.0)**. See [LICENSE](LICENSE) for the full text.

---

**Magnesium is an independent project and is not affiliated with or endorsed by Electronic Arts, EA SPORTS, or Formula 1.**
