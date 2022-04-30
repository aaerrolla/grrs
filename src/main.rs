// feature-3 - using BufReader
// using clap  https://docs.rs/clap/   to handle command line arguments
// clap supports subcommands, shell completions and good help messages

// Exercise for the reader: This is not the best implementation: 
// It will read the whole file into memory – however large the file may be. 
// Find a way to optimize it! 
// (One idea might be to use a BufReader (https://doc.rust-lang.org/1.39.0/std/io/struct.BufReader.html) instead of read_to_string().)

use clap::Parser;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::fs::File;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattren to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();   
   
    // use BufReader
    let file = File::open(&args.path);
    match file {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut line =  String::new();
            while reader.read_line(&mut line).expect("unable to read") != 0 {
                if line.contains(&args.pattern) {
                    println!("{}", line );
                }
                line.clear();
            }
        }
        Err(error) => {
            println!("Error {} ", error);    
        }
    }
}