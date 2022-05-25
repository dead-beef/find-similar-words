use std::io::Write;

use argparse::{ArgumentParser, Collect};

use find_similar_words::util::ArgParser;

#[derive(Debug, PartialEq)]
pub struct Args {
    pub input_filenames: Vec<String>,
}

impl Args {
    pub fn new() -> Self {
        Self {
            input_filenames: Vec::new(),
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
            parser.set_description("Merge results from find-similar-words.");
            parser.refer(&mut opts.input_filenames).add_argument(
                "file",
                Collect,
                "Files to merge (default: stdin)",
            );
            parser.parse(args, stdout, stderr)?;
        }
        Ok(opts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::io::Cursor;

    #[rstest]
    #[case(&["cmd", "-h"], Err(0))]
    #[case(&["cmd"], Ok(Args::new()))]
    #[case(&["cmd", "xx", "yy", "zz"], Ok(Args {input_filenames: vec![String::from("xx"), String::from("yy"), String::from("zz")]}))]
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
