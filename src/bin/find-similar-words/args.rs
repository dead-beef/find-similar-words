use std::io::Write;

use argparse::{ArgumentParser, Collect, Store, StoreTrue};

use find_similar_words::util::ArgParser;

#[derive(Debug, PartialEq)]
pub struct Args {
    pub input_filenames: Vec<String>,
    pub normalize: bool,
    pub max_distance: usize,
    pub min_word_length: usize,
    pub max_word_length: usize,
}

impl Args {
    pub fn new() -> Self {
        Self {
            input_filenames: Vec::new(),
            normalize: false,
            max_distance: 0,
            min_word_length: 0,
            max_word_length: usize::MAX,
        }
    }
}

impl ArgParser for Args {
    fn parse<O: Write, E: Write>(
        args: Vec<String>,
        stdout: &mut O,
        stderr: &mut E,
    ) -> Result<Self, i32> {
        let mut opts = Self::new();
        {
            let mut parser = ArgumentParser::new();
            parser.set_description("Find words with similar pronunciations.");
            parser.refer(&mut opts.normalize).add_option(
                &["-n", "--normalize"],
                StoreTrue,
                "Normalize word transcriptions",
            );
            parser.refer(&mut opts.min_word_length).add_option(
                &["-l", "--min-length"],
                Store,
                "Set minimum word length (default: none)",
            );
            parser.refer(&mut opts.max_word_length).add_option(
                &["-L", "--max-length"],
                Store,
                "Set maximum word length (default: none)",
            );
            parser.refer(&mut opts.max_distance).add_option(
                &["-d", "--max-distance"],
                Store,
                "Set max levenshtein distance between word transcriptions (default: 0)",
            );
            parser.refer(&mut opts.input_filenames).add_argument(
                "file",
                Collect,
                "Dictionary files (tsv) (max: 2) (default: stdin)",
            );
            parser.parse(args, stdout, stderr)?;
        }
        if opts.input_filenames.len() > 2 {
            writeln!(stderr, "Too many file arguments").map_err(|_| 2)?;
            Err(2)
        } else {
            Ok(opts)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::io::Cursor;

    #[rstest]
    #[case(&["cmd", "-h"], Err(0))]
    #[case(&["cmd", "-L", "-l"], Err(2))]
    #[case(&["cmd", "xx", "yy", "zz"], Err(2))]
    #[case(&["cmd"], Ok(Args::new()))]
    #[case(&["cmd", "-n", "-l", "1", "-d", "3", "-L", "2", "xx", "yy"], Ok(Args {input_filenames: vec![String::from("xx"), String::from("yy")], normalize: true, max_distance: 3, min_word_length: 1, max_word_length: 2}))]
    fn test_args_parse(
        #[case] args: &[&str],
        #[case] expected: Result<Args, i32>,
    ) {
        let args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();
        let mut stdout = Cursor::new(Vec::<u8>::new());
        let mut stderr = Cursor::new(Vec::<u8>::new());
        assert_eq!(expected, Args::parse(args, &mut stdout, &mut stderr));
    }
}
