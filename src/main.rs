//! # Hack VM Translator
//!
//! <details>
//!     <summary>Licensing Info</summary>
//!
//! > hack-vm-translator - A VM translator that parses Hack VM commands and
//! > generates Hack assembly.
//! > Copyright (C) 2025  [hashcatHitman]
//! >
//! > This program is free software: you can redistribute it and/or modify
//! > it under the terms of the GNU Affero General Public License as published
//! > by the Free Software Foundation, either version 3 of the License, or
//! > (at your option) any later version.
//! >
//! > This program is distributed in the hope that it will be useful,
//! > but WITHOUT ANY WARRANTY; without even the implied warranty of
//! > MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! > GNU Affero General Public License for more details.
//! >
//! > You should have received a copy of the GNU Affero General Public License
//! > along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!
//! [hashcatHitman]: https://github.com/hashcatHitman
//!
//! </details>
//!
//! A VM translator that parses Hack VM commands and generates Hack assembly.
//! Based on the nand2tetris course.

#![feature(strict_provenance_lints, unqualified_local_imports)]

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
