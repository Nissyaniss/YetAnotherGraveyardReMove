# yagrm (Yet Another Graveyard ReMove)
`yagrm` is a command line deletion tool that does not delete files but put them in a trash folder defined in a configuration file.
The idea was stolen from [rip](https://github.com/nivekuil/rip) (so this README is heavily inspired from them).

Deleted files are send by default to `$HOME/.local/share/Yagrm/trash`, you can change it via commands (see [Usage](https://github.com/Nissyaniss/YetAnotherGraveyardReMove#usage))

**`yagrm` is only available for Linux (not extendly tested for it) at the moment. No MacOS nor Windows version is planned at the moment (if you want to add it create a PR i will be happy to merge)**

# YOU SHOULD NOT ALIAS `rm` TO `yagrm`, BECAUSE IF YOU ARE ON A SYSTEM WITHOUT `yagrm` YOU MIGHT DELETE IMPORTANT THINGS AND NEVER RECOVER THEM. `yagrm`  

## THIS IS NOT A ***REAL*** PROJECT, that's my first projet that i did when i started rust, i come back to it from time to time with refacto to do because i was dumb.

## Installation

### Linux

Get the binary from [release](https://github.com/Nissyaniss/YetAnotherGraveyardReMove/releases) and move it to your `/usr/local/bin` (or any folder in your `PATH`)

## Usage

```text
Usage: yagrm [OPTIONS] [PATHS]...

Arguments:
  [PATHS]...  Files to remove

Options:
  -t, --trash-path                    Reveal the trash path
  -d, --delete-trash-contents         Deletes the trash's contents
      --set-trash-path <path>        Files to remove
      --generate-completions <SHELL>  Generate shell completions [possible values: bash, elvish, fish, powershell, zsh]
  -f, --force                         Force remove file(s) without moving to trash
  -h, --help                          Print help
```

`-f` is to remove files like rm.

`yagrm` doesn't care if what you are removing is a directory but will ask if this is wanted (even when `-f` is used), if this is not what you want it to behave like you can create an issue or a PR.

```bash
$ yagrm test_directory
The file you are trying to trash or remove is a directory. Are you sure ? [y/N]
```

```bash
$ yagrm -f test_directory
The file you are trying to trash or remove is a directory. Are you sure ? [y/N]
```

If the file name is already in the trash folder `yagrm` will add a number to it
```bash
$ ls /home/user/.local/Yagrm/trash/
test
$ yagrm test
$ ls /home/user/.local/Yagrm/trash/
test
test1
```

If you want to change the default trash folder you can either change it in the config at `$HOME/.config/Yagrm/config.toml` or use the `--set-trash-path` like this :

```bash
$ yagrm --set-trash-path /home/user/new_trash/
$ yagrm -t
This is the current trash directory.
/home/user/new_trash/
```

You can also generate completions for your favorite shell like this :
```bash
$ yagrm --generate-completions zsh
# For zsh, you can use bash, elvish, fish and powershell too
```

To empty your trash you can use the `-d` option like this :
```bash
$ yagrm -d
Are you sure you want to erase the trash ? [y/N]
```

## Planned things to do

- [x] Make the code great again (i think its done)
- [x] `fzf` integration
- [ ] Restore files to their original place
- [ ] More options in the config
- [ ] Maybe make it rm compatible ? (like same options + trash)
- [ ] Less dependencies

## Credit

- [clap](https://github.com/clap-rs/clap) for all options, completions ect
- [dialoguer](https://github.com/console-rs/dialoguer) for the [Y/N] questions
- [toml](https://github.com/toml-rs/toml) for all things toml related
- [skim](https://github.com/lotabout/skim) for the fzf integration
