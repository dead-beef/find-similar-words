# find-similar-words

![Master](https://github.com/dead-beef/find-similar-words/workflows/Master/badge.svg) ![PR validation](https://github.com/dead-beef/find-similar-words/workflows/PR%20validation/badge.svg)

## Requirements

In order to run own copy of the project one must fulfill the following requirements.

- [Rust](https://www.rust-lang.org/)
- [eSpeak NG](https://github.com/espeak-ng/espeak-ng/)

## Installation

```bash
cargo install --path .
```

## Building

```bash
cargo build
```

```bash
cargo build --release
```

## Usage

```
> cargo run --release --bin create-ipa-dict -- --help

Usage:
  target/release/create-ipa-dict [OPTIONS] [INPUT]

Create IPA dictionary from a word list.

Positional arguments:
  input                 Set input file (default: stdin)

Optional arguments:
  -h,--help             Show this help message and exit
  -L,--list-languages   Print supported languages and exit
  -l,--language LANGUAGE
                        Set language (default: detect)
  -v,--voice FILE       Set espeak voice (default: use highest priority voice
                        for language)
  -a,--ascii            Use espeak's ascii phoneme names
  -o,--output FILE      Set output file (default: stdout)
```

```
> cargo run --release --bin find-similar-words -- --help

Usage:
  target/release/find-similar-words [OPTIONS] [FILE ...]

Find words with similar pronunciations.

Positional arguments:
  file                  Dictionary files (tsv) (max: 2) (default: stdin)

Optional arguments:
  -h,--help             Show this help message and exit
  -n,--normalize        Normalize word transcriptions
  -l,--min-length MIN_LENGTH
                        Set minimum word length (default: none)
  -L,--max-length MAX_LENGTH
                        Set maximum word length (default: none)
  -d,--max-distance MAX_DISTANCE
                        Set max levenshtein distance between word
                        transcriptions (default: 0)
```

```
> cargo run --release --bin merge-word-groups -- --help

Usage:
  target/release/merge-word-groups [FILE ...]

Merge results from find-similar-words.

Positional arguments:
  file                  Files to merge (default: stdin)

Optional arguments:
  -h,--help             Show this help message and exit
```

## Testing

```bash
cargo test
```

## Linting

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
```

## Licenses

- [`find-similar-words`](LICENSE)
