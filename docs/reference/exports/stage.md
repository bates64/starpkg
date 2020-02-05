# Stage

`src/stage/<name>` directories. A stage is a special kind of map where [battles](battle.md) take
place.

### `<name>.toml`

#### `foreground_models`

An array of strings each referencing a model found in `<name>.xml` which appears in the foreground
(ie. infront of the actors).

### `<name>.xml`

The map data itself, generated/edited via Star Rod. In the future, a command will be made available
to easily edit stage maps with a GUI.

### `<name>.bpat`

A [script](../scripts.md) defining two new Script structs named `$Script_BeforeBattle` and
`$Script_AfterBattle`.
