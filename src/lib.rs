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

/// Foo bar
pub const fn foo(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result: u64 = foo(2, 2);
        assert_eq!(result, 4);
    }
}
