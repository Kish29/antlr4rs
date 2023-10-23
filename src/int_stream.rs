use std::borrow::Cow;

pub const EOF: isize = -1;

pub trait IntStream<'a> {
    fn consume(&mut self);

    /// Gets the value of the symbol at offset i from the current position.
    /// When i==1, this method returns the value of the current symbol in the stream (which is the next symbol to be consumed).
    /// When i==-1, this method returns the value of the previously read symbol in the stream.
    /// It is not valid to call this method with i==0, but the specific behavior is unspecified because this method is frequently called from performance-critical code.
    /// This method is guaranteed to succeed if any of the following are true:
    // i>0
    /// i==-1 and index() returns a value greater than the value of index() after the stream was constructed and LA(1) was called in that order.
    /// Specifying the current index() relative to the index after the stream was created allows for filtering implementations that do not return every symbol from the underlying source.
    /// Specifying the call to LA(1) allows for lazily initialized streams.
    /// LA(i) refers to a symbol consumed within a marked region that has not yet been released.
    /// If i represents a position at or beyond the end of the stream, this method returns EOF.
    /// The return value is unspecified if i<0 and fewer than -i calls to consume() have occurred from the beginning of the stream before calling this method.
    /// Return `EOF` if `i` points to position at or beyond the end of the stream
    fn la(&mut self, i: isize) -> isize;

    fn index(&self) -> isize;

    fn seek(&mut self, index: isize);

    fn size(&self) -> isize;

    fn source_name(&'a self) -> Cow<'a, str>;
}
