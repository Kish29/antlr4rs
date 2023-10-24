use criterion::{Criterion, criterion_group, criterion_main};
use antlr4rs::char_stream::CharStream;
use antlr4rs::input_stream::StringStream;

fn str_input_stream() {
    let mut input = StringStream::new(r#"A你4好§，\❤"#.to_string());
    let input = &mut input as &mut dyn CharStream;
    assert_eq!(input.size(), 8);
    input.consume();
    input.consume();
    input.consume();
    input.consume();
    input.consume();
    input.consume();

    assert_eq!(input.text(1, 1), "你");
    assert_eq!(input.text(1, 2), "你4");
    assert_eq!(input.text(3, 5), "好§，");
    assert_eq!(input.text(0, 5), "A你4好§，");
    assert_eq!(input.text(3, 10), "好§，\\❤");
    assert_eq!(input.text(0, 8), r#"A你4好§，\❤"#);
}

fn bench_input_stream(c: &mut Criterion) {
    c.bench_function("create/access input stream @1: str copy", |b| b.iter(|| str_input_stream()));
}

criterion_group!(benches, bench_input_stream);
criterion_main!(benches);