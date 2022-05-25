use std::error::Error;
use std::fmt::Display;

use args::Args;
use options::Options;

use find_similar_words::dictionary::{Dictionary, Word};
use find_similar_words::util::ArgParser;
use find_similar_words::word_groups::WordGroups;

mod args;
mod options;

fn print_result<I: IntoIterator>(words: I)
where
    I::Item: Display,
{
    let mut first = true;
    for word in words {
        if first {
            print!("{}", word);
            first = false;
        } else {
            print!(" {}", word);
        }
    }
    println!();
}

fn search(dict: &Dictionary, dict2: &Dictionary, max_distance: usize) -> usize {
    let mut words = Vec::<&Word>::new();
    let mut res = 0_usize;
    for word in dict.iter() {
        let similar = dict2
            .find_similar(word, max_distance)
            .filter(|w| *w != word);
        words.push(word);
        words.extend(similar);
        if words.len() > 1 {
            res += 1;
            print_result(words.iter());
        }
        words.clear();
    }
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_env_args();
    let opts = Options::from_args(&args)?;
    let result_count = if opts.max_distance == 0 {
        let results = if let Some(dict2) = opts.dict2 {
            WordGroups::from_dicts(opts.dict, dict2)
        } else {
            WordGroups::from_dict(opts.dict)
        };
        print!("{}", results);
        results.len()
    } else {
        search(
            &opts.dict,
            opts.dict2.as_ref().unwrap_or(&opts.dict),
            opts.max_distance,
        )
    };
    eprintln!("{} results", result_count);
    Ok(())
}
