use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tera::{Context, Tera};
use textcat::storage::learn_from_directory;

#[derive(Deserialize, Serialize)]
struct Fixture {
    category: String,
    fixtures: Vec<String>,
}

fn main() {
    if Path::new("./src/generated.rs").exists() {
        return ();
    }
    // train TextCat with all samples
    let _p = learn_from_directory("./fixtures").unwrap();
    let mut tera = Tera::default();

    // load all tests
    let file = File::open("./tests/fixtures.json").unwrap();
    let reader = BufReader::new(file);
    let tests: Vec<Fixture> = serde_json::from_reader(reader).unwrap();

    let mut f = File::open("src/lib.tpl.rs").unwrap();
    let mut code = String::new();
    f.read_to_string(&mut code).unwrap();

    tera.add_raw_template("tpl", &code).unwrap();

    let mut context = Context::new();
    context.insert("ngrams", &_p.to_vec());
    context.insert("languages", &_p.categories());
    context.insert("version", &env!("CARGO_PKG_VERSION").to_string());
    context.insert("tests", &tests);

    File::create("src/generated.rs")
        .unwrap()
        .write_all(tera.render("tpl", &context).unwrap().as_bytes())
        .unwrap();

    Command::new("cargo").arg("fmt").output().unwrap();
}
