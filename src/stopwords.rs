use crate::generated::Language;
use std::collections::HashSet;
use std::str::FromStr;
use stopwords::{Stopwords, NLTK};

/// If this module is required the stopwords() method will be added
/// to each language. This function will return a list of the most common
/// words in each language or an empty HashSet if it is not supported by the
/// 'stopwords' crate.
pub trait StopwordsTrait {
    fn stopwords(&self) -> HashSet<String>;
}

impl StopwordsTrait for Language {
    fn stopwords(&self) -> HashSet<String> {
        if let Ok(language) = stopwords::Language::from_str(self.name()) {
            return NLTK::stopwords(language)
                .unwrap()
                .iter()
                .map(|r| <&str>::clone(r).to_string())
                .collect();
        }

        HashSet::new()
    }
}

mod test {
    #[allow(unused_imports)]
    use crate::{Language, Stopwords};

    #[test]
    fn test_english_stopwords() {
        let stopwords = Language::English.stopwords();
        assert_eq!(true, 100 < stopwords.len());
    }

    #[test]
    fn test_guarani_stopwords() {
        let stopwords = Language::Guarani.stopwords();
        assert_eq!(0, stopwords.len());
    }
}
