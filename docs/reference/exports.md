# Exports

An _export_ is a structure that a package makes available to packages that depend on it, and,
ultimately, will be compiled into the mod itself. Exports are found in the `src` directory of
packages.

### Identifiers

Exports can cross-reference eachother using identifiers in the form `package_name/export_name`. If
a package name is not provided (ie. the identifier is just a bare export name), it is assumed to be
the name of the package which provides the export.

### Export types

Types of exports include:

- [string](exports/string.md)
- [sprite](exports/sprite.md)
- [actor](exports/actor.md)
