🏓 Rust Pong

A modern, fast-paced take on the classic Pong game, built with Rust and the Macroquad game engine. This version features an adaptive AI, a full power-up system, and a slow-motion replay system.

## Features

- **Single Player Mode**: Play against an AI opponent with three difficulty levels (Easy / Medium / Hard).
- **Two Player Mode**: Local multiplayer for competitive fun.
- **Progressive Difficulty**: The ball speeds up after every point, capped at a max speed, and your paddle shrinks a little each time you concede.
- **Bounce Physics**: Bounce angle depends on where the ball hits the paddle.
- **Countdown Serve**: A 3, 2, 1, GO! countdown before each serve; the ball serves toward whoever just scored.
- **Power-up System**: Eight random power-ups spawn periodically at the center line:
  - Fast Ball / Slow Ball — changes the game pace.
  - Big Paddle — increases your paddle size.
  - Shrink Opponent — shrinks the enemy's paddle.
  - Freeze — temporarily stops the opponent's movement.
  - Reverse Controls — flips the opponent's inputs.
  - Shield — protects your goal line once.
  - Multi-Ball — adds an extra ball to the field.
- **Main Menu**: 1 Player / 2 Player / Controls / Quit, plus a dedicated controls screen.
- **Instant Replay**: The last ~5 seconds before every point are replayed in dramatic slow motion (press Space to skip).

## Controls

| Action       | Player 1 (Left) | Player 2 (Right / AI) |
|--------------|:----------------:|:----------------------:|
| Move Up      | W                | Up Arrow               |
| Move Down    | S                | Down Arrow              |
| Select Menu  | Enter            | Enter                   |
| Skip Replay  | Space            | Space                   |
| Back / Quit  | Escape           | Escape                  |

## 📂 Project Structure

- `src/main.rs` — game loop, app state, and screen-specific update/draw logic.
- `src/game_state.rs` — `GameState` / `GameMode` / `Difficulty` / `Winner` / `ScoreEvent` enums.
- `src/paddle.rs` — `Paddle` struct: movement, AI chasing, size effects.
- `src/ball.rs` — `Ball` struct: movement, wall bounce, paddle bounce angle.
- `src/powerup.rs` — `PowerUpKind` enum plus spawning/drawing logic.
- `src/replay.rs` — `Snapshot` struct and the rolling replay buffer.
- `src/score.rs` — scoring and match-end logic.
- `Cargo.toml` — project dependencies and configuration.

## Running it

You need a Rust toolchain ([rustup.rs](https://rustup.rs)) installed.

```bash
cargo run --release
```

## Notes

Two intentional simplifications:
- The extra ball spawned by Multi-Ball never scores or ends a rally by itself; it just bounces around for its duration and then disappears. Only the original ball can score.
- Slow-motion replays only show the original ball and both paddles, not any extra balls that were on screen at the time.

Not yet implemented: animated countdown digits, a settings toggle to disable slow-motion replays.

---

Created by mahdiyar behaein
