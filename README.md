# Conf-edit

An easy way to quickly edit your configuration files.

* Requirements:
    * Rust (until I provide a binary, anyway)
    * unix system (maybe not actually true, but windows is untested)

## Install

Currently, the only way to install conf-edit is to install by source (hence why Rust is required).

The easiest way to install the program is to run `cargo install --git https://github.com/jrodal98/conf-edit.git`. Alternatively, you can also do this:

```bash
git clone https://github.com/jrodal98/conf-edit.git
cargo install --path conf-edit
```


## About

`conf-edit` provides an easy, intuitive solution for quickly navigating to configuration files. It essentially stores paths to configuration files and some associated metadata, such as the editor you wish to use and any scripts that you want to execute upon closing the file. It works similarly to aliasing, but is more organized and prevents you from having to memorize different aliases for each file.

## Usage

Better documentation to come shortly.

Commands:

- `ls`: list all entries
- `add`: add a new entry
- `rm`: remove an existing entry
- `edit`: edit an entry based on its name
- `execute`: execute the posthook script for a given entry

Examples:

### Adding:

```
USAGE:
    ce add [OPTIONS] <path> <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --editor <editor>    
    -s, --script <script>    

ARGS:
    <path>    
    <name>
```    

```bash
ce add ~/.config/i3/config i3 -s 'i3-msg restart'
ce add ~/software/st/config.h st -e code
ce add ~/.zshrc zsh # 'source ~/.zshrc' doesn't work (not a bug)
```

### listing:

```
USAGE:
    ce ls

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

```bash
ce ls

Name            Path
conf-edit       /home/jake/.config/conf-edit/config
i3              /home/jake/.config/i3/config
st              /home/jake/software/st/config.h
zsh             /home/jake/.zshrc
```

### Remove:

```
USAGE:
    ce rm <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <name>    
```

```bash
ce rm st
```


### Executing:

```
USAGE:
    ce execute <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <name>    
```

```bash
ce execute i3
```

The above example runs the `i3-msg restart` command.

### File editing:

```
USAGE:
    ce edit [FLAGS] <name>

FLAGS:
    -h, --help       Prints help information
    -n, --no-exec    
    -V, --version    Prints version information

ARGS:
    <name>    
```

```bash
ce edit i3
```

This would open my i3 config and then run `i3-msg restart` when I'm done editing.

```bash
ce i3 -n
```

This would edit the `i3` entry but not run the posthook.
