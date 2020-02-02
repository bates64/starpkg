# Contributing to starpkg

## Commits

- Use [Conventional Commit messages](https://www.conventionalcommits.org/en/v1.0.0-beta.4/)
- If you made a user-facing change, add it to the _Unreleased_ section of the changelog

If your commit makes a breaking change, consider whether it would be feasible to make it minor instead (i.e. make it backwards-compatible). Otherwise, merging it will have to be delayed until the next major release of `starpkg`.

## Documentation

To view documentation locally, use [mdBook](https://github.com/rust-lang/mdBook):

```sh
$ cargo install mdbook
$ cargo mdbook serve
```

## Releases

When releasing:

- Update the changelog (_Unreleased_ -> New version, add empty _Unreleased_ section)
- Bump [Cargo.toml](Cargo.toml) version according to [SemVer](https://semver.org/spec/v2.0.0.html)
- `git add -A && git commit -m "chore(release): [VERSION]"`
- `git tag v[VERSION]`
- `git push && git push --tags`
