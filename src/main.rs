use pleco::tools::eval::Eval;
use pleco::Board;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

struct MoveEval {
    uci: String,
    eval: i32,
}

impl MoveEval {
    pub fn new(uci: String, eval: i32) -> Self {
        MoveEval { uci, eval }
    }
}

impl std::fmt::Display for MoveEval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}", self.uci, self.eval)
    }
}

fn main() {
    let move_db_path = Path::new("/home/bordo/chessdb/testing/1kgames_test.txt");

    let mut b = Board::start_pos();
    let legals = b.generate_moves();
    let movesvec = legals.vec();
    let mut evals = Vec::new();
    for m in movesvec.into_iter() {
        b.apply_move(m);
        let score = Eval::eval_low(&b);
        // println!("{}, {}", m, score);
        let current_eval = MoveEval::new(m.stringify(), -score);
        evals.push(current_eval);
        b.undo_move()
    }

    evals.sort_by_key(|e| e.eval);
    evals.reverse();

    let mut move_db = File::open(move_db_path).expect("file not opened");
    let mut buf = String::new();
    let x = move_db.read_to_string(&mut buf).expect("cant read");
    // print!("{}", buf);

    for e in evals {
        println!("{}", e);
    }
}
