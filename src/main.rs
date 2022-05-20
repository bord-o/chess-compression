mod helpers;
mod huff;
use itertools::Itertools;

fn main() {
    let mut db_index: Vec<u16> = Vec::new();
    let games = helpers::extract_games("/home/bordo/chess-compression/modern35.uci");
    let mut g_counter = 0;
    for g in &games {
        println!("Encoding game: {}\n{}", g_counter, g);
        db_index.extend(helpers::index_game(&g));
        g_counter += 1;
    }
    // println!("{:?}, {}", db_index, db_index.len());

    let frequencies = huff::generate_huff_weights(&db_index);
    let (book, tree) = huff::huff_gen(frequencies);
    huff::encode("/home/bordo/chess-compression/etest.txt", &db_index, book);

    let decoded_indexes: Vec<u16> = huff::decode("/home/bordo/chess-compression/etest.txt", tree);
    let deindexed = helpers::deindex_game(decoded_indexes);

    print!("{}", deindexed);

    // for num in t.keys().sorted() {
    //     println!("({:?}, {:?})", num, t[num]);
    // }
    //println!("{:?}", t);

    // std::fs::write("/usr/home/bordo/chess-compression/temp.txt", freqs)
    //     .expect("Unable to write file");

    //print!("{:?}", db_index);
    /*
    helpers::write_indexes(
        "/usr/home/bordo/chess-compression/moderndb_res.csv",
        &db_index,
    );
    */
    //print!("{:?}", game_1_moves);

    // sort_legal_moves(game_board);

    // print!("{}", buf);
}
//let encoded = encoder::test(&test_index);
