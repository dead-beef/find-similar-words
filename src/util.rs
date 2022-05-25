use std::collections::HashMap;
use std::env;
use std::fs;
use std::hash::Hash;
use std::io;
use std::path;
use std::process;

pub trait ArgParser
where
    Self: Sized,
{
    fn parse<O: io::Write, E: io::Write>(
        args: Vec<String>,
        stdout: &mut O,
        stderr: &mut E,
    ) -> Result<Self, i32>;

    fn from_env_args() -> Self {
        match Self::parse(
            env::args().collect(),
            &mut io::stdout(),
            &mut io::stderr(),
        ) {
            Ok(res) => res,
            Err(s) => process::exit(s),
        }
    }
}

pub trait Multimap<K: Eq + Hash, V> {
    fn mm_insert(&mut self, key: K, value: V);
}

impl<K: Eq + Hash, V> Multimap<K, V> for HashMap<K, Vec<V>> {
    fn mm_insert(&mut self, key: K, value: V) {
        match self.get_mut(&key) {
            Some(values) => {
                values.push(value);
            },
            None => {
                self.insert(key, Vec::from([value]));
            },
        };
    }
}

/*pub fn map_char(map: &[(&str, char)], c: char) -> char {
    map.iter().find(|m| m.0.contains(c)).map(|m| m.1).unwrap_or(c)
}*/

pub fn to_tsv_pair(line: &str) -> Option<(&str, &str)> {
    let trimmed = line.trim();
    if let Some((key, value)) = trimmed.split_once('\t') {
        let key = key.trim();
        let value = value.trim();
        if !(key.is_empty() && value.is_empty()) {
            return Some((key, value));
        }
    }
    if !trimmed.is_empty() {
        /*return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Could not parse line {:?}", trimmed)
        ));*/
        eprintln!("Warning: could not parse line {:?}", trimmed);
    }
    None
}

pub fn open_input_file<P: AsRef<path::Path>>(
    path: &Option<P>,
) -> io::Result<Box<dyn io::BufRead>> {
    match path.as_ref() {
        Some(p) => {
            let fp = fs::File::open(p)?;
            Ok(Box::new(io::BufReader::new(fp)))
        },
        None => Ok(Box::new(io::BufReader::new(io::stdin()))),
    }
}

pub fn open_output_file<P: AsRef<path::Path>>(
    path: &Option<P>,
) -> io::Result<Box<dyn io::Write>> {
    match path.as_ref() {
        Some(p) => {
            let fp = fs::File::create(p)?;
            Ok(Box::new(io::BufWriter::new(fp)))
        },
        None => Ok(Box::new(io::stdout())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::collections::HashMap;

    /*#[rstest]
    #[case('a', 'd')]
    #[case('i', 'j')]
    #[case('k', 'l')]
    #[case('j', 'j')]
    #[case('x', 'x')]
    fn test_map_char(#[case] input: char, #[case] expected: char) {
        let map = [("abc", 'd'), ("efghi", 'j'), ("k", 'l')];
        assert_eq!(expected, map_char(&map, input));
    }*/

    #[rstest]
    #[case("")]
    #[case(" ")]
    #[case("\t")]
    #[case("\t \t ")]
    #[case("xx\t \t ")]
    #[case("\t \txx ")]
    fn test_to_tsv_pair_invalid(#[case] input: &str) {
        assert_eq!(None, to_tsv_pair(input));
    }

    #[rstest]
    #[case("xx\tyy", ("xx", "yy"))]
    #[case("  xx\tyy  \n", ("xx", "yy"))]
    #[case(" \t xx\tyy \t ", ("xx", "yy"))]
    fn test_to_tsv_pair(#[case] input: &str, #[case] expected: (&str, &str)) {
        assert_eq!(Some(expected), to_tsv_pair(input));
    }

    #[rstest]
    #[case(&[], vec![])]
    #[case(&[(1, 2), (3, 4)], vec![(1, vec![2]), (3, vec![4])])]
    #[case(&[(1, 2), (3, 4), (1, 5), (1, 2), (3, 6)], vec![(1, vec![2, 5, 2]), (3, vec![4, 6])])]
    fn test_multimap_hash(
        #[case] input: &[(usize, usize)],
        #[case] expected: Vec<(usize, Vec<usize>)>,
    ) {
        let mut map = HashMap::<usize, Vec<usize>>::new();
        for (k, v) in input {
            map.mm_insert(*k, *v);
        }
        let mut items: Vec<(usize, Vec<usize>)> = map.into_iter().collect();
        items.sort_by(|i, j| i.0.cmp(&j.0));
        assert_eq!(expected, items);
    }
}
