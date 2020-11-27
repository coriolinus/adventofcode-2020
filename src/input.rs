use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

pub fn parse<T>(path: &Path) -> std::io::Result<impl Iterator<Item = T>>
where
    T: FromStr,
{
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    Ok(std::iter::from_fn(move || {
        buf.clear();
        reader
            .read_line(&mut buf)
            .map_err(|_| ())
            .and_then(|_| T::from_str(buf.trim()).map_err(|_| ()))
            .ok()
    })
    .fuse())
}

/// adaptor which plugs into parse, splitting comma-separated items from the line
///
/// This can be flattened or consumed by line, as required
pub struct CommaSep<T>(Vec<T>);

impl<T> FromStr for CommaSep<T>
where
    T: FromStr,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map(CommaSep)
    }
}

impl<T> IntoIterator for CommaSep<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
