mod game;
mod board;
mod player;
mod logic;

use std::{io, self};
use std::io::{Write};

fn main() {
    let mut board = [[' '; 3]; 3];

    loop {
        println!("=== JOGO DA VELHA ===");
        println!("1 - Jogar contra o computador");
        println!("2 - Jogar contra outra pessoa");
        println!("3 - Sair");
        println!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let mut mode_input = String::new();
        io::stdin()
            .read_line(&mut mode_input)
            .expect("Falha ao ler entrada");
        let mode = mode_input.trim();

        if mode == "1" {
            game::play_vs_computer(&mut board);
            if !game::ask_play_again() {
                break;
            }
        } else if mode == "2" {
            game::play_vs_player(&mut board);
            if !game::ask_play_again() {
                break;
            }
        } else if mode == "3" {
            println!("Saindo do Jogo. Obrigado por jogar!");
            break;
        } else {
            println!("Opção inválida. Encerrando o jogo.");
            break;
        }
    }
}










