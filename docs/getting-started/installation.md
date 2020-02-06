# Installation

The first thing we need to do is install starpkg.

starpkg is _command-line application_, meaning that it runs in a terminal. If you're not used to
using the terminal, it can be a little confusing, but stick with it! It'll make you a more
productive modder.

> **Command Line Notation**
>
> Throughout this user guide, we'll show some commands used in the terminal. Lines that you should
> enter in a terminal all start with `$`. You don't need to type in the `$` character; it indicates
> the start of a command. Lines without a `$` are typically the output of the previous command.
>
> On Windows, we recommend you use PowerShell rather than Command Prompt.

### Installing on Linux, macOS, or WSL

If you're using Linux, macOS, or [Windows Subsystem for Linux][wsl], execute the following command
in a terminal:

```sh
$ curl -sSLf "https://git.io/JvZKc" | sh
```

[wsl]: https://docs.microsoft.com/en-us/windows/wsl/install-win10

### Installing on Windows

If you're using Windows, open PowerShell and execute the following command:

```sh
$ iex ((New-Object System.Net.WebClient).DownloadString('https://git.io/JvZKl'))
```

### Building from source

If the above methods don't work on your OS for whatever reason, you can compile starpkg yourself:

1. Install Rust with [rustup](https://rustup.rs)
2. `$ cargo install starpkg --force`

### Updating and uninstalling

The commands above can also be used to update starpkg to the latest version. You can check the
version you have installed with `starpkg --version`:

```sh
$ starpkg --version
starpkg 0.2.0
```

To uninstall starpkg, delete the `~/.starpkg` directory (on Windows, this is
`C:\Users\you\.starpkg`).

### Help

If you encounter any issues during installation, the best place to get help is the
[#starpkg channel of the _Paper Mario 64 Modding_ Discord server](https://discord.gg/xzq6egG).

