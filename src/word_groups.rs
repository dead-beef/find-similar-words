use std::collections::hash_map::IntoValues;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::hash::Hash;
use std::slice;

use union_find_rs::prelude::*;

use crate::dictionary::Dictionary;
use crate::util::Multimap;

#[derive(Debug, Clone)]
pub struct WordGroups {
    groups: Vec<Vec<String>>,
}

pub struct GroupBuilder<K, V>
where
    K: Copy + Eq + Hash,
    V: Eq + Hash,
{
    item_set: HashMap<V, K>,
    sets: DisjointSets<K>,
}

impl WordGroups {
    pub fn new() -> Self {
        Self { groups: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.groups.len()
    }

    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    pub fn iter(&self) -> slice::Iter<Vec<String>> {
        self.groups.iter()
    }

    pub fn from_dict(dict: Dictionary) -> Self {
        let mut hash = HashMap::<String, Vec<String>>::new();
        for word in dict {
            hash.mm_insert(word.phonemes, word.word);
        }
        hash.into_values().filter(|v| v.len() > 1).collect()
    }

    pub fn from_dicts(dict: Dictionary, dict2: Dictionary) -> Self {
        let mut hash = HashMap::<String, Vec<(String, usize)>>::new();
        for (i, d) in [dict, dict2].into_iter().enumerate() {
            for word in d {
                hash.mm_insert(word.phonemes, (word.word, i));
            }
        }
        hash.into_values()
            .filter(|v| {
                let l = v.len();
                l > 1 && v[0].1 != v[l - 1].1
            })
            .map(|v| v.into_iter().map(|w| w.0))
            .collect()
    }
}

impl Default for WordGroups {
    fn default() -> Self {
        Self::new()
    }
}

impl<I> FromIterator<I> for WordGroups
where
    I: IntoIterator<Item = String>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let mut res: Vec<Vec<String>> = iter
            .into_iter()
            .filter_map(|g| {
                let mut g: Vec<String> = g.into_iter().collect();
                g.sort();
                g.dedup();
                if g.len() > 1 {
                    Some(g)
                } else {
                    None
                }
            })
            .collect();
        res.sort_by(|g, g1| g[0].cmp(&g1[0]));
        Self { groups: res }
    }
}

impl IntoIterator for WordGroups {
    type Item = Vec<String>;
    type IntoIter = <Vec<Vec<String>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.groups.into_iter()
    }
}

impl Display for WordGroups {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for group in self.groups.iter() {
            write!(f, "{}", group[0])?;
            for word in group.iter().skip(1) {
                write!(f, " {}", word)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl<K, V> GroupBuilder<K, V>
where
    K: Copy + Eq + Hash,
    V: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            item_set: HashMap::new(),
            sets: DisjointSets::new(),
        }
    }

    pub fn add<U>(&mut self, set: K, value: U)
    where
        U: Into<V>,
    {
        self.sets.make_set(set).unwrap_or(());
        if let Some(prev_set) = self.item_set.insert(value.into(), set) {
            let set = self.sets.find_set(&set).unwrap();
            let prev_set = self.sets.find_set(&prev_set).unwrap();
            self.sets.union(&set, &prev_set).unwrap();
        }
    }

    pub fn extend<I>(&mut self, set: K, items: I)
    where
        I: IntoIterator,
        I::Item: Into<V>,
    {
        for item in items {
            self.add(set, item.into());
        }
    }
}

impl<K, V> Default for GroupBuilder<K, V>
where
    K: Copy + Eq + Hash,
    V: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> IntoIterator for GroupBuilder<K, V>
