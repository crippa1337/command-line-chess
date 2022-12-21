mod moves;
mod types;
use ansi_term::Colour::{Red, White, RGB};
use moves::*;
use std::io::{stdin, Write};
use types::*;

fn main() {
    let mut board = Board::new();
    arrow_print("Welcome to C-Chess!", true);
    arrow_print("Input 'exit' to exit the application at anytime.", false);
    arrow_print("Hit Enter to continue . . .", true);

    stdin().read_line(&mut String::new()).unwrap();

    clear_draw(&mut board);
    player_input(&mut board);
}

fn player_input(board: &mut Board) {
    loop {
        // White's turn
        println!("White's Turn");
        let (from, to) = handle_player(board, true);
        let white_moves = legal_moves(board, from, true);
        if white_moves.contains(&to) {
            match move_piece(board, from, to, true) {
                Err(e) => {
                    clear_draw(board);
                    input_error(e);
                    continue;
                }

                Ok(_) => {
                    clear_draw(board);
                }
            }
        } else {
            clear_draw(board);
            input_error(Error::IllegalMove);
            continue;
        }

        // Black's turn
        println!("Black's Turn");
        let (from, to) = handle_player(board, false);
        println!("{from:?} {to:?}");
        println!("{:?}", board.tiles[from.0][from.1].piece.piece_type);
        let black_moves = legal_moves(board, from, false);
        if black_moves.contains(&to) {
            match move_piece(board, from, to, false) {
                Err(e) => {
                    clear_draw(board);
                    input_error(e);
                    continue;
                }

                Ok(_) => {
                    clear_draw(board);
                }
            }
        } else {
            clear_draw(board);
            input_error(Error::IllegalMove);
            continue;
        }
    }
}

fn handle_player(board: &mut Board, is_white: bool) -> ((usize, usize), (usize, usize)) {
    let colour;
    if is_white {
        colour = Colour::White;
    } else {
        colour = Colour::Black;
    }

    loop {
        print!("{} ", White.bold().paint(">>>"));
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_lowercase().to_string();

        if input == "exit" {
            std::process::exit(0);
        }

        if input.len() != 4 {
            clear_draw(board);
            input_error(Error::Length);
            continue;
        }

        let (from, to) = match_input(input);

        if from.0 == 99 || from.1 == 99 || to.0 == 99 || to.1 == 99 {
            clear_draw(board);
            input_error(Error::OutOfBounds);
            continue;
        }

        if board.tiles[from.0][from.1].piece.piece_type == Type::Empty {
            clear_draw(board);
            input_error(Error::Empty);
            continue;
        }

        if board.tiles[from.0][from.1].piece.colour != colour
            && board.tiles[from.0][from.1].piece.piece_type != Type::Empty
        {
            clear_draw(board);
            input_error(Error::EnemyMove);
            continue;
        }

        if board.tiles[to.0][to.1].piece.colour == colour
            && board.tiles[to.0][to.1].piece.piece_type != Type::Empty
        {
            clear_draw(board);
            input_error(Error::TeamDmg);
            continue;
        }

        return (from, to);
    }
}

