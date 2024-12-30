use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", path);

    let content = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    println!("Content: \n{}", content);
}
