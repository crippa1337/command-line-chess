use crate::*;

pub fn is_in_check(board: Board, is_white: bool) -> bool {
    // Get all opponent's moves
    let moves = get_all_moves(board, !is_white);

    // Get the king's position
    let king_pos = if is_white {
        board.kingpos_w
    } else {
        board.kingpos_b
    };

    // Check if any of the opponent's moves are the king's position
    for m in moves {
        for to in m.1 {
            if to == king_pos {
                return true;
            }
        }
    }

    return false;
}

pub fn legal_moves(board: Board, from: (usize, usize), is_white: bool) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();

    match board.tiles[from.0][from.1].piece.piece_type {
        Type::Pawn(_) => {
            legal_moves.append(&mut legal_pawn_moves(board, from, is_white));
        }
        Type::Rook(_) => {
            legal_moves.append(&mut legal_straight_moves(board, from, is_white));
        }
        Type::Knight => {
            legal_moves.append(&mut legal_knight_moves(board, from, is_white));
        }
        Type::Bishop => {
            legal_moves.append(&mut legal_diagonal_moves(board, from, is_white));
        }
        Type::Queen => {
            legal_moves.append(&mut legal_straight_moves(board, from, is_white));
            legal_moves.append(&mut legal_diagonal_moves(board, from, is_white));
        }
        Type::King(_) => {
            legal_moves.append(&mut legal_king_moves(board, from, is_white));
        }
        Type::Empty => unreachable!(),
    }

    return legal_moves;
}

pub fn check_for_mates(board: Board) -> Option<Colour> {
    // Check if any kings are in check, if it is, create a copy of the board and test all the possible moves from all possible pieces
    // If none of the moves will remove the king from check, return winner
    // Check if white king is in checkmate
    if is_in_check(board, true) {
        for i in 0..8 {
            for j in 0..8 {
                if board.tiles[i][j].piece.piece_type == Type::Empty {
                    continue;
                }

                if board.tiles[i][j].piece.colour == Colour::White {
                    let moves = legal_moves(board, (i, j), true);
                    for m in moves {
                        let mut test_board = board.clone();
                        match move_piece(&mut test_board, (i, j), m, true) {
                            Ok(_) => {
                                if !is_in_check(test_board, true) {
                                    return None;
                                }
                            }
                            Err(_) => (),
                        }
                    }
                }
            }
        }

        return Some(Colour::Black);
    }

    // Check if black king is in checkmate
    if is_in_check(board, false) {
        for i in 0..8 {
            for j in 0..8 {
                if board.tiles[i][j].piece.piece_type == Type::Empty {
                    continue;
                }

                if board.tiles[i][j].piece.colour == Colour::Black {
                    let moves = legal_moves(board, (i, j), false);
                    for m in moves {
                        let mut test_board = board.clone();
                        match move_piece(&mut test_board, (i, j), m, false) {
                            Ok(_) => {
                                if !is_in_check(test_board, false) {
                                    return None;
                                }
                            }
                            Err(_) => (),
                        }
                    }
                }
            }
        }

        return Some(Colour::White);
    }

    return None;
}

pub fn move_piece(
    board: &mut Board,
    from: (usize, usize),
    to: (usize, usize),
    is_white: bool,
) -> Result<(), Error> {
    // Make a copy of the board to test the move on
    let mut test_board = board.clone();

    // Make the move on the test board
    match test_board.tiles[from.0][from.1].piece.piece_type {
        Type::King(false) => {
            test_board.tiles[from.0][from.1].piece.piece_type = Type::King(true);
            if is_white {
                test_board.kingpos_w = to;
            } else {
                test_board.kingpos_b = to;
            }
        }
        Type::King(_) => {
            if is_white {
                test_board.kingpos_w = to;
            } else {
                test_board.kingpos_b = to;
            }
        }

        Type::Pawn(false) => {
            test_board.tiles[from.0][from.1].piece.piece_type = Type::Pawn(true);
        }
        Type::Rook(false) => {
            test_board.tiles[from.0][from.1].piece.piece_type = Type::Rook(true);
        }
        _ => (),
    }

    // Castling
    if board.tiles[from.0][from.1].piece.piece_type == Type::King(false) {
        if to.1 == 1 {
            test_board.tiles[from.0][0].piece.piece_type = Type::Empty;
            test_board.tiles[from.0][3].piece.piece_type = Type::Rook(true);
        } else if to.1 == 6 {
            test_board.tiles[from.0][7].piece.piece_type = Type::Empty;
            test_board.tiles[from.0][5].piece.piece_type = Type::Rook(true);
        }
    }
    test_board.tiles[to.0][to.1].piece = test_board.tiles[from.0][from.1].piece;
    test_board.tiles[from.0][from.1].piece.piece_type = Type::Empty;

    // Check if the king is in check after the move
    if is_in_check(test_board, is_white) {
        return Err(Error::Check);
    }

    // Actually do the move if king isn't in check
    // Castling
    if board.tiles[from.0][from.1].piece.piece_type == Type::King(false) {
        if to.1 == 2 {
            board.tiles[from.0][0].piece.piece_type = Type::Empty;
            board.tiles[from.0][3].piece.piece_type = Type::Rook(true);
        } else if to.1 == 6 {
            board.tiles[from.0][7].piece.piece_type = Type::Empty;
            board.tiles[from.0][5].piece.piece_type = Type::Rook(true);
        }
    }

    match board.tiles[from.0][from.1].piece.piece_type {
        Type::King(false) => {
            board.tiles[from.0][from.1].piece.piece_type = Type::King(true);
            if is_white {
                board.kingpos_w = to;
            } else {
                board.kingpos_b = to;
            }
        }
        Type::King(_) => {
            if is_white {
                board.kingpos_w = to;
            } else {
                board.kingpos_b = to;
            }
        }

        Type::Pawn(false) => {
            board.tiles[from.0][from.1].piece.piece_type = Type::Pawn(true);
        }
        Type::Rook(false) => {
            board.tiles[from.0][from.1].piece.piece_type = Type::Rook(true);
        }
        _ => (),
    }

    board.tiles[to.0][to.1].piece = board.tiles[from.0][from.1].piece;
    board.tiles[from.0][from.1].piece.piece_type = Type::Empty;

    // Pawn swap at edge
    if is_white {
        if board.tiles[to.0][to.1].piece.piece_type == Type::Pawn(true) {
            if to.0 == 0 || to.0 == 7 {
                board.tiles[to.0][to.1].piece.piece_type = Type::Queen;
            }
        }
    } else {
        if board.tiles[to.0][to.1].piece.piece_type == Type::Pawn(true) {
            if to.0 == 0 || to.0 == 7 {
                board.tiles[to.0][to.1].piece.piece_type = Type::Queen;
            }
        }
    }

    return Ok(());
}

