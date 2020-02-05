# Battle

`src/battle/<name>` directories.

### `<name>.toml`

A TOML file with the following fields:

#### `stage`

A [stage](stage.md) identifier where this battle is to place.

#### `formation`

Describes the actors that appear in formation when this battle begins.
An array of tables with the following fields:

- `actor` - An [actor](actor.md) identifier.
- `home` - The home position of the actor; either an integer or a `{ x, y, z }` vector.
