use espeakng;

#[derive(Debug, Clone)]
pub struct EspeakVoice {
    filename: String,
    priority: i8,
}

#[derive(Debug)]
pub struct Language {
    name: String,
    espeak_voices: Vec<EspeakVoice>,
}

#[derive(Debug)]
pub struct Languages {
    languages: Vec<Language>,
}

impl EspeakVoice {
    pub fn new(voice: &espeakng::Voice, priority: i8) -> Self {
        Self {
            filename: voice.filename.clone(),
            priority,
        }
    }

    pub fn filename(&self) -> &String {
        &self.filename
    }

    pub fn priority(&self) -> i8 {
        self.priority
    }
}

impl Language {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            espeak_voices: Vec::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn espeak_voices(&self) -> &Vec<EspeakVoice> {
        &self.espeak_voices
    }

    pub fn default_espeak_voice(&self) -> Option<&EspeakVoice> {
        self.espeak_voices.get(0)
    }

    pub fn add_espeak_voice(&mut self, voice: EspeakVoice) {
        let pos = self
            .espeak_voices
            .binary_search_by(|v| voice.priority.cmp(&v.priority));
        let idx = match pos {
            Ok(pos) => pos,
            Err(pos) => pos,
        };
        self.espeak_voices.insert(idx, voice);
    }

    pub fn is(&self, name: &str) -> bool {
        self.name == name
    }
}

impl Languages {
    pub fn new() -> Self {
        Self {
            languages: Vec::new(),
        }
    }

    pub fn get_supported() -> Self {
        let res = Self::from_espeak();
        if res.is_empty() {
            eprintln!("Warning: no supported languages found");
        }
        res
    }

    pub fn index(&self, language: &str) -> Option<usize> {
        self.languages
            .iter()
            .enumerate()
            .filter(|l| l.1.is(language))
            .map(|l| l.0)
            .next()
    }

    pub fn get(&self, language: &str) -> Option<&Language> {
        self.index(language).map(|i| &self.languages[i])
    }

    /*pub fn get_mut(&mut self, language: &str) -> Option<&mut Language> {
        self.index(language).map(|i| &mut self.languages[i])
    }

    pub fn get_or_create(&mut self, language: &str) -> &Language {
        if let Some(i) = self.index(language) {
            &self.languages[i]
        } else {
            self.languages.push(Language::new(language));
            self.languages.last().unwrap()
        }
    }*/

    fn get_or_create_mut(&mut self, language: &str) -> &mut Language {
        if let Some(i) = self.index(language) {
            &mut self.languages[i]
        } else {
            self.languages.push(Language::new(language));
            self.languages.last_mut().unwrap()
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Language> {
        self.languages.iter()
    }

    /*pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Language> {
        self.languages.iter_mut()
    }*/

    pub fn len(&self) -> usize {
        self.languages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.languages.is_empty()
    }

    pub fn from_espeak() -> Self {
        let mut res = Self::new();
        if let Err(e) = espeakng::initialise(None) {
            eprintln!("Warning: failed to initialize espeak: {:?}", e);
        }
        let voices = espeakng::Speaker::get_voices();
        for voice in voices.iter() {
            for language in voice.languages.iter() {
                let voice = EspeakVoice::new(voice, language.priority);
                res.get_or_create_mut(&language.name)
                    .add_espeak_voice(voice);
            }
        }
        res
    }
}

impl Default for Languages {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_voice_get() {
        let filename = "filename";
        let priority = 2_i8;
        let voice = EspeakVoice {
            filename: String::from(filename),
            priority,
        };
        assert_eq!(filename, voice.filename());
        assert_eq!(priority, voice.priority());
    }

    #[test]
    fn test_language_init() {
        let name = "ts";
        let l = Language::new(name);
        assert_eq!(name, l.name());
        assert_eq!(true, l.default_espeak_voice().is_none());
        assert_eq!(0, l.espeak_voices().len());
    }

    #[test]
    fn test_language_is() {
        let name = "ts";
        let l = Language::new(name);
        assert_eq!(true, l.is(name));
        assert_eq!(false, l.is("xx"));
    }

    #[rstest]
    #[case(&[("x", 0)], "x", &["x"])]
    #[case(&[("x", 0), ("y", 1), ("z", 2)], "z", &["z", "y", "x"])]
    #[case(&[("x", 0), ("y", 2), ("z", 1), ("w", -1)], "y", &["y", "z", "x", "w"])]
    fn test_language_add_voice(
        #[case] input: &[(&str, i8)],
        #[case] expected_default: &str,
        #[case] expected_order: &[&str],
    ) {
        let mut l = Language::new("ts");
        for (filename, priority) in input {
            l.add_espeak_voice(EspeakVoice {
                filename: String::from(*filename),
                priority: *priority,
            });
        }
        let default = l.default_espeak_voice().map(|v| &v.filename);
        let order: Vec<&str> =
            l.espeak_voices().iter().map(|v| &v.filename[..]).collect();
        assert_eq!(false, default.is_none());
        assert_eq!(expected_default, default.unwrap());
        assert_eq!(expected_order, order);
    }

    #[test]
    fn test_languages_init() {
        let ls = Languages::new();
        assert_eq!(0, ls.len());
        assert_eq!(true, ls.iter().next().is_none());
    }

    #[rstest]
    #[case(&["x", "y"], &["x", "y"])]
    #[case(&["x", "y", "y", "z", "y", "x", "x"], &["x", "y", "z"])]
    fn test_languages_get_or_create(
        #[case] input: &[&str],
        #[case] expected: &[&str],
    ) {
        let mut ls = Languages::new();
        for l in input {
            ls.get_or_create_mut(l);
        }
        let lnames: Vec<&str> = ls.iter().map(|l| &l.name[..]).collect();
        assert_eq!(expected, lnames);
        assert_eq!(expected.len(), ls.len());
    }

    #[rstest]
    #[case(&["x", "y"], "x", Some(0))]
    #[case(&["x", "y"], "y", Some(1))]
    #[case(&["x", "y"], "z", None)]
    fn test_languages_get(
        #[case] input: &[&str],
        #[case] search: &str,
        #[case] expected: Option<usize>,
    ) {
        let ls = Languages {
            languages: input.iter().map(|s| Language::new(*s)).collect(),
        };
        assert_eq!(expected, ls.index(search));
        assert_eq!(
            expected.map(|i| input[i]),
            ls.get(search).map(|l| &l.name[..])
        );
    }

    #[test]
    fn test_languages_get_supported() {
        let ls = Languages::get_supported();
        assert_eq!(true, ls.len() > 0);
    }
}
