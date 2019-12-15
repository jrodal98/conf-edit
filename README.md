# Conf-edit

An easy way to quickly edit your configuration files.

## About

I've gotten pretty deep into customizing my arch linux laptop.  This means having to regularly edit my i3 config, my zsh config, my polybar config, my st terminal config, and so on.  It got annoying having to type something like `nano ~/.config/i3/config` or `code ~/software/st/config.h` everytime I want to edit one of these files.  You could obviously add aliases for these, but that could get messy quickly.  Plus, you'd have to remember all of the aliases.

Introducing `conf-edit`.  Alias this once to something like `alias ce='~/Projects/conf-edit/conf-edit'` and start ricing in style.

## Usage

Commands:

- `add`: add a new entry
- `remove`: remove an existing entry
- `ls`: list all entries
- `execute`: execute the posthook script for a given entry
- `<entry name>`: edit an entry based on its name

Examples:

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