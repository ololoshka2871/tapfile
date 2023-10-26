use std::{
    io::{Read, Write},
    path::PathBuf,
};

use clap::Parser;

/// Convert .tap file to any file
/// If no output file is specified, writes to stdout
/// If no input file is specified, reads from stdin
#[derive(Parser, Debug)]
struct Cli {
    /// Output file
    output: Option<PathBuf>,

    /// Input file
    input_tap: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    eprintln!("Converting {:?} to {:?}", args.input_tap, args.output);

    let (input, mut output): (Box<dyn Read>, Box<dyn Write>) = if let Some(outputf) = &args.output {
        let output = Box::new(std::fs::File::create(&outputf).unwrap());
        let input = if let Some(f) = args.input_tap {
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

    let iterator = tapfile::TapReader::new(input);

    for (block, info) in iterator {
        eprintln!("Block {:?}", info);
        output.write(&block).unwrap();
    }
}
