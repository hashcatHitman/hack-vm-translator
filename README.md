<!--
SPDX-FileCopyrightText: Copyright © 2025 hashcatHitman

SPDX-License-Identifier: Apache-2.0 OR MIT
-->

# Rust Project Template

[![unsafe forbidden]][safety dance] [![dependency badge]][deps.rs] [![CI status]][CI workflow] [![CodeQL]][CodeQL workflow]

---

This project serves as a base for my projects in Rust. It isn't really meant to
be used by others, but you're welcome to use it if you find it helpful. If you
decide to use it, it IS licensed under the dual Apache 2.0/MIT licensing.

The `CONTRIBUTING.md` is also not exactly meant for this project, but if you do
want to contribute, that's more than welcome - just bear in mind I'm unlikely to
accept contributions that don't make the project more useful for me.

The rest of the information in this README is boilerplate.

## Getting Started

You'll need to install Rust and its package manager, Cargo, on your system.
Please refer to the official [recommended Rust installation method] for your
system.

You should also have some version of git installed. You can refer to the
[Git documentation] if you need help with that.

Clone the repository and navigate inside it:

```bash
git clone https://github.com/hashcatHitman/rust_project_template.git
cd rust_project_template
```

If you'd like to read the documentation, the recommended way to do so is with:

```bash
cargo doc --document-private-items --open
```

Which will open the documentation in your browser.

To build the project, you can do:

```bash
cargo build --profile release --locked
```

Cargo will download the dependencies and compile the project. It will probably
be located at `./target/release/rust_project_template` or
`./target/release/rust_project_template.exe`, depending on your system.

## MSRV Policy

<!-- Adapted from Arti's MSRV policy -->

Our current Minimum Supported Rust Version (MSRV) is 1.89.

We may increase the patch level of the MSRV on any release.

Otherwise, we will not increase MSRV on PATCH releases, though our dependencies
might.

We won't increase MSRV just because we can: we'll only do so when we have a
reason. (We don't guarantee that you'll agree with our reasoning; only that
it will exist.)

[unsafe forbidden]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety dance]: https://github.com/rust-secure-code/safety-dance/

[dependency badge]: https://deps.rs/repo/github/hashcatHitman/rust_project_template/status.svg
[deps.rs]: https://deps.rs/repo/github/hashcatHitman/rust_project_template

[CI status]: https://github.com/hashcatHitman/rust_project_template/actions/workflows/ci.yml/badge.svg
[CI workflow]: https://github.com/hashcatHitman/rust_project_template/actions/workflows/ci.yml

[CodeQL]: https://github.com/hashcatHitman/rust_project_template/actions/workflows/github-code-scanning/codeql/badge.svg
[CodeQL workflow]: https://github.com/hashcatHitman/rust_project_template/actions/workflows/github-code-scanning/codeql

[recommended Rust installation method]: https://www.rust-lang.org/tools/install

[Git documentation]: https://git-scm.com/book/en/v2/Getting-Started-Installing-Git
