# Changelog

starpkg adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). Note that as starpkg is an application, not a library, only changes relevant to the user (e.g. changes to the CLI or package structure) will be listed here.

## [Unreleased]

## [0.2.0-rc.1] - 2020-02-04
### Fixes
- Fixed the build, including crates.io upload

## [0.2.0-rc.0] - 2020-02-04 [NO BUILD AVAILABLE]
### Additions
- Actor & string exports are now supported!
- Scripts can now reference other exports with `{Type:id}` syntax
- `starpkg new` now creates a `.gitignore` file which ignores the starpkg `.build` directory
- `starfmt.toml` now supports a `[dependencies]` section where paths to dependencies can be provided using `{ path = "path/to/dependency/package" }` values
- Windows build

### Changes
- `src/sprites` has been renamed to `src/sprite`
- Split the Unix build into Linux and macOS (in 0.1.0, the Unix build didn't work on MacOS!)

## [0.1.0] - 2020-02-01
Initial release.

[Unreleased]: https://github.com/nanaian/starpkg/compare/v0.2.0-rc.1...HEAD
[0.2.0-rc.1]: https://github.com/nanaian/starpkg/compare/v0.2.0-rc.0...v0.2.0-rc.1
[0.2.0-rc.0]: https://github.com/nanaian/starpkg/compare/v0.1.0...v0.2.0-rc.0
[0.1.0]: https://github.com/nanaian/starpkg/releases/tag/v0.1.0
