# pullr

Based on the [update batteries][1] script by @xJonathanLEI.

## Install

Using [cargo][2]:

```bash
cargo install pullr
```

## Usage

Merge a single pull request:

```bash
pullr --dry-run 123
```

## Release

Run `cargo release hook` to inspect the changelog generated with [git cliff][3].

Use [`cargo release`][4] to create a new tag and publish the release.

[1]: https://github.com/xJonathanLEI/helix/blob/script/update_batteries.sh
[2]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[3]: https://github.com/orhun/git-cliff
[4]: https://github.com/crate-ci/cargo-release
