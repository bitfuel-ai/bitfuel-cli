#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    add: String,
    /// The path to the file to read
    description: String,
}

fn main() {
    let args = Cli::parse();
}