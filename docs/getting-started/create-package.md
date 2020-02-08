# Creating a package

Lets make a basic mod! In starpkg, mods are called _packages_.

## Making a new package directory with `starpkg new`

To create a new package, run `starpkg new <name of package>` in an empty directory:

```terminal
$ mkdir first_package
$ cd first_package

$ starpkg new first_package
info: created package first_package v0.1.0 at ~/first_package
```

(`starpkg new` will automatically create a directory for the package if the current one is not
empty.)

> **What do `mkdir` and `cd` do?**
>
> These commands create a new directory, and open one, respectively. If you want to learn more,
> [see this table][command-table]. Most of the commands listed are also supported by PowerShell.
>
> [command-table]: https://www.dummies.com/computers/operating-systems/linux/common-linux-commands/

By default, `starpkg new` creates a `starpkg.toml` file and an empty `src` directory. It also
creates a `.gitignore` file, but we don't need to worry about it for now.

### starpkg.toml

Let's look at `starpkg.toml` first. Open it in your favourite text editor and you'll see something
like:

```toml
name = "first_package"
version = "0.1.0"

[dependencies]
pm64 = "*"
```

This file, also known as the _package manifest_, declares metadata about a package, like its name,
version number, and its dependencies. We'll talk more about dependencies and version numbers later.

> **What's a TOML file?**
>
> TOML stands for ["Tom's Obvious, Minimal Language"][toml]. starpkg uses it for most of its
> configuration files. If you're using Visual Studio Code, you can get syntax highlighting for TOML
> files by installing [this extension][vsext].
>
> [toml]: https://github.com/toml-lang/toml/blob/master/versions/en/toml-v0.5.0.md
> [vsext]: https://marketplace.visualstudio.com/items?itemName=be5invis.toml

### src

The `src` (short for "source") directory starts off empty, but it is the most important part of your
package. It holds all the new enemies, badges, maps, etc your package adds to the game. As a group,
these are referred to as _exports_.

## Compiling into a rom with `starpkg build`

The `starpkg build` command assembles the package and its dependencies into a Star Rod 'mod
folder.' In the future, this command will also have Star Rod compile the mod folder into a working
modded _Paper Mario_ rom.

```terminal
$ starpkg build
info: assembled first_package v0.1.0 in 0.00458s
```

If you're interested, you can view the mod folder that starpkg builds by viewing the created
`.build` directory. However, do not make any changes there as subsequent `starpkg build`s will
overwrite your changes!
