# neols

A remake of `ls` for the 21st century implemented in rust.

## What is it?

A remake of `ls` which currently serves as a learning project
with the goal of practicing the art of the rust language.

## What is it not?

`neols` is not a complete copy of GNU `ls` nor `exa`.
It aimes to be minimal with a small set of features for the everyday desktop user.
Neither `ls` nor `exa` are bad, but there are so many options that are seldomly used.

## Installation

It's easy! Just use `cargo`:

```sh
cargo install neols
```

Also, remember to include `~/.cargo/bin` in your `$PATH`.

### Optional

It is not very convinient to type `neols` all the time,
so it is therefore recommended to create an alias in your shell profile
(`.bashrc`, `.zshrc`, depending on what shell you use).

Example in `bash` (in `~/.bashrc`):

```bash
alias ls="neols -a"
```

## Features

`neols` can list the contents of a directory when given a path (the default path is `.`).
The following table gives an overview and a description of the optional arguments.

|Name|Argument|Description|
|---|---|---|
|All files|`-a`| Lists all files in the directory. This includes hidden files. This option is incompatible with `-d`|
|Long format|`-l`| Lists all files in the directory with size (in Bytes) and whether the files is ReadOnly (for the user invoking `neols`). This option is incompatible with `-d`|
|Directories only|`-d`| Lists only files that are also directories. This options is incompatible with `-a` `-l`|
