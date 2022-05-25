use rstest::*;
use std::fmt;
use std::fs;
//use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use predicates::reflection::PredicateReflection;

fn cmd() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("create-ipa-dict").unwrap()
}

fn is_dictionary<'a>(words: &'a [&'a str]) -> IsDictionaryPredicate<'a> {
    IsDictionaryPredicate { words }
}

struct IsDictionaryPredicate<'a> {
    words: &'a [&'a str],
}

impl<'a> fmt::Display for IsDictionaryPredicate<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for word in self.words {
            if first {
                first = false;
                write!(f, "{}", word)?;
            } else {
                write!(f, ", {}", word)?;
            }
        }
        write!(f, "]")
    }
}

impl<'a> PredicateReflection for IsDictionaryPredicate<'a> {}

impl<'a> Predicate<str> for IsDictionaryPredicate<'a> {
    fn eval(&self, variable: &str) -> bool {
        for (i, line) in variable.lines().enumerate() {
            let mut line = line.split('\t');
            let word = match line.next() {
                Some(w) => w,
                None => return false,
            };
            if i >= self.words.len() || self.words[i] != word {
                return false;
            }
        }
        true
    }
}

#[test]
fn test_list_languages() {
    cmd()
        .arg("-L")
        .assert()
        .success()
        .stdout(predicate::function(|stdout: &str| {
            stdout.starts_with("Languages") && stdout.lines().count() > 1
        }));
}

#[rstest]
#[case("", &[])]
#[case("test\nabc", &["test", "abc"])]
fn test_create_dict_stdio(#[case] input: &str, #[case] expected: &[&str]) {
    cmd()
        .args(&["-l", "en"])
        .write_stdin(input)
        .assert()
        .success()
        .stdout(is_dictionary(expected));
}

#[rstest]
#[case("test\nabc", &["test", "abc"])]
fn test_create_dict_files(#[case] input: &str, #[case] expected: &[&str]) {
    let dir = assert_fs::TempDir::new().unwrap();
    let input_path = dir.child("input.txt");
    let output_path = dir.child("output.txt");

    input_path.write_str(input).unwrap();

    cmd()
        .args(&[
            "-l",
            "en",
            "-o",
            output_path.to_str().unwrap(),
            input_path.to_str().unwrap(),
        ])
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    output_path.assert(predicate::path::is_file());

    let output = fs::read_to_string(output_path).unwrap();
    assert!(is_dictionary(expected).eval(&output));

    dir.close().unwrap();
}
