pub trait Recognizer {
    fn literal_names(&self) -> &[&str];

    fn rule_names(&self) -> &[&str];
}

pub struct BaseRecognizer {}