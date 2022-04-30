// feature-4 - error handling 
// using match with  panic! 
// using wnwrap() shortcut

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
   
    // how to handle error
    // function like read_to_string returns Result enum 
    let result = std::fs::read_to_string(&args.path);
    match result {
        Ok(content) => { 
            println!("File content: {}" , content);
        }
        Err(error) => {
            println!(" There is error {}", error);
        }
    }


    // if we want to access content outside of the match 
    // we wont be able to do so 
    // to fix this match  has to return the same type from  both branches  Ok and Err 
    // it is easy to  return from Ok  branch  , but difficult to deal with Err case
    //  to hadle error case , we just exist the program flow and do not return anything to caller

    let result1 = std::fs::read_to_string(&args.path);
    let content1 = match result1 {
        Ok(content) => {
            content
        }
        Err(error) => {
            panic!("Can't deal with {}, just exist here" , error);
        }
    };  

    // if above match result in error program will exit and we never reach this line  
    // aslso it makes sense , since we can't to wihout reading file 
    print!("file content: {}", content1);

    //  there is shortcut for this kind of situation where when error condition we panic
    // using unwrap  
    // above lines 37  to 45  are reduced to one line with unwrap()
    
    let content2 = std::fs::read_to_string(&args.path).unwrap();
    print!("file content: {}", content2);
}