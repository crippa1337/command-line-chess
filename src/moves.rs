use crate::{cpu::get_all_moves, *};

pub fn is_in_check(board: &mut Board, is_white: bool) -> bool {
    // Find all the positions of the opponent's pieces
    let mut opponent_positions = Vec::new();
    for i in 0..8 {
        for j in 0..8 {
            // Do not check empty tiles
            if board.tiles[i][j].piece.piece_type == Type::Empty {
                continue;
            }

            if is_white {
                if board.tiles[i][j].piece.colour == Colour::Black {
                    opponent_positions.push((i, j));
                }
            } else {
                if board.tiles[i][j].piece.colour == Colour::White {
                    opponent_positions.push((i, j));
                }
            }
        }
    }

    // Generate all the possible moves for the opponent's pieces
    let mut opponent_moves = Vec::new();
    for pos in opponent_positions {
        opponent_moves.append(&mut legal_moves(board, pos, !is_white));
    }

    let king_pos = if is_white {
        board.kingpos_w
    } else {
        board.kingpos_b
    };

    // Check if any of the opponent's moves will capture the king
    for moves in opponent_moves {
        if moves == king_pos {
            return true;
        }
    }

    return false;
}

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
            legal_moves.append(&mut legal_diagonal_moves(board, from, is_white));
        }
        Type::Queen => {
            legal_moves.append(&mut legal_straight_moves(board, from, is_white));
            legal_moves.append(&mut legal_diagonal_moves(board, from, is_white));
        }
        Type::King => {
            legal_moves.append(&mut legal_king_moves(board, from, is_white));
        }
        Type::Empty => unreachable!(),
    }

    return legal_moves;
}

pub fn check_for_mates(board: &mut Board) -> Option<Colour> {
    // Check if any kings are in check, if it is, create a copy of the board and test all the possible moves from all possible pieces
    // If none of the moves will remove the king from check, return winner
    // Check if white king is in checkmate
    if is_in_check(board, true) {
        let moves = get_all_moves(board, true);
        let mut can_escape_check = false;

        for ((from_row, from_col), tos) in moves {
            for (to_row, to_col) in tos {
                let mut temp_board = board.clone();
                match move_piece(
                    &mut temp_board,
                    (from_row, from_col),
                    (to_row, to_col),
                    true,
                ) {
                    Ok(()) => (),
                    Err(e) => {
                        continue;
                    }
                }

                if is_in_check(&mut temp_board, true) {
                    can_escape_check = true;
                    break;
                }
            }

            if can_escape_check {
                break;
            }
        }

        if !can_escape_check {
            return Some(Colour::Black);
        }
    }

    // Check if black king is in checkmate
    if is_in_check(board, false) {
        let moves = get_all_moves(board, false);
        let mut can_escape_check = false;

        for ((from_row, from_col), tos) in moves {
            for (to_row, to_col) in tos {
                let mut temp_board = board.clone();
                match move_piece(
                    &mut temp_board,
                    (from_row, from_col),
                    (to_row, to_col),
                    false,
                ) {
                    Ok(()) => (),
                    Err(e) => {
                        continue;
                    }
                }

                if is_in_check(&mut temp_board, false) {
                    can_escape_check = true;
                    break;
                }
            }

            if can_escape_check {
                break;
            }
        }

        if !can_escape_check {
            return Some(Colour::White);
        }
    }

    None
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
        Type::King => {
            if is_white {
                test_board.kingpos_w = to;
            } else {
                test_board.kingpos_b = to;
            }
        }
        _ => (),
    }
    test_board.tiles[to.0][to.1].piece = test_board.tiles[from.0][from.1].piece;
    test_board.tiles[from.0][from.1].piece.piece_type = Type::Empty;

    // Check if the king is in check after the move
    if is_in_check(&mut test_board, is_white) {
        return Err(Error::Check);
    }

    // Actually do the move if king isn't in check
    // Pawn has moved -> true if false
    // King has moved -> new king pos
    match board.tiles[from.0][from.1].piece.piece_type {
        Type::King => {
            if is_white {
                board.kingpos_w = to;
            } else {
                board.kingpos_b = to;
            }
        }
        Type::Pawn(false) => {
            board.tiles[from.0][from.1].piece.piece_type = Type::Pawn(true);
        }
        _ => (),
    }
    board.tiles[to.0][to.1].piece = board.tiles[from.0][from.1].piece;
    board.tiles[from.0][from.1].piece.piece_type = Type::Empty;

    // Pawn swap at edge
    if is_white {
        if board.tiles[to.0][to.1].piece.piece_type == Type::Pawn(true) {
            if to.0 == 0 || to.0 == 7 {
                clear_draw(board, true);
                pawn_swap(board, to, true);
            }
        }
    } else {
        if board.tiles[to.0][to.1].piece.piece_type == Type::Pawn(true) {
            if to.0 == 0 || to.0 == 7 {
                clear_draw(board, false);
                pawn_swap(board, to, false);
            }
        }
    }

    return Ok(());
}

pub fn legal_pawn_moves(
    board: &mut Board,
    from: (usize, usize),
    is_white: bool,
) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();
    let mut possible_moves: Vec<(usize, usize)> = Vec::new();
    let mut possible_captures: Vec<(usize, usize)> = Vec::new();

    // Forward moves
    if board.tiles[from.0][from.1].piece.piece_type == Type::Pawn(false) {
        if is_white {
            possible_moves.push((from.0.wrapping_sub(1), from.1));
            possible_moves.push((from.0.wrapping_sub(2), from.1));
        } else {
            possible_moves.push((from.0.wrapping_add(1), from.1));
            possible_moves.push((from.0.wrapping_add(2), from.1));
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

pub fn pawn_swap(board: &mut Board, to: (usize, usize), is_white: bool) {
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
                board.tiles[to.0][to.1].piece.piece_type = Type::Rook;
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

        if axes.iter().all(|&x| !x) {
            break;
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

pub fn legal_diagonal_moves(
    board: &mut Board,
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

pub fn legal_king_moves(
    board: &mut Board,
    from: (usize, usize),
    is_white: bool,
) -> Vec<(usize, usize)> {
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

    return legal_move_list;
}
