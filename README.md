dircnt
======

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/dircnt-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/dircnt)
[<img alt="crates.io" src="https://img.shields.io/crates/v/dircnt.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/dircnt)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/dtolnay/dircnt/CI/master?style=for-the-badge" height="20">](https://github.com/dtolnay/dircnt/actions?query=branch%3Amaster)

Count directory entries faster than `ls -f | wc -l`.

### Installation

```console
$ cargo install dircnt
```

<br>

### Usage

```console
$ dircnt path/to/dir
63496110
```

Like `ls | wc -l` but unlike `ls -f | wc -l`, the special directory entries `.`
and `..` are excluded from the count.

<br>

### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
