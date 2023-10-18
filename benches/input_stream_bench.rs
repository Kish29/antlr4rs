use std::io::BufRead;
use criterion::{Criterion, criterion_group, criterion_main};
use antlr4rs::char_stream::CharStream;
use antlr4rs::input_stream::InputStream;


fn create_input_stream() {
    let mut input = InputStream::new(r#"A你4好§，\❤"#);
    let input = &mut input as &mut dyn CharStream<&str>;
    assert_eq!(input.size(), 8);
    input.consume();
    input.consume();
    input.consume();
    input.consume();
    input.consume();
    input.consume();
}

fn bench_input_stream(c: &mut Criterion) {
    c.bench_function("create input stream", |b| b.iter(|| create_input_stream()));
}

criterion_group!(benches, bench_input_stream);
criterion_main!(benches);