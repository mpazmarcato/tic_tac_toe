pub(crate) fn print_board(board: &[[char; 3]; 3]) {
    println!();
    for i in 0..3 {
        println!(" {} | {} | {} ", board[i][0], board[i][1], board[i][2]);
        if i < 2 {
            println!("-----------");
        }
    }
    println!();
}

pub(crate) fn index_to_coord(index: usize) -> Option<(usize, usize)> {
    match index {
        1 => Some((0, 0)),
        2 => Some((0, 1)),
        3 => Some((0, 2)),
        4 => Some((1, 0)),
        5 => Some((1, 1)),
        6 => Some((1, 2)),
        7 => Some((2, 0)),
        8 => Some((2, 1)),
        9 => Some((2, 2)),
        _ => None,
    }
}