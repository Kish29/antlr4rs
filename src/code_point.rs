use std::borrow::Cow;
use std::char::REPLACEMENT_CHARACTER;
use std::fmt::Debug;

const TEXT_RANGE_EOF: &'static str = "<EOF>";

pub trait CodePoints {
    /// code point at the `pos` of [CodePoints] and try to convert to [u32].
    /// sting type must be indexed by the interpreter as the characters
    fn code_point_at(&self, pos: usize) -> Option<u32>;

    fn size(&self) -> usize;

    /// returns the text for the interval `start`..`end` of characters within this CodePoints
    /// include the end index, the symbol of each must convert to character
    /// must returns "<EOF>" if index out of range
    fn text_range(&self, start: usize, end: usize) -> Cow<'_, str>;
}

impl CodePoints for String {
    #[inline]
    fn code_point_at(&self, pos: usize) -> Option<u32> {
        if pos >= self.len() {
            return None;
        }
        Some(self.chars().nth(pos).unwrap_or(REPLACEMENT_CHARACTER) as u32)
    }

    #[inline]
    fn size(&self) -> usize {
        self.chars().count()
    }

    #[inline]
    fn text_range(&self, start: usize, mut end: usize) -> Cow<'_, str> {
        if start > end || start >= self.len() {
            return Cow::Borrowed(TEXT_RANGE_EOF);
        }
        if end >= self.len() {
            end = self.len() - 1;
        }
        // first, find the byte index of the char index `start`.
        let byte_idx_start = byte_idx_by_chars_pass_through(self, 0, start);
        // second, find the byte index of the char index `end`.
        let byte_idx_end = byte_idx_by_chars_pass_through(self, byte_idx_start, end - start + 1);
        Cow::Borrowed(&self[byte_idx_start..byte_idx_end])
    }
}

/// T convert to `u32` and as `isize`, due to `isize` not implementation the trait `From<u16>`
impl<T: ?Sized + Copy + Debug + Into<u32>> CodePoints for Vec<T> {
    #[inline]
    fn code_point_at(&self, pos: usize) -> Option<u32> {
        if pos >= self.len() {
            return None;
        }
        Some(self[pos].into())
    }

    #[inline]
    fn size(&self) -> usize {
        self.len()
    }

    #[inline]
    fn text_range(&self, start: usize, mut end: usize) -> Cow<'_, str> {
        if start > end || start >= self.len() {
            return Cow::Borrowed(TEXT_RANGE_EOF);
        }
        if end >= self.len() {
            end = self.len() - 1;
        }
        let str: String = self.iter()
            .skip(start)
            .take(end - start + 1)
            .map(|&v| char::from_u32(v.into()).unwrap_or(REPLACEMENT_CHARACTER))
            .collect();
        Cow::Owned(str)
    }
}

/// convert the char index: get the byte index from byte index: `start_char_byte_idx` and pass through `chars_num`
fn byte_idx_by_chars_pass_through(s: &str, start_char_byte_idx: usize, mut chars_num: usize) -> usize {
    let s_len = s.len();
    if start_char_byte_idx >= s_len {
        panic!("[byte_idx_by_chars_pass_through] index out of range, it could not happen. It is a bug.")
    }
    let mut ptr = start_char_byte_idx;
    // loop until ptr pass through the `chars_num` of string.
    loop {
        if chars_num <= 0 {
            break;
        }
        ptr += 1;
        if ptr >= s_len {
            break;
        }
        if s.is_char_boundary(ptr) {
            chars_num -= 1;
        }
    }
    ptr
}