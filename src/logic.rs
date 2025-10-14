pub(crate) fn check_play(board: &[[char; 3]; 3]) -> Option<char> {
    for i in 0..3 {
        if board[i][0] != ' ' && board[i][0] == board[i][1] && board[i][1] == board[i][2] {
            return Some(board[i][0]);
        }
        if board[0][i] != ' ' && board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            return Some(board[0][i]);
        }
    }

    if board[0][0] != ' ' && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return Some(board[0][0]);
    }

    if board[0][2] != ' ' && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return Some(board[0][2]);
    }

    if board.iter().all(|row| row.iter().all(|&c| c != ' ')) {
        return Some('E');
    }

    None
}