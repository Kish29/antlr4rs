use std::fmt::{Debug, Formatter};
use std::ops::{Index, Range, RangeFrom};

pub trait CodePoints:
Index<Range<usize>, Output=Self>
+ Index<RangeFrom<usize>, Output=Self>
+ ToOwned
+ Debug
+ 'static
{
    /// code point at the `pos` of the code points container
    /// sting type must be indexed by the interpreter as the characters
    fn code_point_at(&self, pos: isize) -> Option<isize>;

    fn len(&self) -> usize;
}

impl CodePoints for str {
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
    fn len(&self) -> usize {
        self.chars().count()
    }
}

impl<T: Copy + Debug + Into<isize> + 'static> CodePoints for [T] {
    #[inline]
    fn code_point_at(&self, pos: isize) -> Option<isize> {
        if pos < 0 || pos >= self.len() as isize {
            return None;
        }
        Some(self[pos as usize].into())
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}
