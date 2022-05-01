// feature-5 - error handling 
// in this demo we will 
// 1 instead of  panic!  or  unwrap()  to handle error condition 
// 2  we can return Err from function
// 3 using shortcut ?  to return Err

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
    
    // handle_cmd_wihout_return();    
    // handle_cmd_with_return().expect("Error reading ");  
    handle_cmd_with_return_shortcut().expect("Error reading ");  
}

// fn handle_cmd_wihout_return() {
//     let args = Cli::parse();   

//     let result = std::fs::read_to_string(&args.path);
//     let content = match result {
//         Ok(content) => {
//             content
//         },
//         Err(error) => {
//             return Err(error.into());
//         }
//     }; 
//     println!("file content: {}", content);
//     Ok(()) 
// }

///  we declare this function return type has Result
///  and return Error  and  Ok 
fn handle_cmd_with_return() -> Result<(), Box<dyn std::error::Error> > {
    let args = Cli::parse();   

    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => {
            content
        },
        Err(error) => {
            return Err(error.into());
        }
    }; 
    println!("file content: {}", content);
    Ok(()) 
}

///  we declare this function return type has Result///  
///  using shortcut form ?  to return from Err
fn handle_cmd_with_return_shortcut() -> Result<(), Box<dyn std::error::Error> > {
    let args = Cli::parse();   

    let content = std::fs::read_to_string(&args.path)?;
    println!("file content: {}", content);
    Ok(())
}