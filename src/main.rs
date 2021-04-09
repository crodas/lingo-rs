use lingo::Lingo;
use std::io;

fn main() {
    let lingo = Lingo::new();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from pipe");

    if let Some(lang) = lingo.get_language(&input) {
        println!("Language: {}", lang);
    } else {
        println!("Language: Unknown");
    }

    println!("Input text: {}", input);
}
