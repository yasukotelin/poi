# poipoi

<a href="https://crates.io/crates/poipoi"><img src="https://img.shields.io/crates/v/poipoi"/></a>
<a href="https://crates.io/crates/poipoi"><img src="https://img.shields.io/crates/d/poipoi"/></a>
![](https://img.shields.io/github/license/yasukotelin/poi)

poipoi provides to get your project directories command-line tool.

This tool is inspired ghq. Only access function to the project directory!!

> v1 called this tool "poi". v2 recreated by Rust and rename to "poipoi".

## Description

poipoi is selector with fuzzy finder the git projects from specified directory path by poipoi.yml.

The selected path will be standard output.

```poipoi.yml
# projects - you can specify the directories had projects managed by git.
projects:
  - ~/AndroidStudioProjects
  - ~/IdeaProjects
  - ~/source
  - ~/go/src
```

## Installation

```
cargo install poipoi
```

Second, you shoud be put a `~/.config/poipoi/poipoi.yml` setting file.

```poipoi.yml
# projects - you can specify the directories had projects managed by git.
projects:
  - ~/AndroidStudioProjects
  - ~/IdeaProjects
  - ~/source
  - ~/go/src

# others - you can specify the non project directories that are printed as is.
others:
  - ~/.config/nvim
  - ~/.config/poi
```

`projects:` is target the root directory path to search projects.
poipoi will find the git project. And standard output them.

`others:` is target the non project directories that are printed as is.
If you want to include non project directory in your result, specify this.

## Update

```
cargo install -f poipoi
```

## Usage

Just only use `poipoi` command!

### Move project directory

Use poipoi with cd command.

```
> cd $(poipoi)
```

If you use options.

```
> cd $(poipoi --color light)
```

and you define alias, so useful.

```.bashrc
alias poi='cd $(poipoi)'
```

## Options

### Fuzzy finder color

poipoi using fuzzy finder library is [skim](https://github.com/lotabout/skim).

You can use `poipoi --color [skim color option]` and poipoi calls skim with --color option.

Example, if use light background terminal.

```
poipoi --color light
```

and more custom color.

```
poipoi --color=light,fg:232,bg:255,current_bg:116,info:27
```

If you want to more infomation, see [skim readme page](https://github.com/lotabout/skim).

## Height

Change fuzzy finder height. Default height is 30%.

```
poipoi --height 100%
```

## No fuzzy find

If you don't use fuzzy find. `--noskim` option is only output project paths.

```
> poipoi --noskim
```

## With fzf or peco

If you want to use fzf or peco.

```
> poipoi --noskim | fzf
```

## Author

yasukotelin

## Licence

MIT Licence
