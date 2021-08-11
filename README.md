# Lingo

N-Gram-Based natural language detection library.

## Usage

### Language detection

```rust
use lingo::Lingo;

fn main() {
    let textcat  = Lingo::new();
    let text     = "Hi there, this is a simple text written in what language?";
    let language = textcat.get_language(text).unwrap();

    println!("\"{}\" is written in \"{}\"", text, language);
}
```

### Stopwords and Stemmers

Lingo provides stopwords and stemmers for some languages by wrapping third party libraries.

The usage is quite simple.

```rust
use lingo::Language;

fn main() {
    let stopwords = Language::English.stopwords()?;
    let stemmer   = Language::English.stemmer()?;
}
```
