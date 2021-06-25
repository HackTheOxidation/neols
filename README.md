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

It is easy! Just use `cargo`:

```sh
cargo install neols
```

Also, remember to include `~/.cargo/bin` to your `$PATH`.

### Optional

It is not very convinent to type `neols` all the time,
so it is therefore recommended to create an alias in your shell profile
(`.bashrc`, `.zshrc`, depending on what shell you use).

Example in `bash` (in `~/.bashrc`):

```bash
alias ls="neols -a"
```
