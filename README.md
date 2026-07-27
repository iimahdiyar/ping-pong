# 🏓 Rust Pong

## What is this game?

A modern, fast-paced take on the classic Pong game, built with Rust and the Macroquad game engine. Compared to regular Pong, this version has an AI opponent, a power-up system, and slow-motion replays.

## How is it played?

| Action        | Player 1 (Left) | Player 2 (Right / AI) |
|---------------|:---------------:|:----------------------:|
| Move Up       | W               | Up Arrow                |
| Move Down     | S               | Down Arrow               |
| Confirm Menu  | Enter           | Enter                    |
| Skip Replay   | Space           | Space                    |
| Back / Quit   | Escape          | Escape                   |

- The ball speeds up after every point (up to a set cap).
- The bounce angle off the paddle depends on where the ball hits it.
- A countdown (3, 2, 1, GO!) is shown before every serve.
- Every time you concede a point, your paddle gets a little smaller (down to a minimum).
- After every point, the last few seconds of the rally are replayed in slow motion (press Space to skip).

## What modes does it have?

- **Single Player**: Play against the AI, with three difficulty levels (Easy / Medium / Hard).
- **Two Player**: Head-to-head play on one machine.
- **Main Menu**: Choose between 1 Player / 2 Player / Controls / Quit.

## What items does it have?

Eight power-ups spawn randomly at the center of the field:

- **Fast Ball / Slow Ball** — changes the ball's speed.
- **Big Paddle** — makes your paddle bigger.
- **Shrink Opponent** — shrinks the opponent's paddle.
- **Freeze** — temporarily stops the opponent from moving.
- **Reverse Controls** — reverses the opponent's controls.
- **Shield** — blocks one goal against you.
- **Multi-Ball** — adds an extra ball to the field.

## What's the project structure?

- `src/main.rs` — the main game loop, menu, and rendering of the different screens.
- `src/game_state.rs` — the `GameState` / `GameMode` / `Difficulty` / `Winner` / `ScoreEvent` enums.
- `src/paddle.rs` — the `Paddle` struct: movement, AI, and size changes.
- `src/ball.rs` — the `Ball` struct: movement, wall and paddle collisions.
- `src/powerup.rs` — the `PowerUpKind` enum and the power-up spawn/draw logic.
- `src/replay.rs` — the `Snapshot` struct and the replay buffer.
- `src/score.rs` — scoring and match-end logic.
- `Cargo.toml` — project dependencies and configuration.

### Running it

```bash
cargo run --release
```

---

Created by mahdiyar behaein
