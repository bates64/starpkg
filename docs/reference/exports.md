# Exports

An _export_ is a structure that a package makes available to packages that depend on it, and,
ultimately, will be compiled into the mod itself. Exports are found in the `src` directory of
packages.

### Identifiers

Exports can cross-reference eachother using identifiers in the form `<package>/<export>`. If
a package name is not provided (ie. the identifier is just a bare export name), it is assumed to be
the name of the package which provides the export.

Export names:
- cannot contain punctuation other than underscores
- must be unique in its export type group of packages, ie. a package cannot define two sprites named the same, but may name a string and an actor the same
