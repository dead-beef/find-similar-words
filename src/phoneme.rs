use std::ffi::{c_void, CStr};
use std::os::raw::c_char;

use espeakng::Speaker;
use espeakng_sys::{espeakCHARS_UTF8, espeak_TextToPhonemes};

use crate::iter::IteratorEx;
//use crate::util::map_char;

pub struct TextToPhonemes<'a> {
    _speaker: &'a Speaker,
    ascii: bool,
    buf: Vec<u8>,
}

impl<'a> TextToPhonemes<'a> {
    pub fn new(speaker: &'a Speaker, ascii: bool) -> Self {
        Self {
            _speaker: speaker,
            ascii,
            buf: Vec::new(),
        }
    }

    fn text_to_ptr(&mut self, text: &str) -> *const c_char {
        self.buf.clear();
        self.buf.extend(text.as_bytes());
        self.buf.push(0);
        self.buf.as_ptr().cast()
    }

    pub fn text_to_phonemes(&mut self, text: &str) -> String {
        let text_cstring_ptr = self.text_to_ptr(text);
        //println!("ptr = {:?}", text_cstring_ptr);
        let output = unsafe {
            CStr::from_ptr(espeak_TextToPhonemes(
                &mut text_cstring_ptr.cast() as *mut *const c_void,
                espeakCHARS_UTF8 as i32,
                if self.ascii { 0 } else { 2 },
            ))
        };
        //println!("ptr = {:?}", text_cstring_ptr);
        output.to_string_lossy().to_string()
    }
}

pub fn normalize_phonemes(phonemes: &str) -> String {
    phonemes
        .chars()
        .filter(|c| {
            //dbg!("filter char {:?} ({})", *c, u32::from(*c));
            !(IPA_MODIFIERS.contains(&u32::from(*c)) || c.is_whitespace())
        })
        .map(|c| match c {
            'a' | 'ä' | 'ɐ' | 'ɑ' | 'ʌ' => 'a',
            'e' | 'æ' | 'ɛ' | 'œ' | 'ɜ' => 'e',
            'i' | 'ɨ' | 'ɪ' => 'i',
            'o' | 'ɔ' | 'ɒ' | 'ɵ' | 'ʊ' => 'o',
            'u' | 'ʉ' => 'u',
            'y' | 'ʏ' | 'ø' => 'y',
            'ɘ' | 'ɤ' | 'ɞ' | 'ə' | 'ɯ' => 'ɘ',
            'b' | 'ʙ' => 'b',
            'd' | 'ɖ' | 'ɟ' => 'd',
            'f' | 'ɸ' => 'f',
            'g' | 'ɢ' => 'g',
            'j' | 'ʎ' | 'ʝ' => 'j',
            'k' | 'q' => 'k',
            'l' | 'ɭ' | 'ɫ' | 'ʟ' => 'l',
            'm' | 'ɱ' => 'm',
            'n' | 'ɳ' | 'ɲ' | 'ŋ' | 'ɴ' => 'n',
            'r' | 'ɾ' | 'ɹ' | 'ɽ' | 'ɻ' | 'ʀ' | 'ʁ' => 'r',
            't' | 'ʈ' | 'c' => 't',
            'v' | 'β' | 'ʋ' => 'v',
            'x' | 'ɣ' | 'χ' | 'ħ' | 'h' | 'ɦ' => 'x',
            'θ' | 'ð' => 'θ',
            'ʃ' | 'ʂ' | 'ç' => 'ʃ',
            'ʒ' | 'ʐ' => 'ʒ',
            'ɰ' | 'ʕ' => 'ɰ',
            c => c,
        })
        .dedup()
        .collect()
}

const IPA_MODIFIERS: std::ops::Range<u32> = 688..880;

/*declare_static_array!(pub NORMALIZE_PHONEMES_CHAR_MAP, (&'static str, char), [
    ("aäɐɑʌ", 'a'),
    ("eæɛœɜ", 'e'),
    ("iɨɪ", 'i'),
    ("oɔɒɵʊ", 'o'),
    ("uʉ", 'u'),
    ("yʏø", 'y'),
    ("ɘɤɞəɯ", 'ɘ'),

    ("bʙ", 'b'),
    ("dɖɟ", 'd'),
    ("fɸ", 'f'),
    ("gɢ", 'g'),
    ("jʎʝ", 'j'),
    ("kq", 'k'),
    ("lɭɫʟ", 'l'),
    ("mɱ", 'm'),
    ("nɳɲŋɴ", 'n'),
    ("rɾɹɽɻʀʁ", 'r'),
    ("tʈc", 't'),
    ("vβʋ", 'v'),
    ("xɣχħhɦ", 'x'),
    ("θð", 'θ'),
    ("ʃʂç", 'ʃ'),
    ("ʒʐ", 'ʒ'),
    ("ɰʕ", 'ɰ')
]);*/

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("", "")]
    #[case("        ", "")]
    #[case("ʌbɹiˌviejˈʃʌnz", "abriviejʃanz")]
    #[case("m ʌ nʲ ɪ t o rʲ ɪ n k", "manitorink")]
    fn test_normalize_phonemes(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, normalize_phonemes(input));
    }
}
