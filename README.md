# 2D Particle Simulator
> A real-time, GPU-accelerated particle simulator built in Rust with [Macroquad](https://github.com/not-fl3/macroquad).
---

## Table of Contents
- [Overview](<README#✨ Overview>)
- [Tech Stack](<README#🧩 Tech Stack>)
- [Installation](<README#🛠️ Installation>)

## ✨ Overview

This project is an experimental physics-based particle simulator written entirely in Rust.
It leverages **Macroquad** for cross-platform rendering and aims to provide a clean, extensible codebase for experimenting with particle systems, gravity, collisions, and fluid-like behavior.

The long-term goal is to **port it to the web** via `wasm32` for interactive browser-based simulations while keeping full control over the UI layer (HTML/CSS-based).

---

## 🧩 Tech Stack

| Component | Technology |
|------------|-------------|
| Language | Rust |
| Rendering | Macroquad |
| Physics | The goal of the project |
| Target Platforms | Desktop (native), Web (WASM planned) |

---

## 🛠️ Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)

### Run Locally

```bash
# Clone the repository
git clone https://github.com/vesgt/particle-simulator.git
cd particle-simulator

# Run the simulation
cargo run
```
