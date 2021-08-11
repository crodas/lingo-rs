/// If this module is required the stopwords() method will be added
/// to each language. This function will return a list of the most common
/// words in each language or an empty HashSet if it is not supported by the
/// 'stopwords' crate.
pub trait StopwordsTrait {
    fn stopwords(&self) -> Option<&'static [&'static str]>;
}
