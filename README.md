# clackrs

`clackrs` is a terminal-based typing game written in Rust. It runs in a full-screen terminal UI, presents a scrolling stream of words, highlights correct and incorrect input in real time, and reports typing speed and accuracy at the end of a timed session.

## Features

- Interactive terminal UI built with `crossterm`
- Single-player 60-second typing sessions
- Live countdown timer during gameplay
- Real-time character feedback for correct and incorrect typing
- Rolling word window so the prompt advances as the player types
- End-of-game results for WPM and accuracy
- File-backed word lists stored under `assets/`
- Seeded random word selection using `rand_chacha`

## Demo Flow

When the app starts, it opens an alternate terminal screen and shows a mode selector:

```text
Select Game Mode:

> Single Player
  Multiplayer (coming soon)
  Exit
```

In single-player mode, the player presses Enter to start, types the displayed words for 60 seconds, and then receives a summary:

```text
--- Results ---
Time: 60.00s
WPM: 72.00
Accuracy: 96.50%
```

## Project Structure

```text
.
|-- assets/
|   |-- words_basic.txt
|   |-- words_coding.txt
|   |-- words_numbers.txt
|   `-- words_symbols.txt
|-- src/
|   |-- game.rs
|   |-- main.rs
|   |-- stats.rs
|   |-- ui.rs
|   `-- words.rs
|-- Cargo.lock
|-- Cargo.toml
`-- README.md
```

## How It Works

- `src/main.rs` initializes the terminal, displays the menu, and starts the selected game mode.
- `src/game.rs` runs the game loop, tracks elapsed time, handles keyboard input, advances the visible word window, and calculates WPM and accuracy.
- `src/ui.rs` owns terminal rendering, raw mode setup, menu navigation, color themes, prompt wrapping, timer display, and cleanup.
- `src/words.rs` loads word lists from disk and samples words with a seeded ChaCha RNG.
- `assets/` contains the word sources used by the game. The current default mode uses `words_basic.txt`.

## Requirements

- Rust toolchain with Cargo
- A terminal that supports raw mode and ANSI-style terminal control

Install Rust from [rustup.rs](https://rustup.rs/) if Cargo is not already available.

## Running Locally

Clone the repository and run the project with Cargo:

```bash
cargo run
```

Use the arrow keys to choose a menu option and press Enter to select it. During gameplay, press Esc to exit early.

## Development Commands

```bash
cargo fmt
cargo test
cargo run
```

## Word Lists

The game reads words from plain text files in `assets/`. Each file should contain one word or token per line.

Current bundled lists:

- `words_basic.txt`: large general-purpose typing list
- `words_coding.txt`: programming-related terms
- `words_numbers.txt`: number-focused tokens
- `words_symbols.txt`: symbol-focused tokens

The current game loop defaults to the `basic` list. The word-loading module already supports additional lists that follow the `words_<name>.txt` naming pattern.

## Engineering Notes

This project is intentionally small, but it demonstrates several practical systems-programming concerns:

- Terminal state management with setup and cleanup paths
- Event polling for responsive keyboard input
- Separation between game logic, rendering, and word-list loading
- Deterministic random sampling support through seeded RNGs
- File-based data ingestion from reusable asset lists
- Basic performance metrics derived from user input and elapsed time

## Roadmap

- Add multiplayer mode
- Expose word-list selection in the menu
- Move result calculations into `stats.rs`
- Add unit tests for WPM, accuracy, and word-list loading
- Add configurable session duration
