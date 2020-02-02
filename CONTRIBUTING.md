# Contributing to starpkg

## Commits

- Use [Conventional Commit messages](https://www.conventionalcommits.org/en/v1.0.0-beta.4/)
- Add to the _Unreleased_ section of the changelog

If your commit makes a breaking change, consider whether it would be feasible to make it minor instead (i.e. make it backwards-compatible). Otherwise, merging it will have to be delayed until the next major release of `starpkg`.



## Releases

When releasing:

- Update the changelog (_Unreleased_ -> New version, add empty _Unreleased_ section)
- Bump [Cargo.toml](Cargo.toml) version according to [SemVer](https://semver.org/spec/v2.0.0.html)
- `git add -A && git commit -m chore(release): [VERSION]`
- `git tag v[VERSION]`
- `git push && git push --tags`
