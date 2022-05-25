use rstest::*;
//use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

fn cmd() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("merge-word-groups").unwrap()
}

#[rstest]
#[case("", "")]
#[case("aa bb cc\n bb b\n a c\n", "a c\naa b bb cc\n")]
fn test_stdin(#[case] input: &str, #[case] expected: &str) {
    cmd()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::eq(expected));
}

#[rstest]
#[case("", "")]
#[case("aa bb cc\n bb b\n a c\n", "a c\naa b bb cc\n")]
fn test_single_file(#[case] input: &str, #[case] expected: &str) {
    let dir = assert_fs::TempDir::new().unwrap();
    let input_path = dir.child("input.txt");

    input_path.write_str(input).unwrap();

    cmd()
        .arg(input_path.as_os_str())
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::eq(expected));

    dir.close().unwrap();
}

#[rstest]
#[case("", "", "")]
#[case("aa bb cc\n bb b\n a c\n", "d c e\nbb f\n", "a c d e\naa b bb cc f\n")]
fn test_multiple_files(
    #[case] input: &str,
    #[case] input2: &str,
    #[case] expected: &str,
) {
    let dir = assert_fs::TempDir::new().unwrap();
    let input_path = dir.child("input.txt");
    let input2_path = dir.child("input2.txt");

    input_path.write_str(input).unwrap();
    input2_path.write_str(input2).unwrap();

    cmd()
        .args(&[input_path.as_os_str(), input2_path.as_os_str()])
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::eq(expected));

    dir.close().unwrap();
}
