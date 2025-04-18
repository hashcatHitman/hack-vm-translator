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
#![feature(iterator_try_collect)]

use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::error::HackError;
use crate::parser::Parser;
use crate::translator::Translator;

pub mod error;
pub mod parser;
pub mod translator;

/// The basic configuration of the binary, storing the results from a successful
/// command-line invocation.
#[derive(Debug, Hash)]
pub struct Config {
    /// The path to the target Hack `.vm` file.
    file_path: PathBuf,
}

impl Config {
    /// Attempts to build a valid [`Config`] from the arguments passed on the
    /// command line.
    ///
    /// A valid [`Config`] consists of just a single argument passed - the path
    /// to a Hack VM file or a directory containing several.
    ///
    /// Example:
    /// ```bash
    /// hack-vm-translator ./foo.vm
    /// ```
    /// # Errors
    ///
    /// There are two conditions under which this will return an error:
    ///
    /// - No arguments were passed.
    ///
    /// - More than one argument was passed.
    ///
    /// In either scenario, the error received will be a
    /// [`HackError::Misconfiguration`] holding the number of arguments that
    /// were passed, up to a limit of [`usize::MAX`].
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Self, HackError> {
        let _ = args.next();

        let file_path: PathBuf = match args.next() {
            Some(file_path) => PathBuf::from(file_path),
            None => return Err(HackError::Misconfiguration(0)),
        };

        if args.next().is_some() {
            if let Some(count) = args.count().checked_add(2) {
                return Err(HackError::Misconfiguration(count));
            }
            return Err(HackError::Misconfiguration(usize::MAX));
        }

        Ok(Self { file_path })
    }

    /// Gets a shared reference to [`Config::file_path`].
    ///
    /// This is the path to the target Hack `.asm` file, as a borrowed
    /// [`PathBuf`].
    pub(crate) const fn file_path(&self) -> &PathBuf {
        &self.file_path
    }
}

/// Attempts to translate a single given file.
///
/// Given a borrowed [`Path`], attempts to read the file it corresponds to,
/// creates a new file with the same name/location but using the `*.asm`
/// extension, and translates each line to Hack assembly instructions before
/// writing to the new file.
///
/// # Errors
///
/// The majority of errors can that occur will be propagated here - some may be
/// internal. See [`crate::error`] for more information of the errors.
fn run_for_file(file: &Path) -> Result<(), HackError> {
    let parser: Parser = Parser::try_from(file.as_os_str())?;
    let instructions: std::iter::Enumerate<
        std::vec::IntoIter<parser::Instruction>,
    > = parser.parse()?;
    let new_file: PathBuf = if file.extension().is_some_and(|ext| ext == "vm") {
        file.with_extension("asm")
    } else {
        return Err(HackError::BadFileTypeError);
    };
    let file_name: &OsStr = file.file_stem().ok_or(HackError::Internal)?;
    let mut new_file: File = File::create(new_file)?;

    for (line_number, instruction) in instructions {
        let assembly: String = Translator::translate(
            line_number,
            &instruction,
            file_name.to_str().ok_or(HackError::Internal)?,
        )?
        .join("\n");
        let mut assembly = assembly;
        assembly.push('\n');
        let assembly = assembly;
        new_file.write_all(assembly.as_bytes())?;
        new_file.write_all(b"\n")?;
    }
    Ok(())
}

/// Given a borrow of a valid [`Config`], runs the main program logic.
///
/// If the [`Config`] is targeting a valid Hack VM file, it will be read into
/// memory and have each line deserialized into an
/// [`crate::parser::Instruction`].
///
/// If the input file was `foo.vm`, the program will try to write the output to
/// `foo.asm`. If  the file exists, it will be overwritten.
///
/// # Errors
///
/// Any non-[`Config`] error that can happen is eventually propagated here. See
/// the [`crate::error`] module for more details.
pub fn run(config: &Config) -> Result<(), HackError> {
    let path: PathBuf = config.file_path().canonicalize()?;
    if path.try_exists()? {
        if path.is_dir() {
            let files: Result<std::fs::ReadDir, std::io::Error> =
                path.read_dir();
            let files: std::fs::ReadDir = files?;
            for entry in files {
                let file: PathBuf = entry?.path().canonicalize()?;
                run_for_file(&file)?;
            }
            Ok(())
        } else if path.is_file() {
            run_for_file(&path)
        } else {
            Err(HackError::CannotReadFileFromPath(
                "path does not point to a file or directory".to_owned(),
            ))?
        }
    } else {
        Err(HackError::CannotReadFileFromPath(
            "path does not point to a file or directory".to_owned(),
        ))?
    }
}
