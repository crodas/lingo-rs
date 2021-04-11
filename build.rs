use filetime::FileTime;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::process::Command;
use tera::{Context, Tera};
use textcat::storage::learn_from_directory;

#[derive(Deserialize, Serialize)]
struct Fixture {
    category: String,
    fixtures: Vec<String>,
}

fn mtime(path: &str) -> i64 {
    let metadata = fs::metadata(path).unwrap();
    let mtime = FileTime::from_last_modification_time(&metadata);
    mtime.unix_seconds()
}

fn get_latest_changed_file() -> i64 {
    let mut times: Vec<i64> = vec![];

    for p in glob("./fixtures/*.sample").unwrap() {
        times.push(mtime(p.unwrap().to_str().unwrap()));
    }

    for p in glob("./tests/*.json").unwrap() {
        times.push(mtime(p.unwrap().to_str().unwrap()));
    }

    for p in glob("./src/*.tpl.rs").unwrap() {
        times.push(mtime(p.unwrap().to_str().unwrap()));
    }

    if times.is_empty() {
        return 0;
    }

    times.sort_by(|a, b| b.cmp(&a));

    times[0]
}

fn main() {
    if mtime("./src/generated.rs") >= get_latest_changed_file() {
        // Nothing to update!
        return;
    }

    // train TextCat with all samples
    let _p = learn_from_directory("./fixtures").unwrap();
    let mut tera = Tera::default();

    // load all tests
    let file = File::open("./tests/fixtures.json").unwrap();
    let reader = BufReader::new(file);
    let tests: Vec<Fixture> = serde_json::from_reader(reader).unwrap();
    let stemmers = vec![
        "Arabic",
        "Danish",
        "Dutch",
        "English",
        "Finnish",
        "French",
        "German",
        "Greek",
        "Hungarian",
        "Italian",
        "Norwegian",
        "Portuguese",
        "Romanian",
        "Russian",
        "Spanish",
        "Swedish",
        "Tamil",
        "Turkish",
    ];

    let stopwords = vec![
        "Arabic",
        "Azerbaijani",
        "Danish",
        "Dutch",
        "English",
        "Finnish",
        "French",
        "German",
        "Greek",
        "Hungarian",
        "Italian",
        "Kazakh",
        //"Nepali",
        "Norwegian",
        "Portuguese",
        "Romanian",
        "Russian",
        "Spanish",
        "Swedish",
        "Turkish",
    ];

    let mut f = File::open("src/lib.tpl.rs").unwrap();
    let mut code = String::new();
    f.read_to_string(&mut code).unwrap();

    tera.add_raw_template("tpl", &code).unwrap();

    let mut context = Context::new();
    context.insert("ngrams", &_p.to_vec());
    context.insert("languages", &_p.categories());
    context.insert("version", &env!("CARGO_PKG_VERSION").to_string());
    context.insert("stemmers", &stemmers);
    context.insert("stopwords", &stopwords);
    context.insert("tests", &tests);

    File::create("src/generated.rs")
        .unwrap()
        .write_all(tera.render("tpl", &context).unwrap().as_bytes())
        .unwrap();

    Command::new("cargo").arg("fmt").output().unwrap();
}
