# Rust TUI Calculator 🦀

A robust, interactive Terminal User Interface (TUI) calculator built with **Rust** and **Ratatui**.

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)
![Ratatui](https://img.shields.io/badge/TUI-Ratatui-green.svg)

## ✨ Features

-   **Test-Driven Core**: Logic core is built using strict TDD (Red-Green-Refactor), ensuring high reliability.
-   **Interactive TUI**:
    -   **Keyboard Support**: Numpad compatible.
    -   **Mouse Support**: Full click support on terminal grid.
-   **Responsive Layout**: Calculator stays centered and perfectly sized regardless of terminal dimensions.
-   **Dynamic Feedback**: UI updates titles based on current operation (e.g., "Adding...", "Result").
-   **Safe**: Handles division by zero and undefined operations gracefully.

## 🚀 Getting Started

### Prerequisites

You need to have **Rust** and **Cargo** installed.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Running the App

Clone the repository and run:

```bash
cargo run
```

### Running Tests

To verify the logic integrity:

```bash
cargo test
```

## 🎮 Usage

| Key | Action |
| --- | --- |
| `0-9` | Input numbers |
| `+ - * /` | Operations |
| `Enter` / `=` | Calculate |
| `c` / `C` | Clear |
| `q` | Quit |
| **Mouse Left** | Click any button on screen |

## 🛠️ Tech Stack

-   **Language**: Rust
-   **UI Framework**: [Ratatui](https://github.com/ratatui-org/ratatui)
-   **Backend/Events**: [Crossterm](https://github.com/crossterm-rs/crossterm)

## 📝 License

This project is open-sourced under the MIT license.

## TODO
- [ ] Styling
- [ ] Handling when content out of display
- [ ] More feature like memory, scentific