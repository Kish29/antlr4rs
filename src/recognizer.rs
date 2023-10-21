pub trait Recognizer<'a> {
    fn literal_names(&self) -> &'static [&'static str];

    fn rule_name(&self) -> &'static [&'static str];



}