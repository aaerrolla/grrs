// feature-7 - providing context in the logs
// using  anyhow library to retain the original error and trace information 

use clap::Parser;
use anyhow::{Context, Result};

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
    handle_cmd_with_return_shortcut().expect("Error reading ");  
}


///  we declare this function return type has Result///  
///  using shortcut form ?  to return from Err
fn handle_cmd_with_return_shortcut() -> Result<()> {
    let args = Cli::parse();   

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;

    for line  in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}" , line);
        }
    }
    Ok(())
}