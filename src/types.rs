use ansi_term::Colour::{Red, White, RGB};

#[derive(Clone, Copy)]
pub struct Board {
    pub tiles: [[Tile; 8]; 8],
    pub kingpos_w: (usize, usize),
    pub kingpos_b: (usize, usize),
}

impl Board {
    pub fn new() -> Self {
        // Initialize a chess board, where every other tile is black and every other tile is white
        let mut board = [[Tile {
            piece: Piece {
                piece_type: Type::Empty,
                colour: Colour::White,
            },
            colour: Colour::White,
        }; 8]; 8];

        for (row_id, row) in board.iter_mut().enumerate() {
            for (col_id, tile) in row.iter_mut().enumerate() {
                if (row_id + col_id) % 2 == 0 {
                    tile.colour = Colour::Black;
                }
            }
        }

        // Initialize the pieces
        // Pawns
        for tile in board[1].iter_mut() {
            tile.piece = Piece {
                piece_type: Type::Pawn(false),
                colour: Colour::Black,
            };
        }

        for tile in board[6].iter_mut() {
            tile.piece = Piece {
                piece_type: Type::Pawn(false),
                colour: Colour::White,
            };
        }

        // Rooks
        board[0][0].piece = Piece {
            piece_type: Type::Rook,
            colour: Colour::Black,
        };

        board[0][7].piece = Piece {
            piece_type: Type::Rook,
            colour: Colour::Black,
        };

        board[7][0].piece = Piece {
            piece_type: Type::Rook,
            colour: Colour::White,
        };

        board[7][7].piece = Piece {
            piece_type: Type::Rook,
            colour: Colour::White,
        };

        // Knights
        board[0][1].piece = Piece {
            piece_type: Type::Knight,
            colour: Colour::Black,
        };

        board[0][6].piece = Piece {
            piece_type: Type::Knight,
            colour: Colour::Black,
        };

        board[7][1].piece = Piece {
            piece_type: Type::Knight,
            colour: Colour::White,
        };

        board[7][6].piece = Piece {
            piece_type: Type::Knight,
            colour: Colour::White,
        };

        // Bishops

        board[0][2].piece = Piece {
            piece_type: Type::Bishop,
            colour: Colour::Black,
        };

        board[0][5].piece = Piece {
            piece_type: Type::Bishop,
            colour: Colour::Black,
        };

        board[7][2].piece = Piece {
            piece_type: Type::Bishop,
            colour: Colour::White,
        };

        board[7][5].piece = Piece {
            piece_type: Type::Bishop,
            colour: Colour::White,
        };

        // Queens

        board[0][3].piece = Piece {
            piece_type: Type::Queen,
            colour: Colour::Black,
        };

        board[7][3].piece = Piece {
            piece_type: Type::Queen,
            colour: Colour::White,
        };

        // Kings

        board[0][4].piece = Piece {
            piece_type: Type::King,
            colour: Colour::Black,
        };

        board[7][4].piece = Piece {
            piece_type: Type::King,
            colour: Colour::White,
        };

        return Board {
            tiles: board,
            kingpos_b: (0, 4),
            kingpos_w: (7, 4),
        };
    }

    pub fn draw_board(&self) {
        let grey = RGB(80, 80, 80);
        let brown = Red;
        let mut row_id = 8;
        println!("      a   b   c   d   e   f   g   h");
        for row in self.tiles.iter() {
            row_id -= 1;
            print!(
                "{}\n {}  ",
                grey.paint("    +---+---+---+---+---+---+---+---+"),
                row_id + 1
            );
            for &tile in row {
                print!("{}", grey.paint("| "));
                match tile.piece.piece_type {
                    Type::Empty => match tile.colour {
                        Colour::Black => print!("{}", brown.paint("·")),
                        Colour::White => print!("{}", White.paint("·")),
                    },
                    Type::Pawn(_) => match tile.piece.colour {
                        Colour::Black => print!("{}", brown.bold().paint("♙")),
                        Colour::White => print!("{}", White.bold().paint("♙")),
                    },
                    Type::Rook => match tile.piece.colour {
                        Colour::Black => print!("{}", brown.bold().paint("♖")),
                        Colour::White => print!("{}", White.bold().paint("♜")),
                    },
                    Type::Knight => match tile.piece.colour {
                        Colour::Black => print!("{}", brown.bold().paint("♘")),
                        Colour::White => print!("{}", White.bold().paint("♞")),
                    },
                    Type::Bishop => match tile.piece.colour {
                        Colour::Black => print!("{}", brown.bold().paint("♗")),
                        Colour::White => print!("{}", White.bold().paint("♝")),
                    },
                    Type::Queen => match tile.piece.colour {
                        Colour::Black => print!("{}", brown.bold().paint("♕")),
                        Colour::White => print!("{}", White.bold().paint("♛")),
                    },
                    Type::King => match tile.piece.colour {
                        Colour::Black => print!("{}", brown.bold().paint("♔")),
                        Colour::White => print!("{}", White.bold().paint("♚")),
                    },
                }
                print!("{}", grey.paint(" "));
            }
            println!("{}", grey.paint("|"));
        }
        println!("{}", grey.paint("    +---+---+---+---+---+---+---+---+"));
        println!("      a   b   c   d   e   f   g   h");
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Tile {
    pub piece: Piece,
    pub colour: Colour,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub piece_type: Type,
    pub colour: Colour,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Empty,
    Pawn(bool),
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Colour {
    White,
    Black,
}

pub enum Error {
    Length,
    Empty,
    IllegalMove,
    OutOfBounds,
    EnemyMove,
    TeamDmg,
    Check,
}