fn match_input(input: String) -> ((usize, usize), (usize, usize)) {
    let mut chars = input.chars();
    let mut from = String::new();
    let mut to = String::new();

    for _ in 0..2 {
        from.push(chars.next().unwrap());
    }

    for _ in 0..2 {
        to.push(chars.next().unwrap());
    }

    let from: (usize, usize) = match from.as_str() {
        "a1" => (7, 0),
        "a2" => (6, 0),
        "a3" => (5, 0),
        "a4" => (4, 0),
        "a5" => (3, 0),
        "a6" => (2, 0),
        "a7" => (1, 0),
        "a8" => (0, 0),
        "b1" => (7, 1),
        "b2" => (6, 1),
        "b3" => (5, 1),
        "b4" => (4, 1),
        "b5" => (3, 1),
        "b6" => (2, 1),
        "b7" => (1, 1),
        "b8" => (0, 1),
        "c1" => (7, 2),
        "c2" => (6, 2),
        "c3" => (5, 2),
        "c4" => (4, 2),
        "c5" => (3, 2),
        "c6" => (2, 2),
        "c7" => (1, 2),
        "c8" => (0, 2),
        "d1" => (7, 3),
        "d2" => (6, 3),
        "d3" => (5, 3),
        "d4" => (4, 3),
        "d5" => (3, 3),
        "d6" => (2, 3),
        "d7" => (1, 3),
        "d8" => (0, 3),
        "e1" => (7, 4),
        "e2" => (6, 4),
        "e3" => (5, 4),
        "e4" => (4, 4),
        "e5" => (3, 4),
        "e6" => (2, 4),
        "e7" => (1, 4),
        "e8" => (0, 4),
        "f1" => (7, 5),
        "f2" => (6, 5),
        "f3" => (5, 5),
        "f4" => (4, 5),
        "f5" => (3, 5),
        "f6" => (2, 5),
        "f7" => (1, 5),
        "f8" => (0, 5),
        "g1" => (7, 6),
        "g2" => (6, 6),
        "g3" => (5, 6),
        "g4" => (4, 6),
        "g5" => (3, 6),
        "g6" => (2, 6),
        "g7" => (1, 6),
        "g8" => (0, 6),
        "h1" => (7, 7),
        "h2" => (6, 7),
        "h3" => (5, 7),
        "h4" => (4, 7),
        "h5" => (3, 7),
        "h6" => (2, 7),
        "h7" => (1, 7),
        "h8" => (0, 7),
        _ => (99, 99),
    };

    let to: (usize, usize) = match to.as_str() {
        "a1" => (7, 0),
        "a2" => (6, 0),
        "a3" => (5, 0),
        "a4" => (4, 0),
        "a5" => (3, 0),
        "a6" => (2, 0),
        "a7" => (1, 0),
        "a8" => (0, 0),
        "b1" => (7, 1),
        "b2" => (6, 1),
        "b3" => (5, 1),
        "b4" => (4, 1),
        "b5" => (3, 1),
        "b6" => (2, 1),
        "b7" => (1, 1),
        "b8" => (0, 1),
        "c1" => (7, 2),
        "c2" => (6, 2),
        "c3" => (5, 2),
        "c4" => (4, 2),
        "c5" => (3, 2),
        "c6" => (2, 2),
        "c7" => (1, 2),
        "c8" => (0, 2),
        "d1" => (7, 3),
        "d2" => (6, 3),
        "d3" => (5, 3),
        "d4" => (4, 3),
        "d5" => (3, 3),
        "d6" => (2, 3),
        "d7" => (1, 3),
        "d8" => (0, 3),
        "e1" => (7, 4),
        "e2" => (6, 4),
        "e3" => (5, 4),
        "e4" => (4, 4),
        "e5" => (3, 4),
        "e6" => (2, 4),
        "e7" => (1, 4),
        "e8" => (0, 4),
        "f1" => (7, 5),
        "f2" => (6, 5),
        "f3" => (5, 5),
        "f4" => (4, 5),
        "f5" => (3, 5),
        "f6" => (2, 5),
        "f7" => (1, 5),
        "f8" => (0, 5),
        "g1" => (7, 6),
        "g2" => (6, 6),
        "g3" => (5, 6),
        "g4" => (4, 6),
        "g5" => (3, 6),
        "g6" => (2, 6),
        "g7" => (1, 6),
        "g8" => (0, 6),
        "h1" => (7, 7),
        "h2" => (6, 7),
        "h3" => (5, 7),
        "h4" => (4, 7),
        "h5" => (3, 7),
        "h6" => (2, 7),
        "h7" => (1, 7),
        "h8" => (0, 7),
        _ => (99, 99),
    };

    return (from, to);
}

