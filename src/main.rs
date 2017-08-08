use std::env;
use std::fs::File;
use std::io::prelude::*;


fn match_token(input: &String) -> String {
    let first = input.chars().nth(0).unwrap();
    println!("First: {}\n", first);
    return String::from("Str");
}


fn lex(input: &String) -> Vec<String> {
    let mut tokens: Vec<String> = vec!();
    tokens.push(match_token(input));
    return tokens;
}


fn build_file(filename: &String) {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("error reading contents");

    println!("Contents:\n{}", contents);
    lex(&contents);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let files = &args[1 ..];
    for f in files {
        build_file(f);
    }
}
