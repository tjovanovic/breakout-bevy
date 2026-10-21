# Breakout

A classic Breakout clone written in Rust with the [Bevy](https://bevy.org) game engine (0.19).

Steer the paddle, keep the ball in play, and clear every brick before you run out of lives.

This is a learning project: my first game, and a way to get comfortable with Bevy's ECS (Entity Component System) architecture.

## Gameplay

- Break all the bricks to win.
- You have **3 lives**. Letting the ball fall past the paddle costs one.
- Each round starts with a 3-2-1 countdown.
- Where the ball hits the paddle decides its bounce angle: hit it near an edge to steer the ball sideways.
- When the game ends, press **R** to start over.

## Controls

| Key                       | Action                                  |
| ------------------------- | --------------------------------------- |
| `A` / `Left Arrow`        | Move the paddle left                    |
| `D` / `Right Arrow`       | Move the paddle right                   |
| `R`                       | Restart (on the Game Over / Victory screen) |

## Running

You need a recent Rust toolchain. Bevy 0.19 requires **Rust 1.95 or newer**; install or update it with [rustup](https://rustup.rs):

```sh
rustup update stable
```

Then:

```sh
git clone <this repository>
cd breakout
cargo run
```

The first build compiles Bevy and takes a while. Later builds are much faster. Depending on your platform, Bevy may need extra system libraries; see the [Bevy setup guide](https://bevy.org/learn/quick-start/getting-started/setup/) if the build complains.

The window is a fixed size (1920×1080), so it needs a display at least that large to fit comfortably. The size is set in [`src/constants.rs`](src/constants.rs).

## Tweaking the game

The values that are most useful to change while play-testing live in [`src/config.rs`](src/config.rs): number of lives, brick grid size, and ball size and speed.

## Project structure

Each part of the game is a Bevy plugin in its own module, and `main.rs` only wires them together.

| Module                                  | Responsibility                                                         |
| --------------------------------------- | ---------------------------------------------------------------------- |
| [`main.rs`](src/main.rs)                | Window setup, camera, and the list of plugins                          |
| [`game_sets.rs`](src/game_sets.rs)      | `GameSet` phases that order systems each frame: Input, Movement, Collision, Correction, Outcome |
| [`game_flow.rs`](src/game_flow.rs)      | Game states (`Initialising`, `Countdown`, `Playing`, `GameOver`, `Won`), lives, countdown, end screens, restart |
| [`movement.rs`](src/movement.rs)        | `Velocity` component and the system that applies it                    |
| [`paddle.rs`](src/paddle.rs)            | Paddle spawning, keyboard input, and keeping it inside the window      |
| [`ball.rs`](src/ball.rs)                | Ball spawning with a random launch angle, wall bounces, angle correction |
| [`bricks.rs`](src/bricks.rs)            | Brick grid spawning and despawning                                     |
| [`collision.rs`](src/collision.rs)      | Ball-vs-paddle and ball-vs-brick collisions                            |
| [`config.rs`](src/config.rs)            | Tunable gameplay values                                                |
| [`constants.rs`](src/constants.rs)      | Window size and wall positions                                         |

### How a frame is ordered

Gameplay systems only run while the game is in the `Playing` state, and they run in the phase order defined in `game_sets.rs`:

1. **Input**: read the keyboard and set the paddle's velocity
2. **Movement**: apply every `Velocity` to its `Transform`
3. **Collision**: wall, paddle and brick collisions
4. **Correction**: nudge the ball away from near-vertical paths so it can't get stuck bouncing straight up and down
5. **Outcome**: check for a lost life or a cleared board

## Built with

- [Bevy](https://bevy.org) 0.19
- [rand](https://crates.io/crates/rand) 0.10
