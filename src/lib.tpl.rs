/// This file is automatically generated by `build.rs`. Any changes made here will be
/// lost forever.
///
/// Any modification must be done in `lib.tpl.rs` and it will be compiled automatically
/// by build.rs
use crate::{stopwords::StopwordsTrait, stemmer::StemmerTrait};
use rust_stemmers::{Algorithm, Stemmer};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, str::FromStr};
use stopwords::{Stopwords, NLTK};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use textcat::category::Categories;

#[derive(Clone, Debug, EnumIter, Eq, Display, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Language {
    {% for lang in languages %}
        {{lang|capitalize}},{% endfor %}
}

impl Language {
    pub fn all() -> HashSet<Language> {
        Language::iter().collect()
    }
}

impl StopwordsTrait for Language {
    fn stopwords(&self) -> Option<&'static [&'static str]> {
        match self {
        {% for lang in stopwords %}
            Self::{{lang}} => NLTK::stopwords(stopwords::Language::{{lang}}),{% endfor %}
            _ => None
        }
    }
}

impl StemmerTrait for Language {
    fn stemmer(&self) -> Option<Stemmer> {
        match self {
        {% for lang in stemmers %}
            Self::{{lang}} => Some(Stemmer::create(Algorithm::{{lang}})),{% endfor %}
            _ => None
        }

    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(name: &str) -> Result<Language, String> {
        match name.to_lowercase().as_str() {
        {% for lang in languages %}
            "{{lang}}" => Ok(Self::{{lang|capitalize}}),{% endfor %}
        _ => Err("Invalid argument".to_string()),
        }
    }
}

pub struct Lingo {
    inner: Categories<Language>,
}

#[allow(clippy::new_without_default)]
impl Lingo {
    pub fn new() -> Self {
        Lingo {
            inner: Self::get_embed_languages(),
        }
    }
    
    pub fn get_language(&self, sample: &str) -> Option<Language> {
        self.inner.get_category(sample)
    }

    pub fn get_languages(&self, sample: &str) -> Option<Vec<(Language, u64)>> {
        self.inner.get_categories(sample)
    }

    #[allow(clippy::invisible_characters)]
    pub fn get_embed_languages() -> Categories<Language> {
        let mut f: Categories<Language> = vec![
        {% for c in ngrams %}
            (
                Language::{{c.0|capitalize}},
                vec![
                {% for ngram in c.1|slice(end=400) %}
                    "{{ngram}}",{% endfor %}
                ]
            ),{% endfor %}
        ].into();

        // The more languages we support the less lower the threshold needs to be
        let _ = f.set_threshold(0.01);

        f
    }
}

#[cfg(test)]
mod test {
    use crate::{Language, Lingo, Stopwords, Stemmer};

    #[test]
    fn test_english_stopwords() {
        let stopwords = Language::English.stopwords();
        assert_eq!(true, stopwords.is_some());
        assert_eq!(true, 100 < stopwords.expect("stopwords").len());
    }

    #[test]
    fn test_guarani_stopwords() {
        let stopwords = Language::Guarani.stopwords();
        assert_eq!(true, stopwords.is_none());
    }

    #[test]
    fn test_english_stemmer() {
        let stopwords = Language::English.stemmer();
        assert_eq!(true, stopwords.is_some());
    }

    #[test]
    fn test_guarani_stemmer() {
        let stopwords = Language::Guarani.stemmer();
        assert_eq!(true, stopwords.is_none());
    }

    fn test_expected_language(l: Lingo, sample: &str, expected: Language) {
        if let Some(language) = l.get_language(sample) {
            assert_eq!(
                expected,
                language
            );
        } else {
            panic!(
                "{} -> {}",
                sample,
                if let Some(candidates) = l.get_languages(sample) {
                    candidates.iter().map(|l| l.0.to_string()).collect::<Vec<String>>().join(", ")
                } else {
                    "no candidate".into()
                }
            );
        }
    }

    {% for test in tests %}
        #[test]
        fn test_{{test['category']}}_from_str() {
            assert_eq!(Language::{{test['category']|capitalize}}.to_string(), "{{test['category']|capitalize}}".to_owned());
        }

        {% set i = loop.index %}
        {% for text in test['fixtures'] %}
        #[test]
        fn test_{{test['category']}}_{{loop.index}}() {
            test_expected_language(
                Lingo::new(),
                "{{text|addslashes}}",
                Language::{{test['category']|capitalize}}
            );
        }
        {% endfor %}
    {% endfor %}
}
