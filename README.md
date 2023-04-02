# pullr

Based on the [update batteries](https://github.com/xJonathanLEI/helix/blob/script/update_batteries.sh)
script by @xJonathanLEI.

## Install

Using [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```bash
cargo install pullr
```

## Usage

Merge a single pull request:

```bash
pullr --dry-run 123
```

Run a command on success use `master` as main branch:

```bash
pullr -c="echo OK" -dm 123
```
