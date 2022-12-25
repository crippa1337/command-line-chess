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

    // P = 100
    // N = 320
    // B = 330
    // R = 500
    // Q = 900
    // K = 20000
    for row in 0..8 {
        for col in 0..8 {
            let piece = board.tiles[row][col].piece;

            if piece.piece_type != Type::Empty {
                let piece_value = match piece.piece_type {
                    Type::Pawn(_) => 100,
                    Type::Knight => 320,
                    Type::Bishop => 330,
                    Type::Rook(_) => 500,
                    Type::Queen => 900,
                    Type::King(_) => 20000,
                    _ => 0,
                };

                if piece.colour == Colour::White {
                    let white_pos_value = match piece.piece_type {
                        Type::Pawn(_) => PAWN_TABLE[row][col],
                        Type::Knight => KNIGHT_TABLE[row][col],
                        Type::Bishop => BISHOP_TABLE[row][col],
                        Type::Rook(_) => ROOK_TABLE[row][col],
                        Type::Queen => QUEEN_TABLE[row][col],
                        Type::King(_) => KING_TABLE[row][col],
                        _ => 0,
                    };

                    white_score += piece_value + white_pos_value;
                } else {
                    let black_pos_value = match piece.piece_type {
                        Type::Pawn(_) => PAWN_TABLE[7 - row][col],
                        Type::Knight => KNIGHT_TABLE[7 - row][col],
                        Type::Bishop => BISHOP_TABLE[7 - row][col],
                        Type::Rook(_) => ROOK_TABLE[7 - row][col],
                        Type::Queen => QUEEN_TABLE[7 - row][col],
                        Type::King(_) => KING_TABLE[7 - row][col],
                        _ => 0,
                    };

                    black_score += piece_value + black_pos_value;
                }
            }
        }
    }

    return white_score - black_score;
}

////////////////////////////////////////
//Tomasz Michniewski's evaluation tables
////////////////////////////////////////

//pawns
#[rustfmt::skip]
const PAWN_TABLE: [[i32; 8]; 8] = [
    [0,  0,  0,  0,  0,  0,  0,  0],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [ 5,  5, 10, 25, 25, 10,  5,  5],
    [ 0,  0,  0, 20, 20,  0,  0,  0],
    [ 5, -5,-10,  0,  0,-10, -5,  5],
    [ 5, 10, 10,-20,-20, 10, 10,  5],
    [ 0,  0,  0,  0,  0,  0,  0,  0]
];

//knights
#[rustfmt::skip]
const KNIGHT_TABLE: [[i32; 8]; 8] = [
    [-50,-40,-30,-30,-30,-30,-40,-50],
    [-40,-20,  0,  0,  0,  0,-20,-40],
    [-30,  0, 10, 15, 15, 10,  0,-30],
    [-30,  5, 15, 20, 20, 15,  5,-30],
    [-30,  0, 15, 20, 20, 15,  0,-30],
    [-30,  5, 10, 15, 15, 10,  5,-30],
    [-40,-20,  0,  5,  5,  0,-20,-40],
    [-50,-40,-30,-30,-30,-30,-40,-50],
];

//bishops
#[rustfmt::skip]
const BISHOP_TABLE: [[i32; 8]; 8] = [
    [-20,-10,-10,-10,-10,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5, 10, 10,  5,  0,-10],
    [-10,  5,  5, 10, 10,  5,  5,-10],
    [-10,  0, 10, 10, 10, 10,  0,-10],
    [-10, 10, 10, 10, 10, 10, 10,-10],
    [-10,  5,  0,  0,  0,  0,  5,-10],
    [-20,-10,-10,-10,-10,-10,-10,-20],
];

//rooks
#[rustfmt::skip]
const ROOK_TABLE: [[i32; 8]; 8] = [
    [ 0,  0,  0,  0,  0,  0,  0,  0],
    [ 5, 10, 10, 10, 10, 10, 10,  5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [ 0,  0,  0,  5,  5,  0,  0,  0],
];

//queens
#[rustfmt::skip]
const QUEEN_TABLE: [[i32; 8]; 8] = [
    [-20,-10,-10, -5, -5,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5,  5,  5,  5,  0,-10],
    [ -5,  0,  5,  5,  5,  5,  0, -5],
    [  0,  0,  5,  5,  5,  5,  0, -5],
    [-10,  5,  5,  5,  5,  5,  0,-10],
    [-10,  0,  5,  0,  0,  0,  0,-10],
    [-20,-10,-10, -5, -5,-10,-10,-20],
];

//kings
#[rustfmt::skip]
const KING_TABLE: [[i32; 8]; 8] = [
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-20,-30,-30,-40,-40,-30,-30,-20],
    [-10,-20,-20,-20,-20,-20,-20,-10],
    [ 20, 20,  0,  0,  0,  0, 20, 20],
    [ 20, 30, 10,  0,  0, 10, 30, 20],
];

//kings endgame
// #[rustfmt::skip]
// const KING_ENDGAME_TABLE: [[i32; 8]; 8] = [
//     [-50,-40,-30,-20,-20,-30,-40,-50],
//     [-30,-20,-10,  0,  0,-10,-20,-30],
//     [-30,-10, 20, 30, 30, 20,-10,-30],
//     [-30,-10, 30, 40, 40, 30,-10,-30],
//     [-30,-10, 30, 40, 40, 30,-10,-30],
//     [-30,-10, 20, 30, 30, 20,-10,-30],
//     [-30,-30,  0,  0,  0,  0,-30,-30],
//     [-50,-30,-30,-30,-30,-30,-30,-50],
// ];
