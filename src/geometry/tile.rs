use smallstr::SmallString;
use std::{marker::PhantomData, ops::Not};

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

/// A Tile which is compatible with booleans
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    parse_display::Display,
    parse_display::FromStr,
)]
pub enum Bool {
    #[display("#")]
    True,
    #[display(".")]
    False,
}

impl Default for Bool {
    fn default() -> Self {
        Bool::False
    }
}

impl DisplayWidth for Bool {
    const DISPLAY_WIDTH: usize = 1;
}

impl PartialEq<bool> for Bool {
    fn eq(&self, other: &bool) -> bool {
        (*self == Bool::True) == *other
    }
}

impl Not for Bool {
    type Output = Bool;

    fn not(self) -> Bool {
        match self {
            Bool::True => Bool::False,
            Bool::False => Bool::True,
        }
    }
}

impl From<Bool> for bool {
    fn from(b: Bool) -> bool {
        match b {
            Bool::True => true,
            Bool::False => false,
        }
    }
}

impl From<bool> for Bool {
    fn from(b: bool) -> Bool {
        if b {
            Bool::True
        } else {
            Bool::False
        }
    }
}
