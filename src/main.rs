fn main() {
    let pattern = std::env::args().nth(1)
        .expect("no pattern given");
    let path = std::env::args().nth(2)
        .expect("no path given");
    let content = std::fs::read_to_string(&path)
        .expect("could not read file");

    for line in content.lines() {
        if line.contains(&pattern) {
            println!("{}", line );
        }
    }
}