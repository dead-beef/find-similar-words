use rstest::*;
//use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

fn cmd() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("find-similar-words").unwrap()
}

#[rstest]
#[case("", "")]
#[case("cc\ta\n bb\tb\n cc\ta\n aa\ta\n", "aa cc\n")]
fn test_default(#[case] input: &str, #[case] expected: &str) {
    cmd()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::eq(expected));
}

#[rstest]
#[case("c\ta\n bb\tb\n c\ta\n aa\ta\n ddd\tb\n", 2, 3, "bb ddd\n")]
#[case("c\ta\n bb\tb\n c\ta\n aa\ta\n", 1, 2, "aa c\n")]
#[case("c\ta\n bb\tb\n c\ta\n aa\ta\n ddd\tb\n", 1, 4, "aa c\nbb ddd\n")]
fn test_length(
    #[case] input: &str,
    #[case] min_length: usize,
    #[case] max_length: usize,
    #[case] expected: &str,
) {
    cmd()
        .args(&["-l", &min_length.to_string(), "-L", &max_length.to_string()])
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::eq(expected));
}

#[rstest]
#[case("a\tx\n ab\txy\n bb\tyy\n", 0, "")]
#[case("a\tx\n ab\txy\n bb\tyy\n", 1, "a ab\nab a bb\nbb ab\n")]
#[case("a\tx\n ab\txy\n bb\tyy\n", 2, "a ab bb\nab a bb\nbb a ab\n")]
fn test_distance(
    #[case] input: &str,
    #[case] max_distance: usize,
    #[case] expected: &str,
) {
    cmd()
        .args(&["-d", &max_distance.to_string()])
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::eq(expected));
}

#[rstest]
#[case("cc\ta\n bb\tb\n cc\ta\n aa\ta\n", "aa cc\n")]
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
#[case("cc\ta\n bb\tb\n cc\ta\n aa\ta\n", "cc\tx\n ab\tb\n", "ab bb\n")]
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
