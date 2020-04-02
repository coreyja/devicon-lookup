# devicon-lookup

[![CircleCI](https://circleci.com/gh/coreyja/devicon-lookup.svg?style=svg)](https://circleci.com/gh/coreyja/devicon-lookup) [![Join the chat at https://gitter.im/devicon-lookup/community](https://badges.gitter.im/devicon-lookup/community.svg)](https://gitter.im/devicon-lookup/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

`devicon-lookup` is a simple standalone CLI tool.
It accepts text over stdin, and returns it stdout with a devicon prepended.

## Use Cases

Uses cases include:

### Prepending devicons to the files in your directory

```
ls | devicon-lookup --color
```
![`ls | devicon-lookup --color`](/docs/screenshots/ls.png?raw=true)

### Adding icons to grep results

```
rg test | devicon-lookup --prefix :
```
![`rg test | devicon-lookup --prefix :`](/docs/screenshots/grep.png?raw=true)

### Streaming results for large result sets

```
rg str --color always | devicon-lookup -c -p : | fzf --ansi
```
![`rg str -uuu --color always | devicon-lookup -c -p : | fzf --ansi`](/docs/screenshots/grep+fzf.gif?raw=true)


## Installation

The recommended way to install is via `cargo` the Rust package manager

```
cargo install devicon-lookup
```

### Upgrading

Upgrading can also be done via `cargo`. The following command will install the latest available version on devicon-lookup

```
cargo install devicon-lookup --force
```

## VIM Usage

The primary real world usage of this tool is within `VIM` and specifically with `fzf`.
For more information about one possible solution to integrating these see the `fzf.devicon.vim` repo.
This repo is a fork of `fzf.vim` that uses this tool to add devicons to the fuzzy search results

(fzf 💜 devicon ❤️ vim)[coreyja/fzf.devicon.vim]

## CLI Usage

```
 Dev Icon Lookup

 Usage:
   devicon-lookup [options]
   devicon-lookup (-h | --help)
   devicon-lookup --version

 Options:
   -h --help                      Show this screen.

   --version                      Show version.

   -c --color
   Strip ANSI color codes from each line before processing them
   The original string with ANSI codes preserved is output

   -p --prefix=<delimiter>
   The filename is considered to be everything up to the given
   delimineter. The entire line is still output

   -r --regex=<regex>
   Regex used on each line to find the filename.
   Regex's must be valid based on the Rust `regex` crate
   The regex is expected to have a single capture group,
   which contains the filename. Extra capture groups are ignored
```

## Further Reading

Blog post about the initial build and inspiration: https://coreyja.com/vim-fzf-with-devicons/
