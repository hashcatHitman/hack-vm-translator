//! # Hack Errors
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
//! A submodule containing the various [`HackError`]s that can occur.

use std::fmt::Display;
use std::io::Error;

use crate::parser::Constant;

/// An enum containing all [`HackError`]s.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HackError {
    /// A [`HackError`] returned when failing to read the file provided. The
    /// [`String`] within is meant to hold some arbitrary, message: typically,
    /// this will be the string representation of the original error,
    /// potentially with added context.
    CannotReadFileFromPath(String),
    /// A [`HackError`] returned when a label or address uses characters not
    /// permitted in valid symbols. Symbols must be a sequences of letters
    /// (a-z || A-Z), digits (0-9), underscores (_), dots (.), dollar signs ($),
    /// and/or colons (:) that do not begin with a digit.
    SymbolHasForbiddenCharacter,
    /// A [`HackError`] returned whenever we get an instruction we honestly
    /// aren't sure what to do with, which is contained in its [`String`].
    UnrecognizedInstruction(String),
    /// A [`HackError`] returned if the number of arguments received was
    /// unexpected. Contains the number of arguments received as a [`usize`], up
    /// to [`usize::MAX`]. Anything above will simply be represented as
    /// [`usize::MAX`].
    Misconfiguration(usize),
    /// A [`HackError`] returned if we aren't able to write to the output file,
    /// either because it doesn't exist or something else.
    FileExistsError {
        /// True if we're certain it does not exist.
        certain: bool,
    },
    /// A [`HackError`] returned if the target Hack ASM file doesn't end in the
    /// extension `.asm`.
    BadFileTypeError,
    /// A [`HackError`] returned if any errors are thrown when trying to write
    /// the output. The [`String`] within is meant to hold some arbitrary,
    /// message: typically, this will be the string representation of the
    /// original error, potentially with added context.
    WriteError(String),
    /// A [`HackError`] returned if any errors are thrown due to some internal
    /// misuse or logic error. Report this!
    Internal,
    /// A [`HackError`] returned if any errors are thrown while trying to create
    /// internal data structures from a borrowed [`str`] slice. The [`String`]
    /// it holds should contain additional information.
    FromStrError(String),
    /// A [`HackError`] returned if an attempt to call
    /// [`Constant::try_from<u16>`] uses a [`u16`] which exceeds
    /// [`Constant::MAX_VALID_CONSTANT`].
    Overflow,
    /// A [`HackError`] returned if a [`crate::parser::Instruction`] has been
    /// determined to be illegal, such as by accessing an index it is not
    /// permitted to.
    IllegalInstruction(String),
}

impl From<Error> for HackError {
    /// Creates a [`HackError::CannotReadFileFromPath`] from the [`Error`]
    /// returned by failed file reading operations.
    fn from(value: Error) -> Self {
        Self::CannotReadFileFromPath(value.to_string())
    }
}

impl Display for HackError {
    /// Determines the error message for displaying [`HackError`]s.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message: &str = match self {
            Self::SymbolHasForbiddenCharacter => {
                "symbols must be a sequence of letters (a-z || A-Z), digits \
                (0-9), underscores (_), dots (.), dollar signs ($), and/or \
                colons (:) that does not begin with a digit"
            }
            Self::UnrecognizedInstruction(bad_instruction) => {
                return write!(
                    f,
                    "could not determine instruction type for \
                    \"{bad_instruction}\""
                );
            }
            Self::Misconfiguration(args) => {
                return write!(
                    f,
                    "expected 1 argument (file.asm), found {args} arguments",
                );
            }
            Self::FileExistsError { certain } => {
                if *certain {
                    "the target output file already exists, and this program \
                    refuses to overwrite it"
                } else {
                    "there is uncertainty about whether or not it is safe to \
                    create a new file of the target name"
                }
            }
            Self::BadFileTypeError => {
                "the target file must have the \".asm\" extension"
            }
            Self::Overflow => {
                return write!(
                    f,
                    "constants much be non-negative integers which are less \
                    than or equal to {}",
                    Constant::MAX_VALID_CONSTANT
                );
            }
            Self::IllegalInstruction(error_message)
            | Self::FromStrError(error_message)
            | Self::WriteError(error_message)
            | Self::CannotReadFileFromPath(error_message) => error_message,
            Self::Internal => "internal error, please report this incident",
        };

        write!(f, "{message}")
    }
}
