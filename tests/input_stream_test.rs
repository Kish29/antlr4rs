use antlr4rs::char_stream::CharStream;
use antlr4rs::input_stream::{from_bytes, from_str, from_u16s, from_u32s, from_u8s};
use antlr4rs::int_stream::EOF;

#[test]
fn test_input_stream() {
    let input = &mut from_str(r#"A你4好§，\❤"#) as &mut dyn CharStream;
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
    let v = &mut from_bytes(&b"V\xaa\xbb"[..]) as &mut dyn CharStream;
    assert_eq!(v.la(1), 'V' as isize);
}

#[test]
fn test_code_point_8bit_stream() {
    let v = &mut from_u8s(&b"V12"[..]) as &mut dyn CharStream;
    assert_eq!(v.la(1), 'V' as isize);
    assert_eq!(v.index(), 0);
    v.consume();
    assert_eq!(v.la(1), '1' as isize);
    assert_eq!(v.index(), 1);
    v.consume();
    assert_eq!(v.la(1), '2' as isize);
    assert_eq!(v.index(), 2);
}

#[test]
fn test_code_point_16bit_stream() {
    let v = &mut from_u16s(&[0x00a3, 0x00a4, 0x00a5, 0x00a6, 0x00a7]) as &mut dyn CharStream;
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

#[test]
fn test_code_point_32bit_stream() {
    let v = &mut from_u32s(&[0x00a3, 0x00a4, 0x00a5, 0x00a6, 0x00a7]) as &mut dyn CharStream;
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
}