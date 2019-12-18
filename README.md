# Conf-edit

An easy way to quickly edit your configuration files.

## About

`conf-edit` provides an easy, intuitive solution for quickly navigating to configuration files. It essentially stores paths to configuration files and some associated metadata, such as the editor you wish to use and any scripts that you want to execute upon closing the file. It works similarly to aliasing, but is more organized and prevents you from having to memorize different aliases for each file.

Alias `conf-edit` once to something like `alias ce='~/Projects/conf-edit/conf-edit'` and start ricing in style.

## Usage

Commands:

- `set-editor`: set the default editor for all entries
- `add`: add a new entry
- `remove`: remove an existing entry
- `ls`: list all entries
- `execute`: execute the posthook script for a given entry
- `replace`: modify an entry
- `<entry name>`: edit an entry based on its name

Examples:

Setting editor:

```
ce set-editor vim
ce set-editor /usr/bin/nano
```

Adding:

```bash
alias ce='./conf-edit'
ce add ~/.config/i3/config -n i3 -s 'i3-msg restart'
ce add ~/software/st/config.h -n st -e code
ce add ~/.zshrc # 'source ~/.zshrc' posthook not added because child processes can't modify parent shell
```

listing:

```bash
ce ls

Name            Path
conf-edit       /home/jake/.config/conf-edit/config
i3              /home/jake/.config/i3/config
st              /home/jake/software/st/config.h
.zshrc          /home/jake/.zshrc
```

Remove:

```bash
ce remove .zshrc
ce remove st

ce ls

Name            Path
conf-edit       /home/jake/.config/conf-edit/config
i3              /home/jake/.config/i3/config
```

Replace:

```bash
ce ls

Name            Path
conf-edit       /home/jake/.config/conf-edit/config
i3              /home/jake/.config/i3/config

ce replace conf-edit -n ce -s "echo 'I renamed the conf-edit entry to ce and added this hook'"

ce ls

Name            Path
ce       /home/jake/.config/conf-edit/config
i3              /home/jake/.config/i3/config

```

Executing:

```bash
ce execute i3
```

The above example runs the `i3-msg restart` command.

File editing:

```bash
ce i3
```

This would open my i3 config and then run `i3-msg restart` when I'm done editing.

```bash
ce i3 -v
```

This would open my i3 config in "view" mode, which makes it so that the posthook script doesn't run (e.g. `i3-msg restart` doesn't run).

## Help

Running `ce -h` provides a bit of help, as does `ce <action> -h` (e.g. `ce add -h`)
