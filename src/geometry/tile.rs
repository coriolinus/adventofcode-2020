use smallstr::SmallString;
use std::marker::PhantomData;

/// Number of characters below which the [`Chunks`] iterator does not allocate.
pub const CHUNK_WIDTH: usize = 4;

/// A type implementing `DisplayWidth` has a constant width for display and parsing.
///
/// This makes it suitable for 2d cartesian maps.
pub trait DisplayWidth {
    const DISPLAY_WIDTH: usize;

    /// Split a string into an iterator of chunks of characters of length `DISPLAY_WIDTH`
    fn chunks<'a>(s: &'a str) -> Chunks<'a, Self> {
        Chunks(s.chars(), PhantomData)
    }
}

/// Iterator of chunks of equal width from a string.
///
/// Created with [`DisplayWidth::chunks`]. Never heap-allocates if `T::DISPLAY_WIDTH <= CHUNK_WIDTH`.
pub struct Chunks<'a, T: ?Sized>(std::str::Chars<'a>, PhantomData<T>);

impl<'a, T: DisplayWidth> Iterator for Chunks<'a, T> {
    // 4 bytes in a max-width char
    type Item = SmallString<[u8; 4 * CHUNK_WIDTH]>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s = SmallString::new();
        for _ in 0..T::DISPLAY_WIDTH {
            s.push(self.0.next()?);
        }
        Some(s)
    }
}
