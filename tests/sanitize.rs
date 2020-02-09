use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;
use indoc::indoc;

fn tempdir() -> assert_fs::TempDir {
    assert_fs::TempDir::new().unwrap()
}

fn starpkg() -> Command {
    Command::cargo_bin("starpkg").unwrap()
}

#[test]
fn bad_package_name() {
    let dir = tempdir();

    dir.child("starpkg.toml").write_str(indoc!(r#"
        name = "terrible package name"
        version = "0.1.0"
    "#)).unwrap();

    starpkg()
        .arg("build")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid package name"));

    dir.child(".build").assert(predicate::path::missing());
}

#[test]
fn no_pkg_name() {
    let dir = tempdir();

    dir.child("starpkg.toml").write_str(indoc!(r#"
        name = ""
        version = "0.1.0"
    "#)).unwrap();

    starpkg()
        .arg("build")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid package name"));

    dir.child(".build").assert(predicate::path::missing());
}

#[test]
fn pkg_name_pm64() {
    let dir = tempdir();

    dir.child("starpkg.toml").write_str(indoc!(r#"
        name = "pm64"
        version = "0.1.0"
    "#)).unwrap();

    starpkg()
        .arg("build")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid package name"));

    dir.child(".build").assert(predicate::path::missing());
}

#[test]
fn bad_dependency_name() {
    let dir = tempdir();

    dir.child("starpkg.toml").write_str(indoc!(r#"
        name = "ok"
        version = "0.1.0"

        [dependencies]
        _lol_ = { path = ".." }
    "#)).unwrap();

    starpkg()
        .arg("build")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid dependency name"));

    dir.child(".build").assert(predicate::path::missing());
}

#[test]
fn dep_name_same_as_pkg_name() {
    let dir = tempdir();

    dir.child("starpkg.toml").write_str(indoc!(r#"
        name = "word"
        version = "0.1.0"

        [dependencies]
        word = { path = ".." }
    "#)).unwrap();

    starpkg()
        .arg("build")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid dependency name"));

    dir.child(".build").assert(predicate::path::missing());
}

#[test]
fn bad_export_name_string() {
    let dir = tempdir();

    dir.child("starpkg.toml").write_str(indoc!(r#"
        name = "test_pkg"
        version = "0.1.0"
    "#)).unwrap();

    dir.child("src/string/naughty.str").write_str(indoc!(r#"
        #string:01:(this is very naughty)
        Sample text[WAIT][END]
    "#)).unwrap();

    starpkg()
        .arg("build")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid export name"));

    dir.child(".build").assert(predicate::path::missing());
}

#[test]
fn using_private_export_locally() {
    let dir = tempdir();

    dir.child("starpkg.toml").write_str(indoc!(r#"
        name = "test_pkg"
        version = "0.1.0"
    "#)).unwrap();

    dir.child("src/actor/_cool_actor/_cool_actor.toml").write_str(indoc!(r#"
        name = "namestring"
        tattle = "tattlestring"
    "#)).unwrap();

    // Note: this is technically an invalid actor script so this test may fail in the future if
    // decent script sanitization is added!
    dir.child("src/actor/_cool_actor/_cool_actor.bscr").write_str(indoc!(r#"
        #new:Actor $Actor
        [Index] {Actor:_cool_actor}
    "#)).unwrap();

    dir.child("src/string/strings.str").write_str(indoc!(r#"
        #string:01:(namestring)
        [END]

        #string:01:(tattlestring)
        [END]
    "#)).unwrap();

    starpkg()
        .arg("build")
        .current_dir(dir.path())
        .assert()
        .success();

    dir.child(".build").assert(predicate::path::exists());
}

#[test]
fn no_using_private_export_externally() {
    let dir = tempdir();

    dir.child("starpkg.toml").write_str(indoc!(r#"
        name = "test_pkg"
        version = "0.1.0"

        [dependencies]
        outside = { path = "outside" }
    "#)).unwrap();

    dir.child("src/actor/_cool_actor/_cool_actor.toml").write_str(indoc!(r#"
        name = "outside/namestring"
        tattle = "outside/_tattlestring"
    "#)).unwrap();

    // Note: this is technically an invalid actor script so this test may fail in the future if
    // decent script sanitization is added!
    dir.child("src/actor/_cool_actor/_cool_actor.bscr").write_str(indoc!(r#"
        #new:Actor $Actor
        [Index] {Actor:_cool_actor}
    "#)).unwrap();

    dir.child("outside/starpkg.toml").write_str(indoc!(r#"
        name = "outside"
        version = "0.1.0"
    "#)).unwrap();

    dir.child("outside/src/string/strings.str").write_str(indoc!(r#"
        #string:01:(namestring)
        [END]

        #string:01:(_tattlestring)
        [END]
    "#)).unwrap();

    starpkg()
        .arg("build")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("identifier '_tattlestring'"));
}
