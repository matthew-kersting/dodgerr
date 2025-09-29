# Dodgerr

**Dodgerr** is a simple 2D game made with [Bevy](https://bevyengine.org/) where you control a small player and dodge incoming projectiles. Enemies spawn at the edges of the screen and shoot at you — your goal is to survive as long as you can!

This project is a minimal experiment in using the Bevy game engine and practicing ECS (Entity-Component-System) architecture in Rust.

---

## 🕹️ Gameplay

- Use the **arrow keys** to move your player.
- Projectiles are fired periodically by red enemies positioned on each side of the screen.
- If a projectile hits you, the game ends.
- The longer you survive, the higher your **score** and **level**.
- As levels increase:
  - Enemies and projectiles move faster.
  - Projectiles become more dangerous!

---

## 📦 Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- Cargo

---

## 🚀 Running the Game

```bash
git clone https://github.com/yourusername/dodgerr.git
cd dodgerr
cargo run
````

---

## 🧱 Built With

* [Bevy](https://crates.io/crates/bevy) (v0.13) – Data-driven game engine in Rust.
* [Rand](https://crates.io/crates/rand) – Random number generation.

---

## 📁 Project Structure

```bash
.
├── src
│   └── main.rs       # Game logic and systems
├── assets
│   └── fonts/
│       └── FiraSans-Bold.ttf
├── Cargo.toml
└── README.md
```

> ⚠️ Note: Make sure the font file exists at `assets/fonts/FiraSans-Bold.ttf`, or replace it with another font of your choice.

---
