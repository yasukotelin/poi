# poi

poi provides to get your project directories command-line tool.

This tool is inspired ghq. Only access function to the project directory!!

## Description

poi standard output the projects(git project) from specified directory path by poi.yml.

```poi.yml
# projects - you can specify the directories had projects managed by git.
projects:
  - ~/AndroidStudioProjects
  - ~/IdeaProjects
  - ~/source
  - ~/go/src
```

![poi](./images/poi.gif)

If you have use fuzzy-finder command (like fzf or peco), you can access the project quickly.

![poi with fzf](./images/poi-with-fzf.gif)

## Installation

```
go get github.com/yasukotelin/poi
```

Second, you shoud be put a `~/.config/poi/poi.yml` setting file.

```poi.yml
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
poi will find the git project. And standard output them.

`others:` is target the non project directories that are printed as is.
If you want to include non project directory in your result, specify this.

## Usage

`poi` command standard output the projects simply.

## With fzf(peco)

If poi is used with fuzzy-finder command, it to be very very useful.

Jump to project directory selected with fzf!

```
cd "$(poi | fzf)"
```

If you create a alias and use it, so fast access!!

```.bashrc
alias poipoi='cd "$(poi | fzf)"'
```

## Author

yasukotelin

## Licence

- MIT Licence
