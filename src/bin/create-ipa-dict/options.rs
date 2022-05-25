use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;

use streaming_iterator::{Chain, StreamingIterator};

use crate::args::Args;

use find_similar_words::iter::{lines, str_lines, Lines, StrLines};
use find_similar_words::language::Languages;
use find_similar_words::language_detection::detect_file_language;
use find_similar_words::util::{open_input_file, open_output_file};

#[derive(Debug)]
pub struct NoVoiceForLanguage {
    language: String,
}

pub struct Input {
    file: Box<dyn io::BufRead>,
    start: String,
}

pub struct Options {
    pub input: Input,
    pub output: Box<dyn io::Write>,
    pub voice: String,
    pub ascii: bool,
}

impl<'a> From<&'a str> for NoVoiceForLanguage {
    fn from(s: &'a str) -> Self {
        Self {
            language: String::from(s),
        }
    }
}

impl Display for NoVoiceForLanguage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "No espeak voice found for language {:?}",
            self.language
        ))
    }
}

impl Error for NoVoiceForLanguage {}

impl Input {
    fn get_espeak_voice(
        &mut self,
        args: &Args,
    ) -> Result<String, Box<dyn Error>> {
        let languages = Languages::get_supported();
        match args.voice.as_ref() {
            Some(voice) => Ok(voice.clone()),
            None => {
                let lang: &str = match args.language.as_ref() {
                    Some(l) => l,
                    None => {
                        eprintln!("Detecting language...");
                        let detected = detect_file_language(&mut self.file)?;
                        self.start = detected.partial_file_contents;
                        eprintln!("Detected language {}", detected.language);
                        detected.language
                    },
                };

                match languages.get(lang).and_then(|l| l.default_espeak_voice())
                {
                    Some(v) => Ok(v.filename().clone()),
                    None => Err(Box::new(NoVoiceForLanguage::from(lang))),
                }
            },
        }
    }

    pub fn from_args(args: &Args) -> io::Result<Self> {
        let file = open_input_file(&args.input_filename)?;
        Ok(Self {
            file,
            start: String::new(),
        })
    }

    pub fn iter(&mut self) -> Chain<StrLines, Lines<Box<dyn io::BufRead>>> {
        str_lines(&self.start).chain(lines(&mut self.file))
    }
}

impl Options {
    pub fn from_args(args: &Args) -> Result<Self, Box<dyn Error>> {
        let mut input = Input::from_args(args)?;
        let output = open_output_file(&args.output_filename)?;
        let voice = input.get_espeak_voice(args)?;
        Ok(Self {
            input,
            output,
            voice,
            ascii: args.ascii,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::io::Cursor;
    use streaming_iterator::StreamingIterator;

    #[rstest]
    #[case("", "", &[])]
    #[case("", "xx\nyy\n", &["xx", "yy"])]
    #[case("xx\nyy", "1\n2\n3\n", &["xx", "yy", "1", "2", "3"])]
    fn test_input_iter(
        #[case] start: &str,
        #[case] file_contents: &str,
        #[case] expected: &[&str],
    ) {
        let mut input = Input {
            start: String::from(start),
            file: Box::new(Cursor::new(String::from(file_contents))),
        };
        let res: Vec<String> = input.iter().owned().collect();
        assert_eq!(expected, res);
    }
}
