# ignorers

[![License](https://img.shields.io/github/license/FollowTheProcess/ignorers)](https://github.com/FollowTheProcess/ignorers)
[![GitHub](https://img.shields.io/github/v/release/FollowTheProcess/ignorers?logo=github&sort=semver)](https://github.com/FollowTheProcess/ignorers)
[![CI](https://github.com/FollowTheProcess/ignorers/workflows/CI/badge.svg)](https://github.com/FollowTheProcess/ignorers/actions?query=workflow%3ACI)
[![Unsafe Forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance)

Generate gitignores from the command line 🛠️

* Free software: Apache Software License 2.0

## Project Description

Extremely simple (but very useful) CLI that generates `.gitignore` files from the command line (sort of 😉). It's a sister project to [goignore], replicated here in
rust 🦀.

### Why?

[goignore] was the first real program I wrote while learning Go, so I thought why not do it in Rust too? Like [goignore] it's mostly a thin wrapper around the
[gitignore API]. The list of things you can pass are documented on [gitignore.io].

## Installation

There are pre-compiled binaries available in the GitHub releases, or there is a homebrew tap available:

```shell
brew install FollowTheProcess/homebrew-tap/ignorers
```

## Quickstart

The binary itself is called `ig` (for `ignorers`), so you can run it like so:

```shell
ig macos python visualstudiocode
```

### Credits

This package was created with [copier] and the [FollowTheProcess/rust_copier] project template.

[copier]: https://copier.readthedocs.io/en/stable/
[FollowTheProcess/rust_copier]: https://github.com/FollowTheProcess/rust_copier
[goignore]: https://github.com/FollowTheProcess/goignore
[gitignore API]: https://www.toptal.com/developers/gitignore
[gitignore.io]: https://www.toptal.com/developers/gitignore
