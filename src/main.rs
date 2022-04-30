// feature-2
// using clap  https://docs.rs/clap/   to handle command line arguments
// clap supports subcommands, shell completions and good help messages

use clap::Parser;

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
    let content = std::fs::read_to_string(&args.path);
    match content {
        Ok(content) => {
            for line in content.lines() {
                if line.contains(&args.pattern) {
                    println!("{}", line );
                }
            }
        
        }
        Err(error) => {
            println!("Error {} ", error);
        }
    }
           
}