use std::io;

use whatlang;

use crate::language_code::lang_to_2_letter_code;

#[derive(Debug)]
pub struct DetectedFileLanguage {
    pub language: &'static str,
    pub partial_file_contents: String,
}

pub fn detect_language(text: &str) -> Option<&'static str> {
    let info = whatlang::detect(text)?;
    //dbg!("detect {:?} {:?}", text, &info);
    if info.is_reliable() {
        Some(lang_to_2_letter_code(&info.lang()))
    } else {
        None
    }
}

pub fn detect_file_language<F: io::BufRead>(
    file: &mut F,
) -> io::Result<DetectedFileLanguage> {
    let mut contents = String::new();
    loop {
        let read = file.read_line(&mut contents)?;
        if read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Not enough text for language detection",
            ));
        }
        if let Some(detected) = detect_language(&contents) {
            return Ok(DetectedFileLanguage {
                language: detected,
                partial_file_contents: contents,
            });
        }
    }
    //unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::io::{Cursor, Read};

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[case("test")]
    fn test_detect_language_unreliable(#[case] input: &str) {
        assert_eq!(None, detect_language(input));
    }

    #[rstest]
    #[case("The quick brown fox jumps over the lazy dog and feels as if he were in the seventh heaven of typography", "en")]
    #[case("Stanleys Expeditionszug quer durch Afrika wird von jedermann bewundert", "de")]
    #[case("Le vif renard brun saute par-dessus le chien paresseux", "fr")]
    #[case("Шустрая бурая лисица прыгает через ленивого пса", "ru")]
    fn test_detect_language(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(Some(expected), detect_language(input));
    }

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[case("test")]
    fn test_detect_file_language_unreliable(#[case] input: &str) {
        let mut src = Cursor::new(input);
        assert_eq!(true, detect_file_language(&mut src).is_err());
    }

    #[rstest]
    #[case("Lorem ipsum dolor sit amet,\nconsectetur adipiscing elit, sed\ndo eiusmod tempor incididunt ut labore et dolore\nmagna aliqua. Ut enim ad minim veniam, quis\nnostrud exercitation ullamco laboris nisi ut aliquip\nex ea commodo consequat.", "la")]
    fn test_detect_file_language(#[case] input: &str, #[case] expected: &str) {
        let mut src = Cursor::new(input);
        let res = detect_file_language(&mut src);
        assert_eq!(true, res.is_ok());
        let res = res.unwrap();
        assert_eq!(expected, res.language);
        let mut rem = String::new();
        assert_eq!(true, src.read_to_string(&mut rem).is_ok());
        assert_ne!("", rem);
        assert_eq!(input, res.partial_file_contents + &rem);
    }
}