pub fn reverse_match_input(input: (usize, usize)) -> String {
    match input {
        (0, 0) => return String::from("a8"),
        (0, 1) => return String::from("b8"),
        (0, 2) => return String::from("c8"),
        (0, 3) => return String::from("d8"),
        (0, 4) => return String::from("e8"),
        (0, 5) => return String::from("f8"),
        (0, 6) => return String::from("g8"),
        (0, 7) => return String::from("h8"),
        (1, 0) => return String::from("a7"),
        (1, 1) => return String::from("b7"),
        (1, 2) => return String::from("c7"),
        (1, 3) => return String::from("d7"),
        (1, 4) => return String::from("e7"),
        (1, 5) => return String::from("f7"),
        (1, 6) => return String::from("g7"),
        (1, 7) => return String::from("h7"),
        (2, 0) => return String::from("a6"),
        (2, 1) => return String::from("b6"),
        (2, 2) => return String::from("c6"),
        (2, 3) => return String::from("d6"),
        (2, 4) => return String::from("e6"),
        (2, 5) => return String::from("f6"),
        (2, 6) => return String::from("g6"),
        (2, 7) => return String::from("h6"),
        (3, 0) => return String::from("a5"),
        (3, 1) => return String::from("b5"),
        (3, 2) => return String::from("c5"),
        (3, 3) => return String::from("d5"),
        (3, 4) => return String::from("e5"),
        (3, 5) => return String::from("f5"),
        (3, 6) => return String::from("g5"),
        (3, 7) => return String::from("h5"),
        (4, 0) => return String::from("a4"),
        (4, 1) => return String::from("b4"),
        (4, 2) => return String::from("c4"),
        (4, 3) => return String::from("d4"),
        (4, 4) => return String::from("e4"),
        (4, 5) => return String::from("f4"),
        (4, 6) => return String::from("g4"),
        (4, 7) => return String::from("h4"),
        (5, 0) => return String::from("a3"),
        (5, 1) => return String::from("b3"),
        (5, 2) => return String::from("c3"),
        (5, 3) => return String::from("d3"),
        (5, 4) => return String::from("e3"),
        (5, 5) => return String::from("f3"),
        (5, 6) => return String::from("g3"),
        (5, 7) => return String::from("h3"),
        (6, 0) => return String::from("a2"),
        (6, 1) => return String::from("b2"),
        (6, 2) => return String::from("c2"),
        (6, 3) => return String::from("d2"),
        (6, 4) => return String::from("e2"),
        (6, 5) => return String::from("f2"),
        (6, 6) => return String::from("g2"),
        (6, 7) => return String::from("h2"),
        (7, 0) => return String::from("a1"),
        (7, 1) => return String::from("b1"),
        (7, 2) => return String::from("c1"),
        (7, 3) => return String::from("d1"),
        (7, 4) => return String::from("e1"),
        (7, 5) => return String::from("f1"),
        (7, 6) => return String::from("g1"),
        (7, 7) => return String::from("h1"),
        _ => return String::from("ERR"),
    }
}

pub fn input_error(error: Error) {
    match error {
        Error::Empty => println!(
            "{} {}",
            Red.bold().paint(">>>"),
            "You can't move an empty tile!"
        ),
        Error::Length => println!(
            "{} {}",
            Red.bold().paint(">>>"),
            "Your input needs to be 4 chars long!"
        ),
        Error::IllegalMove => println!("{} {}", Red.bold().paint(">>>"), "Illegal move!"),
        Error::OutOfBounds => println!("{} {}", Red.bold().paint(">>>"), "Invalid choice!"),
        Error::EnemyMove => println!(
            "{} {}",
            Red.bold().paint(">>>"),
            "You can't move your opponent's piece!"
        ),
        Error::TeamDmg => println!(
            "{} {}",
            Red.bold().paint(">>>"),
            "You cannot attack your own piece!"
        ),

        Error::Check => println!(
            "{} {}",
            Red.bold().paint(">>>"),
            "You cannot move into check!"
        ),
    }
}

fn clear_draw(board: &mut Board) {
    clear_screen();
    board.draw_board();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn arrow_print(text: &str, bold: bool) {
    if bold {
        println!(
            "{} {}",
            RGB(80, 80, 80).bold().paint(">>>"),
            White.bold().paint(text)
        );
    } else {
        println!("{} {}", RGB(80, 80, 80).paint(">>>"), text);
    }
}
