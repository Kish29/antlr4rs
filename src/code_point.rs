use std::borrow::Cow;
use std::char::REPLACEMENT_CHARACTER;
use std::fmt::Debug;
use std::ops::{Index, Range, RangeFrom};

const TEXT_RANGE_EOF: &'static str = "<EOF>";

pub trait CodePoints<'a>:
Index<Range<usize>, Output=Self>
+ Index<RangeFrom<usize>, Output=Self>
+ ToOwned
+ Debug
+ 'static
{
    /// code point at the `pos` of the code points container
    /// sting type must be indexed by the interpreter as the characters
    fn code_point_at(&self, pos: isize) -> Option<isize>;

    fn size(&self) -> usize;

    /// returns the text for the interval `start`..`end` of characters within this CodePoints
    /// include the end index, the symbol of each must convert to character
    /// must returns <EOF> if index out of range
    fn text_range(&'a self, start: usize, end: usize) -> Cow<'a, str>;
}

impl<'a> CodePoints<'a> for str {
    #[inline]
    fn code_point_at(&self, pos: isize) -> Option<isize> {
        if pos < 0 || pos >= self.len() as isize {
            return None;
        }
        if let Some(ch) = self.chars().nth(pos as usize) {
            return Some(ch as isize);
        }
        None
    }

    #[inline]
    fn size(&self) -> usize {
        self.chars().count()
    }

    #[inline]
    fn text_range(&'a self, start: usize, mut end: usize) -> Cow<'a, str> {
        let chars = self.chars();
        let chars_len = chars.count();
        if start > end || start >= chars_len {
            return Cow::Borrowed(TEXT_RANGE_EOF);
        }
        if end >= chars_len {
            // here -1 is order to get the character when start equal to end
            end = chars_len - 1;
        }
        // here +1 is order to offset the + 1 below
        Cow::Owned(self.chars().skip(start).take(end - start + 1).collect())
    }
}

/// T convert to `u32` and as `isize`, due to `isize` not implementation the trait `From<u16>`
impl<'a, T: ?Sized + Copy + Debug + Into<u32> + 'static> CodePoints<'a> for [T] {
    #[inline]
    fn code_point_at(&self, pos: isize) -> Option<isize> {
        if pos < 0 || pos >= self.len() as isize {
            return None;
        }
        Some(self[pos as usize].into() as isize)
    }

    #[inline]
    fn size(&self) -> usize {
        self.len()
    }

    #[inline]
    fn text_range(&'a self, start: usize, mut end: usize) -> Cow<'a, str> {
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