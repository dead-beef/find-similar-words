use std::io::Write;

use argparse::{ArgumentParser, StoreOption, StoreTrue};

use find_similar_words::util::ArgParser;

#[derive(Debug, PartialEq)]
pub struct Args {
    pub input_filename: Option<String>,
    pub output_filename: Option<String>,
    pub language: Option<String>,
    pub voice: Option<String>,
    pub list_languages: bool,
    pub ascii: bool,
}

impl Args {
    pub fn new() -> Self {
        Self {
            input_filename: None,
            output_filename: None,
            language: None,
            voice: None,
            list_languages: false,
            ascii: false,
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
        let res: Result<(), i32>;
        {
            let mut parser = ArgumentParser::new();
            parser.set_description("Create IPA dictionary from a word list.");
            parser.refer(&mut opts.list_languages).add_option(
                &["-L", "--list-languages"],
                StoreTrue,
                "Print supported languages and exit",
            );
            parser.refer(&mut opts.language).add_option(
                &["-l", "--language"],
                StoreOption,
                "Set language (default: detect)",
            );
            parser
                .refer(&mut opts.voice)
                .metavar("FILE")
                .add_option(
                    &["-v", "--voice"],
                    StoreOption,
                    "Set espeak voice (default: use highest priority voice for language)",
                );
            parser.refer(&mut opts.ascii).add_option(
                &["-a", "--ascii"],
                StoreTrue,
                "Use espeak's ascii phoneme names",
            );
            parser.refer(&mut opts.input_filename).add_argument(
                "input",
                StoreOption,
                "Set input file (default: stdin)",
            );
            parser
                .refer(&mut opts.output_filename)
                .metavar("FILE")
                .add_option(
                    &["-o", "--output"],
                    StoreOption,
                    "Set output file (default: stdout)",
                );
            res = parser.parse(args, stdout, stderr);
        }
        res.map(|_| opts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::io::Cursor;

    #[rstest]
    #[case(&["cmd", "-h"], Err(0))]
    #[case(&["cmd", "-i", "yy", "-l"], Err(2))]
    #[case(&["cmd"], Ok(Args::new()))]
    #[case(&["cmd", "-L"], Ok(Args {input_filename: None, output_filename: None, language: None, voice: None, list_languages: true, ascii: false}))]
    #[case(&["cmd", "-o", "xx", "-v", "vv", "-l", "zz", "-a", "yy"], Ok(Args {input_filename: Some(String::from("yy")), output_filename: Some(String::from("xx")), language: Some(String::from("zz")), voice: Some(String::from("vv")), list_languages: false, ascii: true}))]
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
