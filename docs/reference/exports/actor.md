# Actor

`src/actor/<name>` directories. Actors are battle-participating entities, eg. enemies.

### `<name>.toml`

A TOML file with two [string](string.md) identifiers: `name` and `tattle`. These correspond to what
will be displayed ingame with respect to this actor - the starpkg `<name>` is for identifiers only.

### `<name>.bscr`

A [script](../scripts.md) defining a new Actor struct named `$Actor`. The `[Index]` field of the
struct should be the [`{Actor:<name>}` reference expression](../scripts.md#reference-expressions).
