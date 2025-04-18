<!--
SPDX-FileCopyrightText: Copyright © 2025 hashcatHitman

SPDX-License-Identifier: Apache-2.0 OR MIT
-->

# Hack VM Translator

[![unsafe forbidden]][safety dance] [![dependency badge]][deps.rs] [![CI status]][CI workflow] [![CodeQL]][CodeQL workflow]

---

A translator that translates programs written in the Hack VM language into Hack
assembly instructions. Based on the nand2tetris course, and written in Rust.

## Getting Started

You'll need to install Rust and its package manager, Cargo, on your system.
Please refer to the official [recommended Rust installation method] for your
system.

You should also have some version of git installed. You can refer to the
[Git documentation] if you need help with that.

Clone the repository and navigate inside it:

```bash
git clone https://github.com/hashcatHitman/hack-vm-translator.git
cd hack-vm-translator
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
be located at `./target/release/hack-vm-translator` or
`./target/release/hack-vm-translator.exe`, depending on your system.

Though relative pathing seems to work fine, for the best experience it is
recommended to keep your `*.vm` files and the translator in the same directory.
If you are doing so and are in the directory yourself, you can translate a
`*.vm` file to the equivalent `*.asm` file like so:

```bash
./hack-vm-translator Foo.vm
```

Where `Foo` can be variable, but should remain valid Unicode.

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

[dependency badge]: https://deps.rs/repo/github/hashcatHitman/hack-vm-translator/status.svg
[deps.rs]: https://deps.rs/repo/github/hashcatHitman/hack-vm-translator

[CI status]: https://github.com/hashcatHitman/hack-vm-translator/actions/workflows/ci.yml/badge.svg
[CI workflow]: https://github.com/hashcatHitman/hack-vm-translator/actions/workflows/ci.yml

[CodeQL]: https://github.com/hashcatHitman/hack-vm-translator/actions/workflows/github-code-scanning/codeql/badge.svg
[CodeQL workflow]: https://github.com/hashcatHitman/hack-vm-translator/actions/workflows/github-code-scanning/codeql

[recommended Rust installation method]: https://www.rust-lang.org/tools/install

[Git documentation]: https://git-scm.com/book/en/v2/Getting-Started-Installing-Git
