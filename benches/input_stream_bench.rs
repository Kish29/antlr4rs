use std::io::BufRead;
use criterion::{Criterion, criterion_group, criterion_main};
use antlr4rs::char_stream::CharStream;
use antlr4rs::input_stream::InputStream;


fn create_input_stream() {
    let is = &mut *InputStream::new("A你4好§，\\❤") as &mut dyn CharStream;
    is.consume();
    is.consume();
    is.consume();
    is.consume();
    is.consume();
    is.consume();
}

fn bench_input_stream(c: &mut Criterion) {
    c.bench_function("create input stream", |b| b.iter(|| create_input_stream()));
}

criterion_group!(benches, bench_input_stream);
criterion_main!(benches);