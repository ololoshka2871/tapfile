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
    #[clap(short('b'), long, default_value = "65535")]
    block_size: usize,

    /// Output file
    output_tap: Option<PathBuf>,

    /// Input file
    input: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    println!(
        "Converting {:?} to {:?}, block size: {}",
        args.input, args.output_tap, args.block_size
    );

    let (input, mut output): (Box<dyn Read>, Box<dyn Write>) =
        if let Some(output) = &args.output_tap {
            let output = Box::new(std::fs::File::create(&output).unwrap());
            let input = args
                .input
                .map(|f| Box::new(std::fs::File::open(&f).unwrap()) as Box<dyn Read>)
                .unwrap_or(Box::new(std::io::stdin()));
            (input, output)
        } else {
            let output = Box::new(std::io::stdout());
            let input = Box::new(std::io::stdin());
            (input, output)
        };

    let iterator = tapfile::TapWriter::new(input, args.block_size);

    for (block, info) in iterator {
        println!("Block {:?}", info);
        output.write(&block).unwrap();
    }
}
