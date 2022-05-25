use std::io;

use streaming_iterator::StreamingIterator;

pub struct Lines<'a, I> {
    file: &'a mut I,
    buf: String,
    eof: bool,
}

pub struct StrLines<'a> {
    iter: std::str::Lines<'a>,
    res: Option<&'a str>,
}

pub struct Dedup<I, T> {
    iter: I,
    prev: Option<T>,
}

pub trait IteratorEx
where
    Self: Iterator,
{
    fn dedup(self) -> Dedup<Self, Self::Item>
    where
        Self: Sized,
        Self::Item: PartialEq,
    {
        Dedup::new(self)
    }
}

pub fn lines<I: io::BufRead>(file: &mut I) -> Lines<I> {
    Lines::new(file)
}

pub fn str_lines(string: &str) -> StrLines {
    StrLines::new(string)
}

impl<I: Iterator> IteratorEx for I {}

impl<'a, I: io::BufRead> Lines<'a, I> {
    pub fn new(file: &'a mut I) -> Self {
        Self {
            file,
            buf: String::new(),
            eof: false,
        }
    }
}

impl<'a, I: io::BufRead> StreamingIterator for Lines<'a, I> {
    type Item = str;

    fn advance(&mut self) {
        if self.eof {
            return;
        }
        self.buf.clear();
        match self.file.read_line(&mut self.buf) {
            Ok(n) => {
                self.eof = n == 0;
            },
            Err(e) => {
                eprintln!("Warning: failed to read line: {:?}", e);
                self.eof = true;
            },
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        if self.eof {
            None
        } else {
            Some(self.buf.trim_end_matches('\n'))
        }
    }
}

impl<'a> StrLines<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            iter: string.lines(),
            res: None,
        }
    }
}

impl<'a> StreamingIterator for StrLines<'a> {
    type Item = str;

    fn advance(&mut self) {
        self.res = self.iter.next();
    }

    fn get(&self) -> Option<&Self::Item> {
        self.res
    }
}

impl<I, T> Dedup<I, T>
where
    I: Iterator<Item = T>,
    T: PartialEq,
{
    pub fn new(mut iter: I) -> Self {
        let first = iter.next();
        Self { iter, prev: first }
    }
}

impl<I, T> Iterator for Dedup<I, T>
where
    I: Iterator<Item = T>,
    T: PartialEq,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.prev.take()?;
        self.prev = self.iter.find(|next| *next != prev);
        Some(prev)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::io::Cursor;
    use streaming_iterator::StreamingIterator;

    fn svec<I: StreamingIterator<Item = str>>(mut iter: I) -> Vec<String> {
        let mut res = Vec::new();
        while let Some(item) = iter.next() {
            res.push(String::from(item));
        }
        res
    }

    #[rstest]
    #[case("", &[])]
    #[case("\n\n", &["", ""])]
    #[case("xx \tyy\nz\n\nzz", &["xx \tyy", "z", "", "zz"])]
    fn test_lines(#[case] input: &str, #[case] expected: &[&str]) {
        assert_eq!(expected, svec(lines(&mut Cursor::new(input))));
    }

    #[rstest]
    #[case("", &[])]
    #[case("\n\n", &["", ""])]
    #[case("xx \tyy\nz\n\nzz", &["xx \tyy", "z", "", "zz"])]
    fn test_str_lines(#[case] input: &str, #[case] expected: &[&str]) {
        assert_eq!(expected, svec(str_lines(input)));
    }

    #[rstest]
    #[case(&[], &[])]
    #[case(&[1], &[1])]
    #[case(&[1, 2, 3], &[1, 2, 3])]
    #[case(&[1, 1, 1, 2, 2, 1, 3, 2, 1, 1], &[1, 2, 1, 3, 2, 1])]
    fn test_dedup(#[case] input: &[usize], #[case] expected: &[usize]) {
        let res: Vec<usize> = input.iter().map(|x| *x).dedup().collect();
        assert_eq!(expected, res);
    }
}
