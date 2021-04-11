use rust_stemmers::Stemmer;

pub trait StemmerTrait {
    fn stemmer(&self) -> Option<Stemmer>;
}
