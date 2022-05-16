mod encoder;
mod helpers;

fn main() {
    let mut db_index: Vec<u8> = Vec::new();
    let games = helpers::extract_games("/home/bordo/chess-compression/modern35.uci");
    let mut g_counter = 0;
    for g in &games {
        println!("Encoding game: {}", g_counter);
        db_index.extend(helpers::index_game(&g));
        g_counter += 1;
    }

    //let test_index: Vec<u8> = vec![0, 2, 14, 13, 0, 0];
    //let encoded = encoder::test(&test_index);
    //let t = helpers::generate_huff_weights(db_index);
    //let freqs = format!("{:?}", t);
    encoder::test(&db_index);

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
