mod helpers;
mod huff;
use std::{env, fs};

use huffman_compress::Book;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = "usage: exe [compress | decompress] input_filename output_filename";
    // the option for the user to train on a different set of data (I have a default set of weights hard-coded)
    // let _frequencies = Some(huff::generate_huff_weights(&indexes));
    let (book, tree) = huff::huff_gen(None);
    match args.len() {
        4 => {
            let command = &args[1];
            let infile = &args[2];
            let outfile = &args[3];
            match command.as_str() {
                "compress" => {
                    println!("compressing {}", infile);
                    compress(infile, outfile, book);
                }

                "decompress" => {
                    println!("decompressing {}", infile);
                    decompress(infile, outfile, tree)
                }
                _ => {
                    println!("{}", usage)
                }
            }
        }
        _ => {
            println!("{}", usage)
        }
    }
}

fn decompress(in_file: &str, out_file: &str, tree: huffman_compress::Tree<u16>) {
    let decoded_indexes: Vec<u16> = huff::decode(in_file, tree);
    let deindexed = helpers::deindex_game(decoded_indexes);

    fs::write(out_file, deindexed).expect("Unable to write file");
}

fn compress(uci_filename: &str, out_filename: &str, book: Book<u16>) {
    let games = helpers::extract_games(uci_filename);
    let mut indexes: Vec<u16> = Vec::new();

    let mut g_counter = 0;
    for g in &games {
        // println!("Indexing game: {}", g_counter);
        indexes.extend(helpers::index_game(&g));
        g_counter += 1;
    }
    huff::encode(out_filename, &indexes, book);
}
