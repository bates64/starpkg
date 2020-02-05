# Actor

Actors are battle-participating entities, typically enemies. They are located in the
`src/actor/NAME` directory.

### `NAME.toml`

A TOML file with two [string](string.md) identifiers: `name` and `tattle`.

### `NAME.bscr`

A [script](../scripts.md) exporting a new Actor struct named `$Actor`. The `[Index]` field of the
struct should be the [`{Actor:NAME}` reference expression](../scripts.md#reference-expressions).
