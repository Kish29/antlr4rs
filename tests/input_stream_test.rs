use antlr4rs::char_stream::CharStream;
use antlr4rs::input_stream::{StringStream, ByteStream, CodePoint16BitStream, CodePoint32BitStream, CodePoint8BitStream};
use antlr4rs::int_stream::EOF;

#[test]
fn test_input_stream() {
    let mut input = StringStream::from(r#"A你4好§，\❤"#);
    let input = &mut input as &mut dyn CharStream;
    assert_eq!(input.size(), 8);
    assert_eq!(input.la(1), 'A' as isize);
    assert_eq!(input.index(), 0);
    input.consume();
    assert_eq!(input.la(1), '你' as isize);
    assert_eq!(input.la(-1), 'A' as isize);
    assert_eq!(input.index(), 1);
    input.consume();
    assert_eq!(input.la(1), '4' as isize);
    assert_eq!(input.index(), 2);
    input.consume();
    assert_eq!(input.la(1), '好' as isize);
    assert_eq!(input.index(), 3);
    assert_eq!(input.la(-2), '你' as isize);
    input.consume();
    assert_eq!(input.la(1), '§' as isize);
    assert_eq!(input.index(), 4);
    assert_eq!(input.la(2), '，' as isize);
    assert_eq!(input.la(-2), '4' as isize);
    assert_eq!(input.la(3), '\\' as isize);
    input.consume();
    assert_eq!(input.la(1), '，' as isize);
    assert_eq!(input.index(), 5);
    assert_eq!(input.la(2), '\\' as isize);
    assert_eq!(input.la(-2), '好' as isize);
    assert_eq!(input.la(4), EOF);
    input.consume();
    assert_eq!(input.la(1), '\\' as isize);
    assert_eq!(input.index(), 6);
    assert_eq!(input.la(3), EOF);
    assert_eq!(input.la(-2), '§' as isize);
    assert_eq!(input.la(-10), EOF);
    input.consume();
    assert_eq!(input.la(1), '❤' as isize);
    assert_eq!(input.index(), 7);
    assert_eq!(input.la(2), EOF);
    assert_eq!(input.la(-3), '§' as isize);
    assert_eq!(input.la(-10), EOF);

    assert_eq!(input.text(1, 1), "你");
    assert_eq!(input.text(1, 2), "你4");
    assert_eq!(input.text(3, 5), "好§，");
    assert_eq!(input.text(0, 5), "A你4好§，");
    assert_eq!(input.text(3, 10), "好§，\\❤");
}

#[test]
fn test_byte_stream() {
    let mut input = ByteStream::new(b"V\xaa\xbb".to_vec());
    let input = &mut input as &mut dyn CharStream;
    assert_eq!(input.la(1), 'V' as isize);
}

#[test]
fn test_code_point_8bit_stream() {
    let mut input = CodePoint8BitStream::new(b"V12".to_vec());
    let input = &mut input as &mut dyn CharStream;
    assert_eq!(input.la(1), 'V' as isize);
    assert_eq!(input.index(), 0);
    input.consume();
    assert_eq!(input.la(1), '1' as isize);
    assert_eq!(input.index(), 1);
    input.consume();
    assert_eq!(input.la(1), '2' as isize);
    assert_eq!(input.index(), 2);
}

#[test]
fn test_code_point_16bit_stream() {
    let mut input = CodePoint16BitStream::new(vec![0x00a3u16, 0x00a4u16, 0x00a5u16, 0x00a6u16, 0x00a7u16]);
    let input = &mut input as &mut dyn CharStream;
    assert_eq!(input.la(1), '£' as isize);
    assert_eq!(input.index(), 0);
    input.consume();
    assert_eq!(input.la(1), '¤' as isize);
    assert_eq!(input.index(), 1);
    input.consume();
    assert_eq!(input.la(1), '¥' as isize);
    assert_eq!(input.index(), 2);
    input.consume();
    assert_eq!(input.la(1), '¦' as isize);
    assert_eq!(input.index(), 3);
    input.consume();
    assert_eq!(input.la(1), '§' as isize);
    assert_eq!(input.index(), 4);
    input.consume();
    assert_eq!(input.la(1), EOF);
    assert_eq!(input.text(1, 1), "¤");
    assert_eq!(input.text(1, 2), "¤¥");
    assert_eq!(input.text(3, 5), "¦§");
    assert_eq!(input.text(0, 5), "£¤¥¦§");
}

#[test]
fn test_code_point_32bit_stream() {
    let mut input = CodePoint32BitStream::new(vec![0x00a3, 0x00a4, 0x00a5, 0x00a6, 0x00a7]);
    let v = &mut input as &mut dyn CharStream;
    assert_eq!(v.la(1), '£' as isize);
    assert_eq!(v.index(), 0);
    v.consume();
    assert_eq!(v.la(1), '¤' as isize);
    assert_eq!(v.index(), 1);
    v.consume();
    assert_eq!(v.la(1), '¥' as isize);
    assert_eq!(v.index(), 2);
    v.consume();
    assert_eq!(v.la(1), '¦' as isize);
    assert_eq!(v.index(), 3);
    v.consume();
    assert_eq!(v.la(1), '§' as isize);
    assert_eq!(v.index(), 4);
    v.consume();
    assert_eq!(v.la(1), EOF);
    assert_eq!(v.text(1, 1), "¤");
    assert_eq!(v.text(1, 2), "¤¥");
    assert_eq!(v.text(3, 5), "¦§");
    assert_eq!(v.text(0, 5), "£¤¥¦§");
}