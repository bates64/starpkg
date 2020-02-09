use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

fn tempdir() -> assert_fs::TempDir {
    assert_fs::TempDir::new().unwrap()
}

fn starpkg() -> Command {
    Command::cargo_bin("starpkg").unwrap()
}

#[test]
fn no_name() {
    let dir = tempdir();

    starpkg()
        .arg("new")
        .current_dir(dir.path())
        .assert()
        .failure();
}

#[test]
fn empty_dir() {
    let dir = tempdir();

    starpkg()
        .arg("new")
        .arg("test_pkg")
        .current_dir(dir.path())
        .assert()
        .success();

    dir.child("starpkg.toml").assert(predicate::path::exists());
}

#[test]
fn unempty_dir() {
    let dir = tempdir();

    dir.child("somefile.txt").touch().unwrap();

    starpkg()
        .arg("new")
        .arg("test_pkg")
        .current_dir(dir.path())
        .assert()
        .success();

    dir.child("test_pkg/starpkg.toml").assert(predicate::path::exists());
}

#[test]
fn package_exists_already() {
    let dir = tempdir();

    dir.child("starpkg.toml").touch().unwrap();

    starpkg()
        .arg("new")
        .arg("test_pkg")
        .current_dir(dir.path())
        .assert()
        .failure();
}

#[test]
fn custom_dir() {
    let dir = tempdir();

    starpkg()
        .arg("-d")
        .arg(dir.path())
        .arg("new")
        .arg("test_pkg")
        .assert()
        .success();

    dir.child("starpkg.toml").assert(predicate::path::exists());
}

#[test]
fn bad_name() {
    let dir = tempdir();

    starpkg()
        .arg("new")
        .arg("bad name for package")
        .current_dir(dir.path())
        .assert()
        .failure();

    starpkg()
        .arg("new")
        .arg("_not_allowed")
        .current_dir(dir.path())
        .assert()
        .failure();

    starpkg()
        .arg("new")
        .arg("not_allowed_")
        .current_dir(dir.path())
        .assert()
        .failure();

    starpkg()
        .arg("new")
        .arg("not/ok")
        .current_dir(dir.path())
        .assert()
        .failure();

    starpkg()
        .arg("new")
        .arg("pm64")
        .current_dir(dir.path())
        .assert()
        .failure();

    dir.child("starpkg.toml").assert(predicate::path::missing());
}

/// #2
#[test]
fn within_existing_package() {
    let dir = tempdir();

    starpkg()
        .arg("new")
        .arg("parent_package")
        .current_dir(dir.path())
        .assert()
        .success();

    starpkg()
        .arg("new")
        .arg("child_package_a")
        .current_dir(dir.path())
        .assert()
        .success();

    let subdir = dir.child("child_package_b");
    subdir.create_dir_all().unwrap();
    starpkg()
        .arg("new")
        .arg("child_package_b")
        .current_dir(subdir.path())
        .assert()
        .success();
}