pub fn legal_pawn_moves(board: Board, from: (usize, usize), is_white: bool) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();
    let mut possible_moves: Vec<(usize, usize)> = Vec::new();
    let mut possible_captures: Vec<(usize, usize)> = Vec::new();

    // Forward moves
    if board.tiles[from.0][from.1].piece.piece_type == Type::Pawn(false) {
        if is_white {
            possible_moves.push((from.0.wrapping_sub(1), from.1));
            // Only push the second move if the first move is legal
            if board.tiles[from.0.wrapping_sub(1)][from.1].piece.piece_type == Type::Empty {
                possible_moves.push((from.0.wrapping_sub(2), from.1));
            }
        } else {
            possible_moves.push((from.0.wrapping_add(1), from.1));
            if board.tiles[from.0.wrapping_add(1)][from.1].piece.piece_type == Type::Empty {
                possible_moves.push((from.0.wrapping_add(2), from.1));
            }
        }
    } else if board.tiles[from.0][from.1].piece.piece_type == Type::Pawn(true) {
        if is_white {
            possible_moves.push((from.0.wrapping_sub(1), from.1));
        } else {
            possible_moves.push((from.0.wrapping_add(1), from.1));
        }
    }

    // Diagonal attacks
    if is_white {
        possible_captures.push((from.0.wrapping_sub(1), from.1.wrapping_sub(1)));
        possible_captures.push((from.0.wrapping_sub(1), from.1.wrapping_add(1)));
    } else {
        possible_captures.push((from.0.wrapping_add(1), from.1.wrapping_sub(1)));
        possible_captures.push((from.0.wrapping_add(1), from.1.wrapping_add(1)));
    }

    for m in possible_moves {
        // if any of the moves are wrapping, they are illegal
        if m.0 > 7 || m.1 > 7 {
            continue;
        }

        if board.tiles[m.0][m.1].piece.piece_type == Type::Empty {
            legal_moves.push(m);
        }
    }

    for m in possible_captures {
        if m.0 > 7 || m.1 > 7 {
            continue;
        }

        if board.tiles[m.0][m.1].piece.piece_type != Type::Empty
            && board.tiles[m.0][m.1].piece.colour != board.tiles[from.0][from.1].piece.colour
        {
            legal_moves.push(m);
        }
    }

    return legal_moves;
}

pub fn pawn_swap(mut board: Board, to: (usize, usize), is_white: bool) {
    let green = RGB(50, 150, 50);
    loop {
        println!(
            "{} What do you want to convert your pawn at {} to?\n{}",
            green.paint(">>>"),
            green.paint(reverse_match_input(to)),
            green.paint("(1) Queen\n(2) Knight\n(3) Bishop\n(4) Rook")
        );

        print!("{} ", ">>>");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_lowercase();

        match input.as_str() {
            "1" => {
                board.tiles[to.0][to.1].piece.piece_type = Type::Queen;
                break;
            }
            "2" => {
                board.tiles[to.0][to.1].piece.piece_type = Type::Knight;
                break;
            }
            "3" => {
                board.tiles[to.0][to.1].piece.piece_type = Type::Bishop;
                break;
            }
            "4" => {
                board.tiles[to.0][to.1].piece.piece_type = Type::Rook(true);
                break;
            }
            _ => {
                clear_draw(board, is_white);
                input_error(Error::OutOfBounds);
                continue;
            }
        }
    }
}

