use std::{
    io::{Read, Write},
    path::PathBuf,
};

use clap::Parser;

/// Convert any file to .tap format
/// If no input file is specified, reads from stdin
/// If no output file is specified, writes to stdout
#[derive(Parser, Debug)]
struct Cli {
    /// Block size
    #[clap(short('b'), long, default_value = "8192")]
    block_size: usize,

    /// Output file
    output_tap: Option<PathBuf>,

    /// Input file
    input: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    eprintln!(
        "Converting {:?} to {:?}, block size: {}",
        args.input, args.output_tap, args.block_size
    );

    let (input, mut output): (Box<dyn Read>, Box<dyn Write>) =
        if let Some(outputf) = &args.output_tap {
            let output = Box::new(std::fs::File::create(&outputf).unwrap());
            let input = if let Some(f) = args.input {
                eprintln!("Convert from {f:?} to {outputf:?}");
                Box::new(std::fs::File::open(&f).unwrap()) as Box<dyn Read> 
            } else {
                eprintln!("Convert from stdin to {outputf:?}");
                Box::new(std::io::stdin())
            };
            (input, output)
        } else {
            eprintln!("Convert from stdin to stdout");
            let output = Box::new(std::io::stdout());
            let input = Box::new(std::io::stdin());
            (input, output)
        };

    let iterator = tapfile::TapWriter::new(input, args.block_size);

    for (block, info) in iterator {
        eprintln!("Block {:?}", info);
        output.write(&block).unwrap();
    }
}
