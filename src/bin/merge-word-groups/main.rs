use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};

use streaming_iterator::StreamingIterator;

use args::Args;

use find_similar_words::iter::lines;
use find_similar_words::util::ArgParser;
use find_similar_words::word_groups::{GroupBuilder, WordGroups};

mod args;

fn merge_file<I: io::BufRead>(
    builder: &mut GroupBuilder<usize, String>,
    line_number: &mut usize,
    mut file: I,
) {
    let mut lines = lines(&mut file);
    while let Some(line) = lines.next() {
        builder.extend(*line_number, line.split_whitespace());
        *line_number += 1;
    }
}

fn merge(fnames: Vec<String>) -> io::Result<WordGroups> {
    let mut builder = GroupBuilder::<usize, String>::new();
    let mut line_number = 0usize;
    if fnames.is_empty() {
        merge_file(&mut builder, &mut line_number, io::stdin().lock());
    } else {
        for fname in fnames {
            let file = File::open(fname)?;
            merge_file(&mut builder, &mut line_number, BufReader::new(file));
        }
    }
    Ok(builder.into_iter().collect())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_env_args();
    let groups = merge(args.input_filenames)?;
    print!("{}", groups);
    Ok(())
}
