use crate::*;

pub fn evaluate_board(board: Board) -> i32 {
    // White is maximizer
    // Black is minimizer

    let mut white_score = 0;
    let mut black_score = 0;

    if let Some(winner) = check_for_mates(board) {
        if winner == Colour::White {
            return std::i32::MAX;
        } else {
            return std::i32::MIN;
        }
    }

    for row in 0..8 {
        for col in 0..8 {
            let piece = board.tiles[row][col].piece;

            if piece.piece_type != Type::Empty {
                let piece_value = match piece.piece_type {
                    Type::Pawn(_) => 1,
                    Type::Knight => 3,
                    Type::Bishop => 3,
                    Type::Rook => 5,
                    Type::Queen => 9,
                    _ => 0,
                };

                if piece.colour == Colour::White {
                    white_score += piece_value;
                } else {
                    black_score += piece_value;
                }
            }
        }
    }

    return white_score - black_score;
}
