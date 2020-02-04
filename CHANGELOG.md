# Changelog

starpkg adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). Note that as starpkg is an application, not a library, only changes relevant to the user (e.g. changes to the CLI or package structure) will be listed here.

## [Unreleased]
### Additions
- Actor & string exports are now supported!
- Scripts can now reference other exports with `{Type:id}` syntax
- `starpkg new` now creates a `.gitignore` file which ignores the starpkg `.build` directory
- `starfmt.toml` now supports a `[dependencies]` section where paths to dependencies can be provided using `{ path = "path/to/dependency/package" }` values

### Changes
- `src/sprites` has been renamed to `src/sprite`

## [0.1.0] - 2020-02-01
Initial release.

[Unreleased]: https://github.com/nanaian/starpkg/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/nanaian/starpkg/releases/tag/v0.1.0
