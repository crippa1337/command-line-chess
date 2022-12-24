use crate::*;
use evaluation::evaluate_board;
use moves::move_piece;

// get all the possible moves for a side, and return a vector with tuples of ((usize, usize), (usize, usize)) representing the from and to positions of the move
pub fn get_all_moves(board: Board, is_white: bool) -> Vec<((usize, usize), Vec<(usize, usize)>)> {
    let mut moves = Vec::new();

    for i in 0..8 {
        for j in 0..8 {
            // Do not check empty tiles
            if board.tiles[i][j].piece.piece_type == Type::Empty {
                continue;
            }

            // Check if the piece is the same colour as the side
            if is_white {
                if board.tiles[i][j].piece.colour == Colour::White {
                    let piece_moves = legal_moves(board, (i, j), is_white);
                    moves.push(((i, j), piece_moves));
                }
            } else {
                if board.tiles[i][j].piece.colour == Colour::Black {
                    let piece_moves = legal_moves(board, (i, j), is_white);
                    moves.push(((i, j), piece_moves));
                }
            }
        }
    }

    return moves;
}

pub fn minimax(board: Board, depth: i32, mut alpha: i32, mut beta: i32, is_white: bool) -> i32 {
    // White is maximizer
    // Black is minimizer

    if depth == 0 {
        return evaluate_board(board);
    }

    // if maximizing player
    if is_white {
        let mut max_eval = std::i32::MIN;
        let moves = get_all_moves(board, true);

        for (from, to) in moves {
            for to_pos in to {
                // Do the move on a cloned board
                let mut new_board = board.clone();
                match move_piece(&mut new_board, from, to_pos, true) {
                    Ok(_) => (),
                    Err(_) => continue,
                }

                // Recursively call minimax on the new board
                let eval = minimax(new_board, depth - 1, alpha, beta, false);
                max_eval = std::cmp::max(max_eval, eval);

                // Alpha beta pruning
                alpha = std::cmp::max(alpha, eval);
                if beta <= alpha {
                    break;
                }
            }
        }
        return max_eval;
    } else {
        let mut min_eval = std::i32::MAX;
        let moves = get_all_moves(board, false);

        for (from, to) in moves {
            for to_pos in to {
                let mut new_board = board.clone();
                match move_piece(&mut new_board, from, to_pos, false) {
                    Ok(_) => (),
                    Err(_) => continue,
                }

                let eval = minimax(new_board, depth - 1, alpha, beta, true);
                min_eval = std::cmp::min(min_eval, eval);

                beta = std::cmp::min(beta, eval);
                if beta <= alpha {
                    break;
                }
            }
        }
        return min_eval;
    }
}

pub fn max_move(board: Board, depth: i32) -> ((usize, usize), (usize, usize)) {
    // White is maximizer

    let mut max_eval = std::i32::MIN;
    let mut best_move = ((0, 0), (0, 0));
    let moves = get_all_moves(board, true);

    for (from, to) in moves {
        for to_pos in to {
            let mut new_board = board.clone();
            match move_piece(&mut new_board, from, to_pos, true) {
                Ok(_) => (),
                Err(_) => continue,
            }

            let eval = minimax(new_board, depth - 1, std::i32::MIN, std::i32::MAX, false);
            if eval > max_eval {
                max_eval = eval;
                best_move = (from, to_pos);
            }
        }
    }

    println!("Best move: {:?} with score: {}", best_move, max_eval);
    return best_move;
}

pub fn min_move(board: Board, depth: i32) -> ((usize, usize), (usize, usize)) {
    // Black is minimizer

    let mut min_eval = std::i32::MAX;
    let mut best_move = ((0, 0), (0, 0));
    let moves = get_all_moves(board, false);

    for (from, to) in moves {
        for to_pos in to {
            let mut new_board = board.clone();
            match move_piece(&mut new_board, from, to_pos, false) {
                Ok(_) => (),
                Err(_) => continue,
            }

            let eval = minimax(new_board, depth - 1, std::i32::MIN, std::i32::MAX, true);
            if eval < min_eval {
                min_eval = eval;
                best_move = (from, to_pos);
            }
        }
    }

    return best_move;
}