pub fn legal_straight_moves(
    board: Board,
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

        if axes.iter().all(|&x| !x) {
            break;
        }
    }

    return legal_moves;
}

pub fn legal_knight_moves(
    board: Board,
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

pub fn legal_diagonal_moves(
    board: Board,
    from: (usize, usize),
    is_white: bool,
) -> Vec<(usize, usize)> {
    let mut diagonals: [bool; 4] = [true; 4];
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();
    let mut distance = 0;

    loop {
        distance += 1;
        for (i, diagonal) in diagonals.iter_mut().enumerate() {
            if !*diagonal {
                continue;
            }

            let pos: (i32, i32);
            // First diagonal, incrementing TOP RIGHT
            if i == 0 {
                pos = (
                    from.0 as i32 + distance as i32,
                    from.1 as i32 + distance as i32,
                );
            // Second diagonal, incrementing TOP LEFT
            } else if i == 1 {
                pos = (
                    from.0 as i32 + distance as i32,
                    from.1 as i32 - distance as i32,
                );
            // Third diagonal, incrementing DOWN RIGHT
            } else if i == 2 {
                pos = (
                    from.0 as i32 - distance as i32,
                    from.1 as i32 + distance as i32,
                );
            // Fourth diagonal, incrementing DOWN LEFT
            } else {
                pos = (
                    from.0 as i32 - distance as i32,
                    from.1 as i32 - distance as i32,
                );
            }

            if pos.0 < 0 || pos.0 > 7 || pos.1 < 0 || pos.1 > 7 {
                *diagonal = false;
                continue;
            }

            let tile = &board.tiles[pos.0 as usize][pos.1 as usize];
            if tile.piece.piece_type == Type::Empty {
                legal_moves.push((pos.0 as usize, pos.1 as usize));
            } else if tile.piece.colour == Colour::White && is_white {
                *diagonal = false;
            } else if tile.piece.colour == Colour::Black && !is_white {
                *diagonal = false;
            } else {
                legal_moves.push((pos.0 as usize, pos.1 as usize));
                *diagonal = false;
            }
        }

        if diagonals.iter().all(|&x| !x) {
            break;
        }
    }

    return legal_moves;
}

pub fn legal_king_moves(board: Board, from: (usize, usize), is_white: bool) -> Vec<(usize, usize)> {
    let mut legal_move_list: Vec<(usize, usize)> = Vec::new();
    let mut possible_moves: Vec<(i32, i32)> = Vec::new();

    // All possible king moves from a given position
    possible_moves.push(((from.0 as i32 + 1), (from.1 as i32 + 1)));
    possible_moves.push(((from.0 as i32 + 1), (from.1 as i32 - 1)));
    possible_moves.push(((from.0 as i32 - 1), (from.1 as i32 + 1)));
    possible_moves.push(((from.0 as i32 - 1), (from.1 as i32 - 1)));
    possible_moves.push(((from.0 as i32 + 1), (from.1 as i32)));
    possible_moves.push(((from.0 as i32 - 1), (from.1 as i32)));
    possible_moves.push(((from.0 as i32), (from.1 as i32 + 1)));
    possible_moves.push(((from.0 as i32), (from.1 as i32 - 1)));

    for possible_move in possible_moves.iter() {
        if possible_move.0 < 0 || possible_move.0 > 7 || possible_move.1 < 0 || possible_move.1 > 7
        {
            continue;
        }

        let tile = &board.tiles[possible_move.0 as usize][possible_move.1 as usize];
        if tile.piece.piece_type == Type::Empty {
            legal_move_list.push((possible_move.0 as usize, possible_move.1 as usize));
        } else if tile.piece.colour == Colour::White && is_white {
            continue;
        } else if tile.piece.colour == Colour::Black && !is_white {
            continue;
        } else {
            legal_move_list.push((possible_move.0 as usize, possible_move.1 as usize));
        }
    }

    // Castling
    let king_pos: (usize, usize);
    if is_white {
        king_pos = board.kingpos_w;
    } else {
        king_pos = board.kingpos_b;
    };

    // Do not check castlings if king has moved
    if board.tiles[king_pos.0][king_pos.1].piece.piece_type == Type::King(true) {
        return legal_move_list;
    }

    // Castling to the left
    if board.tiles[king_pos.0][king_pos.1 - 1].piece.piece_type == Type::Empty
        && board.tiles[king_pos.0][king_pos.1 - 2].piece.piece_type == Type::Empty
        && board.tiles[king_pos.0][king_pos.1 - 3].piece.piece_type == Type::Empty
        && board.tiles[king_pos.0][king_pos.1 - 4].piece.piece_type == Type::Rook(false)
    {
        legal_move_list.push((king_pos.0, king_pos.1 - 2));
    }

    // Castling to the right
    if board.tiles[king_pos.0][king_pos.1 + 1].piece.piece_type == Type::Empty
        && board.tiles[king_pos.0][king_pos.1 + 2].piece.piece_type == Type::Empty
        && board.tiles[king_pos.0][king_pos.1 + 3].piece.piece_type == Type::Rook(false)
    {
        legal_move_list.push((king_pos.0, king_pos.1 + 2));
    }

    return legal_move_list;
}
