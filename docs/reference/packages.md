# Packages

A _package_ is a collection of [exports](exports.md) with metadata attached. Packages can depend on
other packages. Package metadata is found in the `starpkg.toml` manifest file, and its exports can
be found in the `src` directory.

## Package manifest

### `name`
The name of the package. Must be unique when compared with its dependency tree, and cannot:

- be "pm64"
- contain punctuation other than underscores

### `version`
A valid [semantic version](https://semver.org/spec/v2.0.0.html) string. It is expected
that semantic versioning is adhered to by package authors, but this is not enforced.

### `[dependencies]`
A table of dependency names each mapped to one of the following:

- A [version range](https://docs.npmjs.com/misc/semver#ranges) string.
- `{ path = "path/to/package" }` - Reads the dependency package from the given directory.
