use crate::*;

pub fn legal_moves(board: &mut Board, from: (usize, usize), is_white: bool) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();

    match board.tiles[from.0][from.1].piece.piece_type {
        Type::Pawn(_) => {
            legal_moves.append(&mut legal_pawn_moves(board, from, is_white));
        }
        Type::Rook => {
            legal_moves.append(&mut legal_straight_moves(board, from, is_white));
        }
        Type::Knight => {
            legal_moves.append(&mut legal_knight_moves(board, from, is_white));
        }
        Type::Bishop => {
            // legal_moves.append(&mut legal_diagonal_moves(board, from, is_white));
        }
        Type::Queen => {
            // legal_moves.append(&mut legal_straight_moves(board, from, is_white));
            // legal_moves.append(&mut legal_diagonal_moves(board, from, is_white));
        }
        Type::King => {
            // legal_moves.append(&mut legal_king_moves(board, from, is_white));
        }
        Type::Empty => {}
    }

    return legal_moves;
}

pub fn move_piece(board: &mut Board, from: (usize, usize), to: (usize, usize)) {
    board.tiles[to.0][to.1].piece = board.tiles[from.0][from.1].piece;

    if board.tiles[from.0][from.1].piece.piece_type == Type::Pawn(false) {
        board.tiles[to.0][to.1].piece.piece_type = Type::Pawn(true);
    }

    board.tiles[from.0][from.1].piece.piece_type = Type::Empty;
}

pub fn legal_straight_moves(
    board: &mut Board,
    from: (usize, usize),
    is_white: bool,
) -> Vec<(usize, usize)> {
    let mut axes: [bool; 4] = [true; 4];
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();
    let mut distance = 0;

    loop {
        distance += 1;
        for (i, axis) in axes.iter_mut().enumerate() {
            if !*axis {
                continue;
            }

            let pos: (i32, i32);
            if i % 2 == 0 {
                pos = (
                    from.0 as i32,
                    from.1 as i32 + distance as i32 * (i as i32 - 1),
                );
            } else {
                pos = (
                    from.0 as i32 + distance as i32 * (i as i32 - 2),
                    from.1 as i32,
                );
            }

            if pos.0 < 0 || pos.0 > 7 || pos.1 < 0 || pos.1 > 7 {
                *axis = false;
                continue;
            }

            let tile = &board.tiles[pos.0 as usize][pos.1 as usize];
            if tile.piece.piece_type == Type::Empty {
                legal_moves.push((pos.0 as usize, pos.1 as usize));
            } else if tile.piece.colour == Colour::White && is_white {
                *axis = false;
            } else if tile.piece.colour == Colour::Black && !is_white {
                *axis = false;
            } else {
                legal_moves.push((pos.0 as usize, pos.1 as usize));
                *axis = false;
            }
        }
    }

    return legal_moves;
}

pub fn legal_pawn_moves(
    board: &mut Board,
    from: (usize, usize),
    is_white: bool,
) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();

    // If pawn hasn't moved, allow 2 tiles forward
    if board.tiles[from.0][from.1].piece.piece_type == Type::Pawn(false) {
        if is_white {
            if board.tiles[from.0 - 1][from.1].piece.piece_type == Type::Empty {
                legal_moves.push((from.0 - 1, from.1));
                if board.tiles[from.0 - 2][from.1].piece.piece_type == Type::Empty {
                    legal_moves.push((from.0 - 2, from.1));
                }
            }
        } else {
            if board.tiles[from.0 + 1][from.1].piece.piece_type == Type::Empty {
                legal_moves.push((from.0 + 1, from.1));
                if board.tiles[from.0 + 2][from.1].piece.piece_type == Type::Empty {
                    legal_moves.push((from.0 + 2, from.1));
                }
            }
        }
        // If pawn has moved, allow 1 tile forward
    } else if board.tiles[from.0][from.1].piece.piece_type == Type::Pawn(true) {
        if is_white {
            if board.tiles[from.0 - 1][from.1].piece.piece_type == Type::Empty {
                legal_moves.push((from.0 - 1, from.1));
            }
        } else {
            if board.tiles[from.0 + 1][from.1].piece.piece_type == Type::Empty {
                legal_moves.push((from.0 + 1, from.1));
            }
        }
    }

    return legal_moves;
}

pub fn legal_knight_moves(
    board: &mut Board,
    from: (usize, usize),
    is_white: bool,
) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();
    let mut possible_moves: Vec<(i32, i32)> = Vec::new();

    // All possible knight moves from a given position
    possible_moves.push(((from.0 as i32 + 2), (from.1 as i32 + 1)));
    possible_moves.push(((from.0 as i32 + 2), (from.1 as i32 - 1)));
    possible_moves.push(((from.0 as i32 - 2), (from.1 as i32 + 1)));
    possible_moves.push(((from.0 as i32 - 2), (from.1 as i32 - 1)));
    possible_moves.push(((from.0 as i32 + 1), (from.1 as i32 + 2)));
    possible_moves.push(((from.0 as i32 + 1), (from.1 as i32 - 2)));
    possible_moves.push(((from.0 as i32 - 1), (from.1 as i32 + 2)));
    possible_moves.push(((from.0 as i32 - 1), (from.1 as i32 - 2)));

    for possible_move in possible_moves.iter() {
        if possible_move.0 < 0 || possible_move.0 > 7 || possible_move.1 < 0 || possible_move.1 > 7
        {
            continue;
        }

        let tile = &board.tiles[possible_move.0 as usize][possible_move.1 as usize];
        if tile.piece.piece_type == Type::Empty {
            legal_moves.push((possible_move.0 as usize, possible_move.1 as usize));
        } else if tile.piece.colour == Colour::White && is_white {
            continue;
        } else if tile.piece.colour == Colour::Black && !is_white {
            continue;
        } else {
            legal_moves.push((possible_move.0 as usize, possible_move.1 as usize));
        }
    }

    return legal_moves;
}
