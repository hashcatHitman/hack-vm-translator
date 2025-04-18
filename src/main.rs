// SPDX-FileCopyrightText: Copyright © 2025 hashcatHitman
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Hack VM Translator
//!
//! A VM translator that parses Hack VM commands and generates Hack assembly.
//! Based on the nand2tetris course.

use std::{env, process};

use hack_vm_translator::{Config, run};

/// The entrypoint of the translator executable.
pub(crate) fn main() {
    let args: env::Args = env::args();

    let config: Config = Config::build(args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    if let Err(error) = run(&config) {
        eprintln!("Problem running: {error}");
        process::exit(1);
    }
}
