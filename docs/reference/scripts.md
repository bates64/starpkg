# Scripts

Some exports, such as [actors](../exports/actor.md), require scripting. This is done in the same way
as in Star Rod (ie. `mscr`/`bscr` files), albeit with a few starpkg-specific preprocessing features.

## Reference expressions

Scripts can reference exports using `{...}` syntax. These are replaced at assembly time.
Note that Star Rod has some of its own also, but these expressions do not refer to starpkg exports.

### `{String:identifier}`
Expands to the ID of the given string in the form `00XX0YYY`, where `XX` is the string section and
`YYY` is string's index within that section.

This feature only works for named, exported strings, not `$Pointer` strings defined within scripts,
and it supercedes Star Rod's `{String:name}` syntax.

### `{Sprite:identifier}`
Expands to the numeric index of the given sprite, padded to two hex digits.

### `{Sprite:identifier:animation}`
Expands to `00II00AA`, ie. the value representing the given sprite performing the given animation.

### `{Sprite:identifier:animation:palette}`
Expands to `00IIPPAA`, ie. the value representing the given sprite performing the given animation
with the given palette.

### `{Actor:identifier}`
Expands to the numeric index of the given actor, padded to two hex digits.
