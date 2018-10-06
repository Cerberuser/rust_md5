extern crate rust_md5_updated;

use rust_md5_updated::{md5, util};

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn process_input<R>(filename: &str, input: R)
    where R: Read
{
    let input = io::BufReader::new(input);

    let input_iter = input.bytes().map(|res| res.unwrap());

    let hash_vec = &mut Vec::new();
    md5::hash(input_iter, hash_vec);

    let mut out = io::stdout();
    out.write(&util::to_hex_string(hash_vec).into_bytes()).unwrap();
    out.write(&format!(" {}\n", filename).into_bytes()).unwrap();
    out.flush().unwrap();
}

fn main() {
    let mut args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        process_input(&"-".to_string(), io::stdin());
    } else {
        let filename = args.remove(1);

        if filename == "-" {
            process_input(&filename, io::stdin());
        } else {
            process_input(&filename, File::open(&filename).unwrap());
        };
    }
}
