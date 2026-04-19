# 🐹 Whack-a-Mole

A fast-paced arcade game built with [Bevy](https://bevyengine.org/) and Rust.
Click the moles before they disappear, rack up the highest score you can in
**30 seconds**, and try to beat your personal best! 🏆

---

## 🎮 How to Play

1. Launch the game – you'll see a green menu screen with a **Start** button and
   your current high score.
2. Click **Start** to begin a round.
3. Brown mole circles pop up at random positions across the screen.
4. **Left-click** a mole to whack it and earn **1 point**. 🖱️
5. Each mole only sticks around for **1 second** – act fast! ⚡
6. The round lasts **30 seconds**.  When time runs out you're taken back to
   the menu and your high score is updated if you beat it.
7. Click **Start** again to play another round and try to top your score! 🔁

---

## 🚀 How to Run

### Prerequisites

Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- System libraries required by Bevy on Linux:

  ```bash
  sudo apt install libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev pkg-config
  ```

  On macOS and Windows no extra libraries are needed.

### Build & run

```bash
git clone https://github.com/SKutai/whack-a-mole
cd whack-a-mole
cargo run
```

For a faster, optimised build:

```bash
cargo run --release
```

---

## 🗂️ Project Structure

```
src/
├── main.rs        – App entry point; wires all plugins and systems together
├── constants.rs   – Shared magic numbers (timing, sizes, colours)
├── components.rs  – ECS component definitions
├── resources.rs   – ECS resource definitions and the GameState machine
├── menu.rs        – Main-menu UI systems
├── hud.rs         – In-game HUD (timer & score display) systems
└── game.rs        – Mole spawning, lifetime ticking, click detection
```

---

## 🛠️ Built With

- [Bevy 0.15](https://bevyengine.org/) – game engine
- [rand 0.8](https://docs.rs/rand) – random number generation
