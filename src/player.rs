use std::io::{self};

use crate::board;
pub(crate) fn update_play(board: &[[char; 3]; 3], player: char) -> (usize, usize) {
    loop {
        board::print_board(board);
        println!("Jogador {}, escolha sua jogada (1-9): ", player);

        let mut position = String::new();
        io::stdin()
            .read_line(&mut position)
            .expect("Falha ao ler entrada");

        if let Ok(num) = position.trim().parse::<usize>() {
            if let Some((row, column)) = board::index_to_coord(num) {
                if board[row][column] == ' ' {
                    return (row, column);
                } else {
                    println!("Posição ocupada!");
                }
            } else {
                println!("Número fora do intervalo (1-9)!");
            }
        } else {
            println!("Entrada inválida! Digite um número de 1 a 9.");
        }
    }
}