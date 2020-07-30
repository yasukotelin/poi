# poipoi

poipoi provides to get your project directories command-line tool.

This tool is inspired ghq. Only access function to the project directory!!

> v1 called this tool "poi". v2 recreated by Rust and rename to "poipoi".

## Description

poipoi standard output the projects(git project) from specified directory path by poipoi.yml.

poipoi selected the project with fuzzy finder from specified directory path by `~/.config/poipoi/poipoi.yml`.

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

## Usage

`poipoi` command standard output the projects simply.

## With fzf(peco)

If poipoi is used with fuzzy-finder command, it to be very very useful.

Jump to project directory selected with fzf!

```
cd "$(poipoi | fzf)"
```

If you create a alias and use it, so fast access!!

```.bashrc
alias poipoi='cd "$(poi | fzf)"'
```

and you can use `poipoi` command.

![poipoi](./images/poipoi.gif)

## Author

yasukotelin

## Licence

MIT Licence
