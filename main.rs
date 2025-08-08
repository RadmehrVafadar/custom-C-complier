
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ./main <fileName>");
        return;
    }
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Could not read the file");



    println!("{contents}");
}
