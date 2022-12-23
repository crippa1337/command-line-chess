use std::collections::HashMap;

use crate::*;

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

// evaluate the board for a side
// Pawns are 1, Knights are 3, Bishops are 3, Rooks are 5, Queens are 9
pub fn evaluate_board(board: Board, is_white: bool) -> i32 {
    let mut white_score = 0;
    let mut black_score = 0;
    let mut white_middle_score = 0;
    let mut black_middle_score = 0;

    match check_for_mates(board.clone()) {
        Some(Colour::White) => {
            if is_white {
                return std::i32::MAX;
            } else {
                return std::i32::MIN;
            }
        }
        Some(Colour::Black) => {
            if is_white {
                return std::i32::MIN;
            } else {
                return std::i32::MAX;
            }
        }
        _ => {}
    }

    for i in 0..8 {
        for j in 0..8 {
            if board.tiles[i][j].piece.piece_type == Type::Empty {
                continue;
            }

            let piece_color = board.tiles[i][j].piece.colour;
            let piece_value = match board.tiles[i][j].piece.piece_type {
                Type::Pawn(_) => 1,
                Type::Knight => 3,
                Type::Bishop => 3,
                Type::Rook => 5,
                Type::Queen => 9,
                _ => 0,
            };

            if piece_color == Colour::White {
                white_score += piece_value;

                // If the piece is in the middle of the board, add a bonus to the score
                if i > 2 && i < 5 && j > 2 && j < 5 {
                    white_middle_score += 1;
                }
            } else {
                black_score += piece_value;

                // If the piece is in the middle of the board, add a bonus to the score
                if i > 2 && i < 5 && j > 2 && j < 5 {
                    black_middle_score += 1;
                }
            }
        }
    }

    if is_white {
        return white_score - black_score + white_middle_score - black_middle_score;
    } else {
        return black_score - white_score + black_middle_score - white_middle_score;
    }
}

// negamax algorithm with alpha-beta pruning

// the depth parameter is the depth of the search tree
// the alpha parameter is the best score that the maximizer currently can guarantee at that level or above
// the beta parameter is the best score that the minimizer currently can guarantee at that level or above
// returns a score

// pseudocode:
// int alphaBeta( int alpha, int beta, int depthleft ) {
//    if( depthleft == 0 ) return quiesce( alpha, beta );
//    for ( all moves)  {
//       score = -alphaBeta( -beta, -alpha, depthleft - 1 );
//       if( score >= beta )
//          return beta;   //  fail hard beta-cutoff
//       if( score > alpha )
//          alpha = score; // alpha acts like max in MiniMax
//    }
//    return alpha;
// }

pub fn negamax(
    board: Board,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    is_white: bool,
    old_boards: &mut HashMap<([[Tile; 8]; 8], bool, i32), i32>,
) -> i32 {
    if depth == 0 {
        return evaluate_board(board, is_white);
    }

    let mut best_score = -1000000;

    if let Some(score) = old_boards.get(&(board.tiles, is_white, depth)) {
        return *score;
    }

    let moves = get_all_moves(board, is_white);

    for (from, to) in &moves {
        for to_pos in to {
            let mut new_board = board.clone();

            match move_piece(&mut new_board, *from, *to_pos, is_white) {
                Ok(_) => {}
                Err(_) => continue,
            }

            let score = -negamax(new_board, depth - 1, -beta, -alpha, !is_white, old_boards);

            if score >= beta {
                return beta;
            }

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                alpha = score;
            }
        }
    }

    old_boards.insert((board.tiles, is_white, depth), best_score);

    return best_score;
}

// get the best move for a side
pub fn get_best_move(
    board: Board,
    mut depth: i32,
    is_white: bool,
) -> ((usize, usize), (usize, usize)) {
    let mut old_boards = HashMap::new();
    let mut best_score = -1000000;
    let mut best_move = ((0, 0), (0, 0));

    let moves = get_all_moves(board, is_white);
    let pieces1 = moves.len();
    let pieces2 = get_all_moves(board, !is_white).len();

    if pieces1 + pieces2 < 12 {
        depth += 1;
    } else if pieces1 + pieces2 < 6 {
        depth += 2;
    }

    for (from, to) in moves {
        for to_pos in to {
            let mut new_board = board.clone();

            match move_piece(&mut new_board, from, to_pos, is_white) {
                Ok(_) => {}
                Err(_) => continue,
            }

            let score = -negamax(
                new_board,
                depth,
                -1000000,
                1000000,
                !is_white,
                &mut old_boards,
            );

            if score > best_score {
                best_score = score;
                best_move = (from, to_pos);
            }
        }
    }

    return best_move;
}
