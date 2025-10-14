# Tic-Tac-Toe (Rust)

A simple **Tic-Tac-Toe (Jogo da Velha)** game written in **Rust**, featuring two modes:
- **Player vs Computer**
- **Player vs Player**

This project was refactored and modularized into multiple files for better organization and readability.

---

## Project Structure
```bash
src/
├── main.rs # Entry point and main menu
├── game.rs # Game flow and modes
├── board.rs # Board display and position handling
├── player.rs # Player input and symbol selection
└── logic.rs # Game rules and win/draw logic
```
---

## Features

- Two game modes (vs Computer or vs Player)  
- Random AI opponent (simple move generator)  
- Clear and modular code structure  
- Input validation and replay option  
- Command-line interface (no external dependencies besides `rand`)

---

## How It Works

- The game board is a `3x3` array of `char`s (`'X'`, `'O'`, or `' '`).
- Players alternate turns selecting positions `1–9`.
- The computer randomly chooses an available spot when playing vs CPU.
- The game checks for a win, draw, or continuation after every move.

---

## Running the Game

### 1. Clone the repository
```bash
git clone https://github.com/<your-username>/tic-tac-toe-rust.git
cd tic-tac-toe-rust
```

2. Run with Cargo
```bash
cargo run
```

Example Gameplay
```bash
=== JOGO DA VELHA ===
1 - Jogar contra o computador
2 - Jogar contra outra pessoa
3 - Sair
Escolha uma opção:
1
Escolha seu símbolo (X ou O): 
X
Você escolheu 'X'. Você começa!

   |   |   
-----------
   |   |   
-----------
   |   |   

Jogador X, escolha sua jogada (1-9): 
```

## Requirements
- Rust 1.70+
- Cargo package manager
- rand crate (automatically included via Cargo.toml)

## Installation of Dependencies
If needed, add the dependency manually:
```bash
cargo add rand
```

## License
This project is open-source and available under the MIT License.

## Author
María Marcato
