mod moves;
mod types;
use ansi_term::Colour::{Red, White, RGB};
use moves::*;
use std::io::{stdin, Write};
use types::*;

fn main() {
    let mut board = Board::new();
    arrow_print("Welcome to C-Chess!");

    stdin().read_line(&mut String::new()).unwrap();

    clear_draw(&mut board);
    player_input(&mut board);
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn arrow_print(text: &str) {
    println!("{} {}", RGB(80, 80, 80).bold().paint(">>>"), text);
}

fn player_input(board: &mut Board) {
    'initial: loop {
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

        if board.tiles[from.0][from.1].piece.colour != Colour::White
            && board.tiles[from.0][from.1].piece.piece_type != Type::Empty
        {
            clear_draw(board);
            input_error(Error::EnemyMove);
            continue;
        }

        if board.tiles[to.0][to.1].piece.colour == Colour::White
            && board.tiles[to.0][to.1].piece.piece_type != Type::Empty
        {
            clear_draw(board);
            input_error(Error::TeamDmg);
            continue;
        }

        let white_moves = legal_moves(board, from, true);
        if white_moves.contains(&to) {
            match move_piece(board, from, to, true) {
                Err(e) => {
                    clear_draw(board);
                    input_error(e);
                    continue 'initial;
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
    let mut output = String::new();
    match input {
        (0, 0) => output.push_str("a1"),
        (1, 0) => output.push_str("a2"),
        (2, 0) => output.push_str("a3"),
        (3, 0) => output.push_str("a4"),
        (4, 0) => output.push_str("a5"),
        (5, 0) => output.push_str("a6"),
        (6, 0) => output.push_str("a7"),
        (7, 0) => output.push_str("a8"),
        (0, 1) => output.push_str("b1"),
        (1, 1) => output.push_str("b2"),
        (2, 1) => output.push_str("b3"),
        (3, 1) => output.push_str("b4"),
        (4, 1) => output.push_str("b5"),
        (5, 1) => output.push_str("b6"),
        (6, 1) => output.push_str("b7"),
        (7, 1) => output.push_str("b8"),
        (0, 2) => output.push_str("c1"),
        (1, 2) => output.push_str("c2"),
        (2, 2) => output.push_str("c3"),
        (3, 2) => output.push_str("c4"),
        (4, 2) => output.push_str("c5"),
        (5, 2) => output.push_str("c6"),
        (6, 2) => output.push_str("c7"),
        (7, 2) => output.push_str("c8"),
        (0, 3) => output.push_str("d1"),
        (1, 3) => output.push_str("d2"),
        (2, 3) => output.push_str("d3"),
        (3, 3) => output.push_str("d4"),
        (4, 3) => output.push_str("d5"),
        (5, 3) => output.push_str("d6"),
        (6, 3) => output.push_str("d7"),
        (7, 3) => output.push_str("d8"),
        (0, 4) => output.push_str("e1"),
        (1, 4) => output.push_str("e2"),
        (2, 4) => output.push_str("e3"),
        (3, 4) => output.push_str("e4"),
        (4, 4) => output.push_str("e5"),
        (5, 4) => output.push_str("e6"),
        (6, 4) => output.push_str("e7"),
        (7, 4) => output.push_str("e8"),
        (0, 5) => output.push_str("f1"),
        (1, 5) => output.push_str("f2"),
        (2, 5) => output.push_str("f3"),
        (3, 5) => output.push_str("f4"),
        (4, 5) => output.push_str("f5"),
        (5, 5) => output.push_str("f6"),
        (6, 5) => output.push_str("f7"),
        (7, 5) => output.push_str("f8"),
        (0, 6) => output.push_str("g1"),
        (1, 6) => output.push_str("g2"),
        (2, 6) => output.push_str("g3"),
        (3, 6) => output.push_str("g4"),
        (4, 6) => output.push_str("g5"),
        (5, 6) => output.push_str("g6"),
        (6, 6) => output.push_str("g7"),
        (7, 6) => output.push_str("g8"),
        (0, 7) => output.push_str("h1"),
        (1, 7) => output.push_str("h2"),
        (2, 7) => output.push_str("h3"),
        (3, 7) => output.push_str("h4"),
        (4, 7) => output.push_str("h5"),
        (5, 7) => output.push_str("h6"),
        (6, 7) => output.push_str("h7"),
        (7, 7) => output.push_str("h8"),
        _ => output.push_str("ERR"),
    }

    return output;
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
