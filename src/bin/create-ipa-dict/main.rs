use std::error::Error;
use std::io::Write;

use streaming_iterator::StreamingIterator;

use args::Args;
use options::Options;

use find_similar_words::language::Languages;
use find_similar_words::phoneme::TextToPhonemes;
use find_similar_words::util::ArgParser;

mod args;
mod options;

fn list_languages() {
    let languages = Languages::get_supported();
    let min_width = 8;
    let width = languages
        .iter()
        .flat_map(|l| l.espeak_voices().iter().map(|v| v.filename().len()))
        .fold(min_width, |r, l| r.max(l));
    println!("Languages:");
    for language in languages.iter() {
        println!("\nName:   {}", language.name());
        println!("Voices:");
        println!("{:>width$} {:>width$}", "Filename", "Priority");
        for voice in language.espeak_voices().iter() {
            println!(
                "{:>width$} {:>width$}",
                voice.filename(),
                voice.priority()
            );
        }
    }
}

fn process_input(args: &Args) -> Result<(), Box<dyn Error>> {
    let mut opts = Options::from_args(args)?;

    let mut speaker = espeakng::initialise(None)?.lock();
    speaker.set_voice_raw(&opts.voice)?;
    let mut speaker = TextToPhonemes::new(&*speaker, opts.ascii);

    let mut lines = opts.input.iter();
    while let Some(line) = lines.next() {
        let line = line.trim();
        if !line.is_empty() {
            let phonemes = speaker.text_to_phonemes(line);
            if phonemes.is_empty() {
                eprintln!("Warning: no phonemes found for {:?}", line);
            } else {
                writeln!(opts.output, "{}\t{}", line, phonemes)?;
                //output.flush()?;
            }
        }
    }

    opts.output.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_env_args();
    if args.list_languages {
        list_languages();
    } else {
        process_input(&args)?;
    }
    Ok(())
}
