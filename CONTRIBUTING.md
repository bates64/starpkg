# Contributing

:tada: Thank you for contributing to starpkg!! :tada:

## Commits

Use [Conventional Commit messages](https://www.conventionalcommits.org/en/v1.0.0-beta.4/).

If your commit makes a breaking change, consider whether it would be feasible to make it minor
instead (i.e. make it backwards-compatible)! Otherwise, merging it will have to be delayed until the
next major release of starpkg.

For commit scopes, prefer using the name of a starpkg subcommand, eg. `new`, if relevant.

**If you make a user-facing change, make sure to add it to the changelog!**

## Linting

Source files are linted with [clippy](https://crates.io/crates/clippy). Linting errors will fail a
build, so it is suggested that you run clippy locally:

```sh
$ rustup component add clippy
$ cargo clippy
```

## Documentation

To view the user guide locally, use [mdBook](https://github.com/rust-lang/mdBook):

```sh
$ cargo install mdbook
$ cargo mdbook serve
```

## Releases

- Bump [Cargo.toml](Cargo.toml) version according to [semver](https://semver.org/spec/v2.0.0.html)
- Prepend `## [VERSION]` to `CHANGELOG.md`
- `git add -A && git commit -m "chore(release): [VERSION]"`
- `git tag v[VERSION]`
- `git push && git push --tags`
