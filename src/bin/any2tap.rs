use std::{io::Write, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    /// Input file
    input: PathBuf,

    /// Output file
    output_tap: PathBuf,

    /// Block size
    #[clap(short('b'), long, default_value = "65535")]
    block_size: usize,
}

fn main() {
    let args = Cli::parse();

    println!(
        "Converting {:?} to {:?}, block size: {}",
        args.input, args.output_tap, args.block_size
    );

    let input = std::fs::File::open(&args.input).unwrap();
    let mut output = std::fs::File::create(&args.output_tap).unwrap();

    let iterator = tapfile::TapWriter::new(input, args.block_size);

    for (block, info) in iterator {
        println!("Block {:?}", info);
        output.write(&block).unwrap();
    }
}