where
    K: Copy + Eq + Hash,
    V: Eq + Hash,
{
    type Item = Vec<V>;
    type IntoIter = IntoValues<K, Vec<V>>;
    fn into_iter(self) -> Self::IntoIter {
        let mut set_items = HashMap::<K, Vec<V>>::new();
        for (item, mut set) in self.item_set {
            set = self.sets.find_set(&set).unwrap();
            set_items.mm_insert(set, item);
        }
        set_items.into_values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dictionary::Dictionary;
    use rstest::*;

    #[rstest]
    #[case(vec![], vec![])]
    #[case(vec![vec!["2", "1"]], vec![vec!["1", "2"]])]
    #[case(vec![vec!["1", "1"]], vec![])]
    #[case(vec![vec!["4", "2", "2"], vec!["3", "3", "3"], vec!["5", "6", "1", "5"]], vec![vec!["1", "5", "6"], vec!["2", "4"]])]
    fn test_word_groups_from_iter(
        #[case] input: Vec<Vec<&str>>,
        #[case] expected: Vec<Vec<&str>>,
    ) {
        let input = input.iter().map(|g| g.iter().map(|i| String::from(*i)));
        assert_eq!(expected, WordGroups::from_iter(input).groups);
    }

    #[rstest]
    #[case(&[], vec![])]
    #[case(&[("w", "p"), ("w1", "p1")], vec![])]
    #[case(&[("w", "p"), ("w", "p")], vec![])]
    #[case(&[("c", "p"), ("a", "p"), ("d", "q")], vec![vec!["a", "c"]])]
    #[case(&[("c", "p"), ("b", "p"), ("d", "q"), ("a", "q")], vec![vec!["a", "d"], vec!["b", "c"]])]
    fn test_word_groups_from_dict(
        #[case] entries: &[(&str, &str)],
        #[case] expected: Vec<Vec<&str>>,
    ) {
        let dict = Dictionary::from_entries(entries);
        assert_eq!(expected, WordGroups::from_dict(dict).groups);
    }

    #[rstest]
    #[case(&[], &[], vec![])]
    #[case(&[("w", "p")], &[("w1", "p1")], vec![])]
    #[case(&[("w", "p"), ("w1", "p")], &[("w1", "p1")], vec![])]
    #[case(&[("w", "p")], &[("w", "p")], vec![])]
    #[case(&[("w", "p")], &[("w1", "p")], vec![vec!["w", "w1"]])]
    #[case(&[("c", "p"), ("d", "q")], &[("b", "p"), ("a", "q")], vec![vec!["a", "d"], vec!["b", "c"]])]
    fn test_word_groups_from_dicts(
        #[case] entries: &[(&str, &str)],
        #[case] entries2: &[(&str, &str)],
        #[case] expected: Vec<Vec<&str>>,
    ) {
        let dict = Dictionary::from_entries(entries);
        let dict2 = Dictionary::from_entries(entries2);
        assert_eq!(expected, WordGroups::from_dicts(dict, dict2).groups);
    }

    #[rstest]
    #[case(&[], vec![])]
    #[case(&[(1, "a"), (1, "a")], vec![vec!["a"]])]
    #[case(&[(1, "a"), (1, "b")], vec![vec!["a", "b"]])]
    #[case(&[(1, "a"), (1, "b"), (1, "c"), (2, "b"), (2, "a")], vec![vec!["a", "b", "c"]])]
    #[case(&[(1, "a"), (1, "b"), (2, "d"), (2, "c"), (3, "e"), (3, "a")], vec![vec!["a", "b", "e"], vec!["c", "d"]])]
    #[case(&[(1, "a"), (1, "b"), (1, "c"), (2, "d"), (2, "c"), (3, "e"), (3, "a"),], vec![vec!["a", "b", "c", "d", "e"]])]
    fn test_group_builder(
        #[case] input: &[(usize, &str)],
        #[case] expected: Vec<Vec<&str>>,
    ) {
        let mut builder = GroupBuilder::<usize, String>::new();
        for (set, item) in input {
            builder.add(*set, *item);
        }
        let mut groups: Vec<Vec<String>> = builder
            .into_iter()
            .map(|mut v| {
                v.sort();
                v
            })
            .collect();
        groups.sort_by(|g, h| g[0].cmp(&h[0]));
        assert_eq!(expected, groups);
    }
}
