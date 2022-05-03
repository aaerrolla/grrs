// feature-6 - providing context in the logs
// The errors you get when using ? in your main function are okay, but they are not great. 
// For example: When you run std::fs::read_to_string("test.txt")? but the file test.txt doesn’t exist, you get this output:
// Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
// For example, we can create our own error type, and then use that to build a custom error message:
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
    handle_cmd_with_return_shortcut().expect("Error reading ");  
}


///  we declare this function return type has Result///  
///  using shortcut form ?  to return from Err
#[derive(Debug)]
struct CustomError(String);

fn handle_cmd_with_return_shortcut() -> Result<(), CustomError > {
    let args = Cli::parse();   

    let content = std::fs::read_to_string(&args.path)
        .map_err(|err| CustomError(format!("Error reading `{:?}`: {}", &args.path, err)))?;
    println!("file content: {}", content);
    Ok(())
}