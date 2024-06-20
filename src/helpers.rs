use pleco::tools::eval::Eval;
use pleco::{BitMove, Board};
use std::fs;

#[derive(Debug)]
pub struct MoveEval {
    bitmove: BitMove,
    eval: i32,
}

impl MoveEval {
    pub fn new(bitmove: BitMove, eval: i32) -> Self {
        MoveEval { bitmove, eval }
    }
}

impl std::fmt::Display for MoveEval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}", self.bitmove, self.eval)
    }
}

pub fn extract_games(db_path: &str) -> Vec<String> {
    let buf = fs::read_to_string(db_path).expect("couldnt read");

    // print!("{}", &buf);
    let x: Vec<String> = buf.split("\n\n").map(str::to_string).collect();
    // print!("{}", x[1]);
    return x;
}

pub fn write_indexes(write_path: &str, indexes: &Vec<u16>) {
    let data = format!("{:?}", indexes);
    fs::write(write_path, data).expect("Unable to write file");
}

pub fn extract_moves(uci_game: &String) -> Vec<String> {
    uci_game.split(" ").map(str::to_string).collect()
}

pub fn index_game(uci_game: &String) -> Vec<u16> {
    // loops through uci formatted move list representing a game
    // and returns a vector with the played move's index into the
    // sorted engine evaluation results
    let mut board = Board::start_pos();
    let mut indexes: Vec<u16> = Vec::new();
    let moves = extract_moves(uci_game);
    for m in moves {
        // make sure the game is not over e.g. "1-0" or
        // "0-1" as the current move
        if m.starts_with("1-0") {
            indexes.push(400 as u16);
            break;
        }

        if m.starts_with("0-1") {
            indexes.push(500 as u16);
            break;
        }

        if m.starts_with("1/2-1/2") {
            indexes.push(600 as u16);
            break;
        }

        let current_evals = sort_legal_moves(&board);
        for e in 0..current_evals.len() {
            if current_evals[e].bitmove.stringify() == m.to_ascii_lowercase() {
                indexes.push(e as u16);
            }
        }
        board.apply_uci_move(&m.to_ascii_lowercase());
        //println!("{}", m);
    }
    return indexes;
}

pub fn deindex_game(indexes: Vec<u16>) -> String {
    let mut board = Board::start_pos();
    let mut moves: Vec<String> = Vec::new();
    let mut current_move = String::new();
    let mut game_counter = 0;

    for i in indexes {
        //println!("{}", i);
        if i == 400 {
            current_move = String::from("1-0");
            moves.push(format!("{}\n\n", current_move.clone()));
            game_counter += 1;
            board = Board::start_pos();
        }

        if i == 500 {
            current_move = String::from("0-1");
            moves.push(format!("{}\n\n", current_move.clone()));
            game_counter += 1;
            board = Board::start_pos();
        }

        if i == 600 {
            current_move = String::from("1/2-1/2");
            moves.push(format!("{}\n\n", current_move.clone()));
            game_counter += 1;
            board = Board::start_pos();
        }

        let current_evals = sort_legal_moves(&board);
        for e in 0..current_evals.len() {
            if e as u16 == i {
                current_move = current_evals[e].bitmove.stringify();
                moves.push(current_move.clone() + " ");
            }
        }
        board.apply_uci_move(&current_move);
    }
    let formatted = moves.join("").trim_start().to_string();
    return formatted;
}

pub fn sort_legal_moves(ref_board: &Board) -> Vec<MoveEval> {
    let mut board = ref_board.shallow_clone();
    let legals = board.generate_moves();
    let mut evals: Vec<MoveEval> = Vec::new();

    for m in legals {
        board.apply_move(m);
        let eval = Eval::eval_low(&board);
        evals.push(MoveEval::new(m, -eval));
        board.undo_move();
    }

    evals.sort_by_key(|e| e.eval);
    evals.reverse();

    return evals;
}
