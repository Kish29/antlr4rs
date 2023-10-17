pub trait Recognizer {
    fn literal_names(&self) -> &'static [&'static str];
}