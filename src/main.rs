use lingo::Lingo;
use std::io;

fn main() {
    let lingo = Lingo::new();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from pipe");

    let lang = match lingo.get_language(&input) {
        Some(l) => l.to_string(),
        _ => "Unknown".to_string(),
    };

    println!("Language: {}", lang);
    println!("Input text: {}", input);
}
