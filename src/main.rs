use std::io::{stdin, Write};

use ansi_term::Colour::{Fixed, Red, White, RGB};

fn main() {
    let board = Board::new();
    board.draw_board();
    player_input(board);
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn player_input(board: Board) {
    'initial: loop {
        print!("{} ", White.bold().paint(">>>"));
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_lowercase().to_string();

        if input == "exit" {
            std::process::exit(0);
        }

        if input.len() != 5 {
            input_error(Error::Length);
            continue;
        }
    }
}

fn input_error(error: Error) {
    match error {
        Error::Empty => println!(
            "{} {}",
            Red.bold().paint(">>>"),
            "You can't move an empty tile!"
        ),
        Error::Length => println!(
            "{} {}",
            Red.bold().paint(">>>"),
            "Your input needs to be 5 chars long!"
        ),
        Error::InvalidMove => println!("{} {}", Red.bold().paint(">>>"), "Invalid move!"),
        Error::OutOfBounds => println!("{} {}", Red.bold().paint(">>>"), "Out of bounds!"),
        Error::IncorrectTile => println!("{} {}", Red.bold().paint(">>>"), "Incorrect tile!"),
    }
}

struct Board {
    tiles: [[Tile; 8]; 8],
}

impl Board {
    fn new() -> Self {
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
        for (col_id, tile) in board[1].iter_mut().enumerate() {
            tile.piece = Piece {
                piece_type: Type::Pawn,
                colour: Colour::Black,
            };
        }

        for (col_id, tile) in board[6].iter_mut().enumerate() {
            tile.piece = Piece {
                piece_type: Type::Pawn,
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

        return Board { tiles: board };
    }

    fn draw_board(&self) {
        let grey = RGB(80, 80, 80);
        let brown = Red;
        println!("      A   B   C   D   E   F   G   H");
        for (row_id, row) in self.tiles.iter().enumerate() {
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
                        Colour::White => print!("·"),
                    },
                    Type::Pawn => match tile.piece.colour {
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
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Tile {
    piece: Piece,
    colour: Colour,
}

#[derive(Copy, Clone, PartialEq)]
struct Piece {
    piece_type: Type,
    colour: Colour,
}

#[derive(Copy, Clone, PartialEq)]
enum Type {
    Empty,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq)]
enum Colour {
    White,
    Black,
}

enum Error {
    Length,
    Empty,
    InvalidMove,
    OutOfBounds,
    IncorrectTile,
}
