use crate::*;

// get all the possible moves for a side, and return a vector with tuples of ((usize, usize), (usize, usize)) representing the from and to positions of the move
pub fn get_all_moves(
    board: &mut Board,
    is_white: bool,
) -> Vec<((usize, usize), Vec<(usize, usize)>)> {
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
