use crate::*;

pub fn pawn_positional_value(row: usize, col: usize, color: Colour) -> i32 {
    let mut value = 0;

    if color == Colour::White {
        value += (7 - row) as i32;
    } else {
        value += row as i32;
    }

    if col > 0 && col < 7 {
        if color == Colour::White {
            value -= 1;
        } else {
            value += 1;
        }
    }

    return value;
}

const KNIGHT_POSITIONAL_VALUES: [[i32; 8]; 8] = [
    [-10, -5, -3, -3, -3, -3, -5, -10],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-3, 0, 5, 5, 5, 5, 0, -3],
    [-3, 5, 5, 5, 5, 5, 5, -3],
    [-3, 0, 5, 5, 5, 5, 0, -3],
    [-3, 5, 5, 5, 5, 5, 5, -3],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-10, -5, -3, -3, -3, -3, -5, -10],
];

pub fn knight_positional_value(row: usize, col: usize, color: Colour) -> i32 {
    if color == Colour::White {
        return KNIGHT_POSITIONAL_VALUES[row][col];
    } else {
        return KNIGHT_POSITIONAL_VALUES[7 - row][7 - col];
    }
}

pub fn bishop_positional_value(row: usize, col: usize, color: Colour, board: Board) -> i32 {
    let mut value = 0;

    if color == Colour::White {
        value += BISHOP_POSITIONAL_VALUES[row][col];
    } else {
        value += BISHOP_POSITIONAL_VALUES[7 - row][7 - col];
    }

    if is_diagonally_blocked(row, col, color, board) {
        value -= 5;
    }

    return value;
}

const BISHOP_POSITIONAL_VALUES: [[i32; 8]; 8] = [
    [-20, -10, -10, -5, -5, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-5, 0, 5, 5, 5, 5, 0, -5],
    [-5, 0, 5, 5, 5, 5, 0, -5],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-20, -10, -10, -5, -5, -10, -10, -20],
];

fn is_diagonally_blocked(row: usize, col: usize, color: Colour, board: Board) -> bool {
    let mut blocked = false;

    let mut i = row;
    let mut j = col;
    while i > 0 && j > 0 {
        i -= 1;
        j -= 1;
    }

    while i < 7 && j < 7 {
        if board.tiles[i][j].piece.piece_type != Type::Empty {
            blocked = true;
            break;
        }
        i += 1;
        j += 1;
    }

    if !blocked {
        return false;
    }

    blocked = false;
    i = row;
    j = col;
    while i > 0 && j < 7 {
        i -= 1;
        j += 1;
    }

    while i < 7 && j > 0 {
        if board.tiles[i][j].piece.piece_type != Type::Empty {
            blocked = true;
            break;
        }
        i += 1;
        j -= 1;
    }

    return blocked;
}

const ROOK_POSITIONAL_VALUES: [[i32; 8]; 8] = [
    [0, 0, 0, 5, 5, 0, 0, 0],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [5, 10, 10, 10, 10, 10, 10, 5],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

pub fn rook_positional_value(row: usize, col: usize, color: Colour, board: Board) -> i32 {
    let mut value = 0;

    if color == Colour::White {
        value += ROOK_POSITIONAL_VALUES[row][col];
    } else {
        value += ROOK_POSITIONAL_VALUES[7 - row][7 - col];
    }

    if is_open_file(row, col, board) {
        value += 5;
    }

    return value;
}

fn is_open_file(row: usize, col: usize, board: Board) -> bool {
    for i in 0..8 {
        if board.tiles[i][col].piece.piece_type != Type::Empty {
            return false;
        }
    }
    return true;
}

const QUEEN_POSITIONAL_VALUES: [[i32; 8]; 8] = [
    [-20, -10, -10, -5, -5, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-5, 0, 5, 5, 5, 5, 0, -5],
    [0, 0, 5, 5, 5, 5, 0, -5],
    [-10, 5, 5, 5, 5, 5, 0, -10],
    [-10, 0, 5, 0, 0, 0, 0, -10],
    [-20, -10, -10, -5, -5, -10, -10, -20],
];

pub fn queen_positional_value(row: usize, col: usize, color: Colour, board: Board) -> i32 {
    let mut value = 0;

    if color == Colour::White {
        value += QUEEN_POSITIONAL_VALUES[row][col];
    } else {
        value += QUEEN_POSITIONAL_VALUES[7 - row][7 - col];
    }

    if is_open_file(row, col, board) {
        value += 5;
    }

    if is_diagonally_blocked(row, col, color, board) {
        value -= 5;
    }

    return value;
}

pub fn passed_pawns_value(board: Board, is_white: bool) -> i32 {
    let mut value = 0;

    for i in 0..8 {
        for j in 0..8 {
            if (board.tiles[i][j].piece.piece_type == Type::Pawn(true)
                || board.tiles[i][j].piece.piece_type == Type::Pawn(false))
                && board.tiles[i][j].piece.colour == Colour::White
            {
                value += passed_pawns_value_helper(i, j, Colour::White, board);
            } else if (board.tiles[i][j].piece.piece_type == Type::Pawn(true)
                || board.tiles[i][j].piece.piece_type == Type::Pawn(false))
                && board.tiles[i][j].piece.colour == Colour::Black
            {
                value += passed_pawns_value_helper(i, j, Colour::Black, board);
            }
        }
    }

    if is_white {
        return value;
    } else {
        return -value;
    }
}

fn passed_pawns_value_helper(row: usize, col: usize, color: Colour, board: Board) -> i32 {
    if !is_passed_pawn(row, col, color, board) {
        return 0;
    }

    let mut value = 0;

    if row == 0 {
        value += 20;
    } else if row == 1 {
        value += 10;
    } else if row == 2 {
        value += 5;
    } else if row == 3 {
        value += 3;
    } else if row == 4 {
        value += 2;
    } else if row == 5 {
        value += 1;
    } else if row == 6 {
        value += 1;
    } else if row == 7 {
        value += 1;
    }

    return value;
}

fn is_passed_pawn(row: usize, col: usize, color: Colour, board: Board) -> bool {
    let opponent_color = if color == Colour::White {
        Colour::Black
    } else {
        Colour::White
    };

    // Check if there are any enemy pawns on the same file
    for i in 0..8 {
        if board.tiles[i][col].piece.piece_type == Type::Pawn(false)
            || board.tiles[i][col].piece.piece_type == Type::Pawn(true)
                && board.tiles[i][col].piece.colour == opponent_color
        {
            return false;
        }
    }

    return true;
}
