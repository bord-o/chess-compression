use itertools::Itertools;
use pleco::tools::eval::Eval;
use pleco::{BitMove, Board};
use std::collections::HashMap;
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

pub fn write_indexes(write_path: &str, indexes: &Vec<u8>) {
    let data = format!("{:?}", indexes);
    fs::write(write_path, data).expect("Unable to write file");
}

pub fn extract_moves(uci_game: &String) -> Vec<String> {
    uci_game.split(" ").map(str::to_string).collect()
}

pub fn index_game(uci_game: &String) -> Vec<u8> {
    // loops through uci formatted move list representing a game
    // and returns a vector with the played move's index into the
    // sorted engine evaluation results
    let mut board = Board::start_pos();
    let mut indexes: Vec<u8> = Vec::new();
    let moves = extract_moves(uci_game);
    for m in moves {
        if m.starts_with("1") || m.starts_with("0") {
            break;
            // make sure the game is not over e.g. "1-0" or
            // "0-1" as the current move
        }

        let current_evals = sort_legal_moves(&board);
        for e in 0..current_evals.len() {
            if current_evals[e].bitmove.stringify() == m.to_ascii_lowercase() {
                indexes.push(e as u8);
            }
        }
        board.apply_uci_move(&m.to_ascii_lowercase());
        //println!("{}", m);
    }
    return indexes;
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

pub fn generate_huff_weights(index_db: Vec<u8>) -> HashMap<u8, usize> {
    let weights = index_db.into_iter().counts();
    //println!("{:?}", &weights);
    return weights;
}
