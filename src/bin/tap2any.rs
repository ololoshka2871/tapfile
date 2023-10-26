use std::{io::Write, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    /// Input file
    input_tap: PathBuf,

    /// Output file
    output: PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("Converting {:?} to {:?}", args.input_tap, args.output);

    let input = std::fs::File::open(&args.input_tap).unwrap();
    let mut output = std::fs::File::create(&args.output).unwrap();

    let iterator = tapfile::TapReader::new(input);

    for (block, info) in iterator {
        println!("Block {:?}", info);
        output.write(&block).unwrap();
    }
}
