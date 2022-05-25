use std::fmt::{self, Display, Formatter};

use levenshtein::levenshtein;

use crate::phoneme::normalize_phonemes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    pub word: String,
    pub phonemes: String,
}

#[derive(Debug, Clone)]
pub struct Dictionary {
    words: Vec<Word>,
}

pub struct WordSearchIterator<'a> {
    word: &'a Word,
    dict: &'a Dictionary,
    max_distance: usize,
    index: usize,
}

impl Word {
    pub fn new(word: &str, phonemes: &str) -> Self {
        Self {
            word: String::from(word),
            phonemes: String::from(phonemes),
        }
    }

    /*pub fn len(&self) -> usize {
        self.word.graphemes(true).count()
    }*/

    pub fn is_similar(&self, word: &Word, max_distance: usize) -> bool {
        let l1 = self.phonemes.chars().count();
        let l2 = word.phonemes.chars().count();
        l1.abs_diff(l2) <= max_distance
            && levenshtein(&self.phonemes, &word.phonemes) <= max_distance
    }

    pub fn normalize_phonemes(&mut self) {
        self.phonemes = normalize_phonemes(&self.phonemes);
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.word.fmt(f)
    }
}

impl Dictionary {
    pub fn new() -> Self {
        Self { words: Vec::new() }
    }

    pub fn from_entries(entries: &[(&str, &str)]) -> Self {
        let mut res = Self::new();
        res.extend(entries);
        res
    }

    pub fn normalize(&mut self) {
        for word in self.words.iter_mut() {
            word.normalize_phonemes();
        }
    }

    pub fn add(&mut self, word: &str, phonemes: &str) {
        self.words.push(Word::new(word, phonemes));
    }

    /*pub fn extend_streaming<'a, I>(&mut self, mut entries: I)
        where I: StreamingIterator<Item = (&'a str, &'a str)>
    {
        while let Some((word, phonemes)) = entries.next() {
            self.add(word, phonemes);
        }
    }*/

    pub fn iter(&self) -> std::slice::Iter<Word> {
        self.words.iter()
    }

    pub fn find_similar<'a>(
        &'a self,
        word: &'a Word,
        max_distance: usize,
    ) -> WordSearchIterator<'a> {
        WordSearchIterator {
            dict: self,
            word,
            max_distance,
            index: 0,
        }
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for Dictionary {
    type Item = Word;
    type IntoIter = std::vec::IntoIter<Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.words.into_iter()
    }
}

impl<'a> Extend<&'a (&'a str, &'a str)> for Dictionary {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = &'a (&'a str, &'a str)>,
    {
        for (word, phonemes) in iter {
            self.add(word, phonemes);
        }
    }
}

impl<'a> Iterator for WordSearchIterator<'a> {
    type Item = &'a Word;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.dict.words.len() {
            return None;
        }
        let res = self.dict.words[self.index..]
            .iter()
            .enumerate()
            .find(|w| w.1.is_similar(self.word, self.max_distance));
        match res {
            Some((i, w)) => {
                self.index += i + 1;
                Some(w)
            },
            None => {
                self.index = usize::MAX;
                None
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn entries<'a>(dict: &'a Dictionary) -> Vec<(&'a str, &'a str)> {
        dict.iter()
            .map(|w| (&w.word[..], &w.phonemes[..]))
            .collect::<Vec<(&str, &str)>>()
    }

    #[rstest]
    #[case("word", "phonemes")]
    fn test_word_init(#[case] word: &str, #[case] phonemes: &str) {
        let w = Word::new(word, phonemes);
        assert_eq!(word, w.word);
        assert_eq!(phonemes, w.phonemes);
    }

    #[rstest]
    #[case(("w", "p"), ("w2", "p"), 0, true)]
    #[case(("w", "p"), ("w2", "p2"), 0, false)]
    #[case(("w", "p"), ("w2", "p2"), 1, true)]
    #[case(("w", "p"), ("w2", "q2"), 1, false)]
    #[case(("w", "p"), ("w2", "q2"), 2, true)]
    fn test_word_is_similar(
        #[case] word: (&str, &str),
        #[case] word2: (&str, &str),
        #[case] max_distance: usize,
        #[case] expected: bool,
    ) {
        let w = Word::new(word.0, word.1);
        let w2 = Word::new(word2.0, word2.1);
        assert_eq!(expected, w.is_similar(&w2, max_distance));
    }

    #[test]
    fn test_dict_init() {
        let dict = Dictionary::new();
        assert_eq!(0, dict.iter().count());
    }

    #[rstest]
    #[case(&[("w", "p"), ("w2", "p2")])]
    fn test_dict_add(#[case] items: &[(&str, &str)]) {
        let mut dict = Dictionary::new();
        for (w, p) in items {
            dict.add(w, p);
        }
        assert_eq!(items, entries(&dict));
    }

    #[rstest]
    #[case(&[("w", "p"), ("w2", "p2")])]
    fn test_dict_extend(#[case] items: &[(&str, &str)]) {
        let mut dict = Dictionary::new();
        dict.extend(items);
        assert_eq!(items, entries(&dict));
    }

    #[rstest]
    #[case(&[("w", "p"), ("w2", "p2")])]
    fn test_dict_from_entries(#[case] items: &[(&str, &str)]) {
        let dict = Dictionary::from_entries(items);
        assert_eq!(items, entries(&dict));
    }

    #[rstest]
    #[case(&[("w", "p"), ("w2", "p2")])]
    fn test_dict_into_iter(#[case] items: &[(&str, &str)]) {
        let mut dict = Dictionary::new();
        for (w, p) in items {
            dict.add(w, p);
        }
        assert_eq!(items.len(), dict.iter().count());
        let dict_items = dict
            .into_iter()
            .map(|w| (w.word, w.phonemes))
            .collect::<Vec<(String, String)>>();
        let dict_items_ref = dict_items
            .iter()
            .map(|(w, p)| (&w[..], &p[..]))
            .collect::<Vec<(&str, &str)>>();
        assert_eq!(items, dict_items_ref);
    }

    #[rstest]
    #[case(&[("w", "p"), ("w2", "p2")], ("x", "y"), 0, &[])]
    #[case(&[("w", "p"), ("w2", "p2")], ("x", "y"), 1, &["w"])]
    #[case(&[("w", "p1"), ("w2", "p2")], ("x", "2"), 1, &["w2"])]
    #[case(&[("w", "p"), ("w2", "p2")], ("x", "p"), 1, &["w", "w2"])]
    fn test_dict_find_similar(
        #[case] items: &[(&str, &str)],
        #[case] search: (&str, &str),
        #[case] max_distance: usize,
        #[case] expected: &[&str],
    ) {
        let dict = Dictionary::from_entries(items);
        let search = Word::new(search.0, search.1);
        let results = dict
            .find_similar(&search, max_distance)
            .map(|w| &w.word[..])
            .collect::<Vec<&str>>();
        assert_eq!(expected, results);
    }
}
