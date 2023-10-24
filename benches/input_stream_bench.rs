use criterion::{black_box, Criterion, criterion_group, criterion_main};
use antlr4rs::char_stream::{CharStream};
use antlr4rs::input_stream::{CodePoint32BitStream, StringStream};

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

fn code_point32_input_stream() {
    let mut input = CodePoint32BitStream::new(vec![0x00a3, 0x00a4, 0x00a5, 0x00a6, 0x00a7]);
    let v = &mut input as &mut dyn CharStream;
    v.consume();
    v.consume();
    v.consume();
    v.consume();
    v.consume();

    assert_eq!(v.text(1, 1), "¤");
    assert_eq!(v.text(1, 2), "¤¥");
    assert_eq!(v.text(3, 5), "¦§");
    assert_eq!(v.text(0, 5), "£¤¥¦§");
}

fn code_point32_input_stream_into_owned() {
    let mut input = CodePoint32BitStream::new(vec![0x00a3, 0x00a4, 0x00a5, 0x00a6, 0x00a7]);
    let v = &mut input as &mut dyn CharStream;
    v.consume();
    v.consume();
    v.consume();
    v.consume();
    v.consume();

    assert_eq!(v.text(1, 1).into_owned(), "¤");
    assert_eq!(v.text(1, 2).into_owned(), "¤¥");
    assert_eq!(v.text(3, 5).into_owned(), "¦§");
    assert_eq!(v.text(0, 5).into_owned(), "£¤¥¦§");
}

fn bench_input_stream(c: &mut Criterion) {
    c.bench_function("create/access string input stream.", |b| b.iter(|| str_input_stream()));
    c.bench_function("create/access code_point32 input stream.", |b| b.iter(|| code_point32_input_stream()));
    c.bench_function("create/access code_point32 input stream with into_owned invoke.", |b| b.iter(|| code_point32_input_stream_into_owned()));
}

criterion_group!(benches, bench_input_stream);
criterion_main!(benches);