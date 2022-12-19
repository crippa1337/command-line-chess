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

fn arrow_print(text: &str) {
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

        let legal_moves = legal_moves(board, from, true);
        if legal_moves.contains(&to) {
            move_piece(board, from, to);
            clear_draw(board);
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
        "a1" => (0, 0),
        "a2" => (1, 0),
        "a3" => (2, 0),
        "a4" => (3, 0),
        "a5" => (4, 0),
        "a6" => (5, 0),
        "a7" => (6, 0),
        "a8" => (7, 0),
        "b1" => (0, 1),
        "b2" => (1, 1),
        "b3" => (2, 1),
        "b4" => (3, 1),
        "b5" => (4, 1),
        "b6" => (5, 1),
        "b7" => (6, 1),
        "b8" => (7, 1),
        "c1" => (0, 2),
        "c2" => (1, 2),
        "c3" => (2, 2),
        "c4" => (3, 2),
        "c5" => (4, 2),
        "c6" => (5, 2),
        "c7" => (6, 2),
        "c8" => (7, 2),
        "d1" => (0, 3),
        "d2" => (1, 3),
        "d3" => (2, 3),
        "d4" => (3, 3),
        "d5" => (4, 3),
        "d6" => (5, 3),
        "d7" => (6, 3),
        "d8" => (7, 3),
        "e1" => (0, 4),
        "e2" => (1, 4),
        "e3" => (2, 4),
        "e4" => (3, 4),
        "e5" => (4, 4),
        "e6" => (5, 4),
        "e7" => (6, 4),
        "e8" => (7, 4),
        "f1" => (0, 5),
        "f2" => (1, 5),
        "f3" => (2, 5),
        "f4" => (3, 5),
        "f5" => (4, 5),
        "f6" => (5, 5),
        "f7" => (6, 5),
        "f8" => (7, 5),
        "g1" => (0, 6),
        "g2" => (1, 6),
        "g3" => (2, 6),
        "g4" => (3, 6),
        "g5" => (4, 6),
        "g6" => (5, 6),
        "g7" => (6, 6),
        "g8" => (7, 6),
        "h1" => (0, 7),
        "h2" => (1, 7),
        "h3" => (2, 7),
        "h4" => (3, 7),
        "h5" => (4, 7),
        "h6" => (5, 7),
        "h7" => (6, 7),
        "h8" => (7, 7),
        _ => (99, 99),
    };

    let to: (usize, usize) = match to.as_str() {
        "a1" => (0, 0),
        "a2" => (1, 0),
        "a3" => (2, 0),
        "a4" => (3, 0),
        "a5" => (4, 0),
        "a6" => (5, 0),
        "a7" => (6, 0),
        "a8" => (7, 0),
        "b1" => (0, 1),
        "b2" => (1, 1),
        "b3" => (2, 1),
        "b4" => (3, 1),
        "b5" => (4, 1),
        "b6" => (5, 1),
        "b7" => (6, 1),
        "b8" => (7, 1),
        "c1" => (0, 2),
        "c2" => (1, 2),
        "c3" => (2, 2),
        "c4" => (3, 2),
        "c5" => (4, 2),
        "c6" => (5, 2),
        "c7" => (6, 2),
        "c8" => (7, 2),
        "d1" => (0, 3),
        "d2" => (1, 3),
        "d3" => (2, 3),
        "d4" => (3, 3),
        "d5" => (4, 3),
        "d6" => (5, 3),
        "d7" => (6, 3),
        "d8" => (7, 3),
        "e1" => (0, 4),
        "e2" => (1, 4),
        "e3" => (2, 4),
        "e4" => (3, 4),
        "e5" => (4, 4),
        "e6" => (5, 4),
        "e7" => (6, 4),
        "e8" => (7, 4),
        "f1" => (0, 5),
        "f2" => (1, 5),
        "f3" => (2, 5),
        "f4" => (3, 5),
        "f5" => (4, 5),
        "f6" => (5, 5),
        "f7" => (6, 5),
        "f8" => (7, 5),
        "g1" => (0, 6),
        "g2" => (1, 6),
        "g3" => (2, 6),
        "g4" => (3, 6),
        "g5" => (4, 6),
        "g6" => (5, 6),
        "g7" => (6, 6),
        "g8" => (7, 6),
        "h1" => (0, 7),
        "h2" => (1, 7),
        "h3" => (2, 7),
        "h4" => (3, 7),
        "h5" => (4, 7),
        "h6" => (5, 7),
        "h7" => (6, 7),
        "h8" => (7, 7),
        _ => (99, 99),
    };

    return (from, to);
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
    }
}

fn clear_draw(board: &mut Board) {
    clear_screen();
    board.draw_board();
}
