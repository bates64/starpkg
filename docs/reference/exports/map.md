# Map

`src/map/<name>` directories.

### `<name>.xml`

The map data itself, generated/edited via Star Rod. In the future, a command will be made available
to easily edit maps with a GUI.

The texture set and background image this file uses will be replicated ingame.

### `<name>.mscr`

A [script](../scripts.md) defining a new Script_Main struct named `$Script_Main`.

Map scripts may optionally define a string called `$Tattle`, which will be used as the map's tattle.

The Header struct will be automatically generated - do not provide one. Additionally, it is
forbidden for a map to define a Script_Init struct at the moment.
