use std::io::{self, BufRead};
use std::ops::RangeBounds;

use streaming_iterator::StreamingIterator;
use unicode_segmentation::UnicodeSegmentation;

use crate::args::Args;

use find_similar_words::dictionary::Dictionary;
use find_similar_words::iter::lines;
use find_similar_words::util::open_input_file;
use find_similar_words::util::to_tsv_pair;

pub struct Options {
    pub dict: Dictionary,
    pub dict2: Option<Dictionary>,
    pub max_distance: usize,
}

impl Options {
    fn load_dict<R: RangeBounds<usize>, I: BufRead>(
        word_length: &R,
        file: &mut I,
    ) -> Dictionary {
        let mut res = Dictionary::new();
        let mut lines = lines(file);
        let filter_word_length =
            !(word_length.contains(&0) && word_length.contains(&usize::MAX));
        while let Some(line) = lines.next() {
            if let Some((word, phonemes)) = to_tsv_pair(line) {
                if !filter_word_length
                    || word_length.contains(&word.graphemes(true).count())
                {
                    res.add(word, phonemes)
                }
            }
        }
        res
    }

    pub fn from_args(args: &Args) -> io::Result<Self> {
        let word_length = args.min_word_length..=args.max_word_length;

        let mut file = open_input_file(&args.input_filenames.get(0))?;
        let mut dict = Self::load_dict(&word_length, &mut file);
        let mut dict2 = match args.input_filenames.get(1) {
            Some(fname) => {
                let mut file = open_input_file(&Some(fname))?;
                Some(Self::load_dict(&word_length, &mut file))
            },
            None => None,
        };

        if args.normalize {
            dict.normalize();
            if let Some(d) = dict2.as_mut() {
                d.normalize();
            }
        }

        Ok(Self {
            dict,
            dict2,
            max_distance: args.max_distance,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::io::Cursor;
    use std::ops::Range;

    #[rstest]
    #[case("", 0..10, &[])]
    #[case("xx\tx\nyyy\ty\nzzzz\tz\n", 0..10, &[("xx", "x"), ("yyy", "y"), ("zzzz", "z")])]
    #[case("xx\tx\nyyy\ty\nzzzz\tz\n", 0..1, &[])]
    #[case("xx\tx\nyyy\ty\nzzzz\tz\n", 0..4, &[("xx", "x"), ("yyy", "y")])]
    #[case("xx\tx\nyyy\ty\nzzzz\tz\n", 4..10, &[("zzzz", "z")])]
    #[case("H̡̫̤̤̣͉̤ͭ̓̓̇͗̎̀\txxx", 1..2, &[("H̡̫̤̤̣͉̤ͭ̓̓̇͗̎̀", "xxx")])]
    fn test_options_load_dict(
        #[case] file_contents: &str,
        #[case] word_length: Range<usize>,
        #[case] expected: &[(&str, &str)],
    ) {
        let mut file = Cursor::new(String::from(file_contents));
        let dict = Options::load_dict(&word_length, &mut file);
        let mut res: Vec<(&str, &str)> = dict
            .iter()
            .map(|w| (&w.word[..], &w.phonemes[..]))
            .collect();
        res.sort_by(|x, y| x.0.cmp(y.0));
        assert_eq!(expected, res);
    }
}
