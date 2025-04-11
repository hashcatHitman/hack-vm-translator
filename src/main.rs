// SPDX-FileCopyrightText: Copyright © 2025 hashcatHitman
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Hack VM Translator
//!
//! A VM translator that parses Hack VM commands and generates Hack assembly.
//! Based on the nand2tetris course.

/// The entrypoint of the assembler executable.
///
/// ```rust
/// println!("hello")
/// ```
fn main() {
    println!("{}", hack_vm_translator::read_a_book());
}
