use std::{io, thread, time::Duration};
use std::io::Write;
use rand::Rng;

use crate::{board, player, logic};

// =========================
// ==== MODE 0: VS CPU =====
// =========================
pub(crate) fn play_vs_computer(board: &mut [[char; 3]; 3]) {
    println!("Escolha seu símbolo (X ou O): ");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Falha ao ler entrada");

    let player = match choice.trim().to_uppercase().as_str() {
        "O" => 'O',
        _ => 'X',
    };

    let opponent = if player == 'X' { 'O' } else { 'X' };
    let mut current_player;

    if player == 'X' {
        println!("Você escolheu 'X'. Você começa!");
        current_player = player;
    } else {
        println!("Você escolheu 'O'. O oponente (X) começa!");
        current_player = opponent;
    }

    loop {
        if current_player == player {
            let (row, column) = player::update_play(board, current_player);
            board[row][column] = current_player;
        } else {
            println!("Oponente pensando...");
            thread::sleep(Duration::from_secs(1));
            let (row, column) = random_play(board);
            board[row][column] = current_player;
        }

        board::print_board(board);

        if let Some(result) = logic::check_play(board) {
            match result {
                'X' | 'O' => {
                    if result == player {
                        println!("Você venceu!");
                    } else {
                        println!("O oponente venceu!");
                    }
                }
                'E' => println!("Empate!"),
                _ => (),
            }
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}

// ===========================
// ==== MODE 1: VS PLAYER ====
// ===========================
pub(crate) fn play_vs_player(board: &mut [[char; 3]; 3]) {
    println!("Jogador 0 será 'X'");
    println!("Jogador 1 será 'O'");

    let mut current_player = 'X';

    loop {
        let (row, column) = player::update_play(board, current_player);
        board[row][column] = current_player;

        board::print_board(board);

        if let Some(result) = logic::check_play(board) {
            match result {
                'X' => println!("Jogador 0 venceu!"),
                'O' => println!("Jogador 1 venceu!"),
                'E' => println!("Empate!"),
                _ => (),
            }
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}

pub(crate) fn ask_play_again() -> bool {
    loop {
        print!("Deseja jogar novamente? ");
        io::stdout().flush().unwrap();

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();

        match answer.trim().to_lowercase().as_str() {
            "s" => return true,
            "n" => {
                println!("Encerrando o jogo. Até a próxima!");
                return false
            }
            _ => println!("Resposta inválida. Digite 's' para jogar novamente ou 'n' para sair do jogo.")
        }
    }
}

fn random_play(board: &[[char; 3]; 3]) -> (usize, usize) {
    let mut rng = rand::rng();
    loop {
        let index = rng.random_range(1..=9);
        if let Some((row, column)) = board::index_to_coord(index) {
            if board[row][column] == ' ' {
                println!("Oponente jogou na posição {}", index);
                return (row, column);
            }
        }
    }
}