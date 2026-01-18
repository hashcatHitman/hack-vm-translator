// SPDX-FileCopyrightText: Copyright © 2025 hashcatHitman
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Hack VM Translator - Parser Module
//!
//! Parses Hack VM commands. Based on the nand2tetris course.

use core::fmt::Display;
use core::iter::Enumerate;
use core::str::FromStr;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::vec::IntoIter;

use crate::error::HackError;

/// Reads the contents of a file and deserializes them.
///
/// The [`Parser`] is used to read the contents of a [`std::fs::File`] line by
/// line, parsing the text into [`Instruction`]s that can be translated into
/// assembly.
#[derive(Debug, Clone, Hash)]
pub(crate) struct Parser {
    /// The contents of the file as a String.
    file: String,
}

impl Parser {
    /// Returns a more workable form of the file contents.
    ///
    /// Returns an [`Iterator`] over the lines of a the held file contents,
    /// trimmed, filtered for comments, and split by whitespace as vectors of
    /// string slices.
    pub(crate) fn lines(&self) -> impl Iterator<Item = Vec<&str>> {
        self.file.lines().filter_map(|line: &str| {
            let line = line.trim();
            if line.starts_with("//") || line.is_empty() {
                return None;
            }
            Some(line.split_whitespace().collect())
        })
    }

    /// Deserializes the file contents into [`Instruction`]s.
    pub(crate) fn to_internal_types<'a, I>(
        iterator: I,
    ) -> Result<Enumerate<IntoIter<Instruction>>, HackError>
    where
        I: Iterator<Item = Vec<&'a str>>,
    {
        let iterator: Vec<Instruction> = iterator
            .map(|parts: Vec<&str>| match parts[..] {
                [command] => Instruction::from_str(command),
                [command, symbol] => {
                    match (command, Symbol::from_str(symbol)) {
                        (command, Ok(symbol)) => {
                            Instruction::try_from(&(command, symbol))
                        }
                        (_, Err(symbol_error)) => Err(symbol_error),
                    }
                }
                [command, symbol, constant] => match (
                    command,
                    Symbol::from_str(symbol),
                    Constant::from_str(constant),
                ) {
                    (command, Ok(symbol), Ok(constant)) => {
                        Instruction::try_from(&(command, symbol, constant))
                    }
                    (_, Err(symbol_error), Err(constant_error)) => {
                        Err(HackError::UnrecognizedInstruction(format!(
                            "{symbol_error}\n\n{constant_error}"
                        )))
                    }
                    (.., Err(error)) | (_, Err(error), _) => Err(error),
                },
                _ => Err(HackError::IllegalInstruction(
                    "received an illegal instruction".to_owned(),
                )),
            })
            .collect::<Result<Vec<Instruction>, HackError>>()?;
        Ok(iterator.into_iter().enumerate())
    }

    /// Deserializes the file contents into [`Instruction`]s, returning an
    /// iterator over tuples for each line with an associated index and the
    /// [`Instruction`] received from it.
    pub(crate) fn parse(
        &self,
    ) -> Result<Enumerate<IntoIter<Instruction>>, HackError> {
        Self::to_internal_types(self.lines())
    }
}

impl TryFrom<&OsStr> for Parser {
    type Error = HackError;

    /// Tries to read the contents of a file located at the path indicated by
    /// `value`.
    fn try_from(value: &OsStr) -> Result<Self, Self::Error> {
        let file: String = read_to_string(value)?;
        Ok(Self { file })
    }
}

/// A representation of a valid Hack VM instruction.
///
/// [`Instruction::StackManipulation`] can contain [`StackManipulation::Push`]
/// and [`StackManipulation::Pop`].
///
/// [`Instruction::Branching`] can contain [`Branching::Label`],
/// [`Branching::GoTo`], and [`Branching::IfGoTo`].
///
/// [`Instruction::Functional`] can contain [`Functional::Function`],
/// [`Functional::Call`], and [`Functional::Return`]
#[derive(Debug, Clone, Hash)]
pub(crate) enum Instruction {
    /// A discriminant for stack manipulating instructions.
    StackManipulation(StackManipulation),
    /// A discriminant for branching instructions.
    Branching(Branching),
    /// A discriminant for functional instructions.
    Functional(Functional),
    /// A discriminant for arithmetic and logical instructions.
    Arithmetic(Arithmetic),
}

impl FromStr for Instruction {
    type Err = HackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let both: (
            Result<Arithmetic, HackError>,
            Result<Functional, HackError>,
        ) = (Arithmetic::from_str(s), Functional::from_str(s));

        match both {
            (Ok(arithmetic), Err(_)) => Ok(Self::from(arithmetic)),
            (Err(_), Ok(return_command)) => Ok(Self::from(return_command)),
            (Err(_), Err(_)) => {
                Err(HackError::UnrecognizedInstruction(s.to_owned()))
            }
            (Ok(_), Ok(_)) => Err(HackError::Internal),
        }
    }
}

impl TryFrom<&(&str, Symbol)> for Instruction {
    type Error = HackError;

    fn try_from(value: &(&str, Symbol)) -> Result<Self, Self::Error> {
        match Branching::try_from(value) {
            Ok(branching) => Ok(Self::from(branching)),
            Err(error) => Err(error),
        }
    }
}

impl TryFrom<&(&str, Symbol, Constant)> for Instruction {
    type Error = HackError;

    fn try_from(value: &(&str, Symbol, Constant)) -> Result<Self, Self::Error> {
        let both: (
            Result<StackManipulation, HackError>,
            Result<Functional, HackError>,
        ) = (
            StackManipulation::try_from(value),
            Functional::try_from(value),
        );

        match both {
            (Ok(stack_manipulation), Err(_)) => {
                Ok(Self::from(stack_manipulation))
            }
            (Err(_), Ok(functional)) => Ok(Self::from(functional)),
            (Err(_), Err(_)) => Err(HackError::UnrecognizedInstruction(
                format!("{} {} {}", value.0, value.1, value.2),
            )),
            (Ok(_), Ok(_)) => Err(HackError::Internal),
        }
    }
}

impl From<StackManipulation> for Instruction {
    fn from(value: StackManipulation) -> Self {
        Self::StackManipulation(value)
    }
}

impl From<Branching> for Instruction {
    fn from(value: Branching) -> Self {
        Self::Branching(value)
    }
}

impl From<Functional> for Instruction {
    fn from(value: Functional) -> Self {
        Self::Functional(value)
    }
}

impl From<Arithmetic> for Instruction {
    fn from(value: Arithmetic) -> Self {
        Self::Arithmetic(value)
    }
}

/// A valid symbol.
///
/// See [`Symbol::is_allowed_symbol`] for the criteria.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct Symbol {
    /// The actual String containing the value of this [`Symbol`].
    literal_representation: String,
}

impl Symbol {
    /// Borrows a [`str`] slice containing the value of this [`Symbol`].
    pub(crate) fn literal_representation(&self) -> &str {
        &self.literal_representation
    }

    /// Determine if a given string is a valid symbol.
    ///
    /// A symbol must be a sequence of letters (a-z || A-Z), digits (0-9),
    /// underscores (_), dots (.), dollar signs ($), and/or colons (:) that does
    /// not begin with a digit.
    pub(crate) fn is_allowed_symbol(string: &str) -> bool {
        !string.is_empty()
            && !string.contains(|character: char| {
                !(character.is_ascii_alphanumeric()
                    || character == '_'
                    || character == ':'
                    || character == '.'
                    || character == '$')
            })
            && !string
                .starts_with(|character: char| char::is_ascii_digit(&character))
    }
}

impl FromStr for Symbol {
    type Err = HackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_allowed_symbol(s) {
            Ok(Self {
                literal_representation: s.to_owned(),
            })
        } else {
            Err(HackError::SymbolHasForbiddenCharacter)
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.literal_representation)
    }
}

/// A valid constant.
///
/// See [`Constant::MAX_VALID_CONSTANT`] for the upper limit.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) struct Constant {
    /// The actual [`u16`] storing the value of this [`Constant`].
    literal_representation: u16,
}

impl Constant {
    /// The highest valid constant in the Hack computer.
    pub(crate) const MAX_VALID_CONSTANT: u16 = 0x7FFF;

    /// Gets a [`u16`] representing the value of this [`Constant`].
    pub(crate) const fn literal_representation(self) -> u16 {
        self.literal_representation
    }
}

impl TryFrom<u16> for Constant {
    type Error = HackError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value <= Self::MAX_VALID_CONSTANT {
            Ok(Self {
                literal_representation: value,
            })
        } else {
            Err(HackError::Overflow)
        }
    }
}

impl FromStr for Constant {
    type Err = HackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let together: (&str, Result<u16, core::num::ParseIntError>) =
            (s, s.parse::<u16>());

        match together {
            (_, Ok(value)) => Self::try_from(value),
            (s, Err(error)) => Err(HackError::FromStrError(format!(
                "invalid constant: \"{s}\" for reason: {error}"
            ))),
        }
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.literal_representation)
    }
}

/// Stack manipulation instructions.
#[derive(Debug, Clone, Hash)]
pub(crate) enum StackManipulation {
    /// Push a value on to the stack.
    Push {
        /// Where to get the value from.
        symbol: Symbol,
        /// The index.
        value: Constant,
    },
    /// Pop a value off of the stack.
    Pop {
        /// Where to store the value.
        symbol: Symbol,
        /// The index.
        value: Constant,
    },
}

impl StackManipulation {
    /// The string representation of a push command base.
    const PUSH: &str = "push";
    /// The string representation of a pop command base.
    const POP: &str = "pop";

    /// Get the string representation of the base command of this
    /// [`StackManipulation`] instruction.
    pub(crate) const fn name(&self) -> &'static str {
        match self {
            Self::Push { .. } => Self::PUSH,
            Self::Pop { .. } => Self::POP,
        }
    }
}

impl TryFrom<&(&str, Symbol, Constant)> for StackManipulation {
    type Error = HackError;

    fn try_from(value: &(&str, Symbol, Constant)) -> Result<Self, Self::Error> {
        match value {
            (Self::PUSH, symbol, value) => Ok(Self::Push {
                symbol: symbol.clone(),
                value: *value,
            }),
            (Self::POP, symbol, value) => Ok(Self::Pop {
                symbol: symbol.clone(),
                value: *value,
            }),
            (command, symbol, value) => Err(HackError::FromStrError(format!(
                "invalid stack manipulation operation: \"{command} {symbol} {value}\""
            ))),
        }
    }
}

impl Display for StackManipulation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Pop { symbol, value } | Self::Push { symbol, value } => {
                write!(f, "{} {} {}", self.name(), symbol, value)
            }
        }
    }
}

/// Branching instructions.
#[derive(Debug, Clone, Hash)]
pub(crate) enum Branching {
    /// TODO: DOC
    Label {
        /// TODO: DOC
        symbol: Symbol,
    },
    /// TODO: DOC
    GoTo {
        /// TODO: DOC
        symbol: Symbol,
    },
    /// TODO: DOC
    IfGoTo {
        /// TODO: DOC
        symbol: Symbol,
    },
}

impl Branching {
    /// The string representation of a label command base.
    const LABEL: &str = "label";
    /// The string representation of a goto command base.
    const GO_TO: &str = "goto";
    /// The string representation of an if-goto command base.
    const IF_GO_TO: &str = "if-goto";

    /// Get the string representation of the base command of this [`Branching`]
    /// instruction.
    pub(crate) const fn name(&self) -> &'static str {
        match self {
            Self::Label { .. } => Self::LABEL,
            Self::GoTo { .. } => Self::GO_TO,
            Self::IfGoTo { .. } => Self::IF_GO_TO,
        }
    }
}

impl TryFrom<&(&str, Symbol)> for Branching {
    type Error = HackError;

    fn try_from(value: &(&str, Symbol)) -> Result<Self, Self::Error> {
        match value {
            (Self::LABEL, symbol) => Ok(Self::Label {
                symbol: symbol.clone(),
            }),
            (Self::GO_TO, symbol) => Ok(Self::GoTo {
                symbol: symbol.clone(),
            }),
            (Self::IF_GO_TO, symbol) => Ok(Self::IfGoTo {
                symbol: symbol.clone(),
            }),
            (command, symbol) => Err(HackError::FromStrError(format!(
                "invalid branching operation: \"{command} {symbol}\""
            ))),
        }
    }
}

impl Display for Branching {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::GoTo { symbol }
            | Self::Label { symbol }
            | Self::IfGoTo { symbol } => {
                write!(f, "{} {}", self.name(), symbol)
            }
        }
    }
}

/// Functional instructions.
#[derive(Debug, Clone, Hash)]
pub(crate) enum Functional {
    /// TODO: DOC
    Function {
        /// TODO: DOC
        symbol: Symbol,
        /// TODO: DOC
        value: Constant,
    },
    /// TODO: DOC
    Call {
        /// TODO: DOC
        symbol: Symbol,
        /// TODO: DOC
        value: Constant,
    },
    /// TODO: DOC
    Return,
}

impl Functional {
    /// The string representation of a function command base.
    const FUNCTION: &str = "function";
    /// The string representation of a call command base.
    const CALL: &str = "call";
    /// The string representation of a return command.
    const RETURN: &str = "return";

    /// Get the string representation of the base command of this [`Functional`]
    /// instruction.
    pub(crate) const fn name(&self) -> &'static str {
        match self {
            Self::Function { .. } => Self::FUNCTION,
            Self::Call { .. } => Self::CALL,
            Self::Return => Self::RETURN,
        }
    }
}

impl TryFrom<&(&str, Symbol, Constant)> for Functional {
    type Error = HackError;

    fn try_from(value: &(&str, Symbol, Constant)) -> Result<Self, Self::Error> {
        match value {
            (Self::CALL, symbol, value) => Ok(Self::Call {
                symbol: symbol.clone(),
                value: *value,
            }),
            (Self::FUNCTION, symbol, value) => Ok(Self::Function {
                symbol: symbol.clone(),
                value: *value,
            }),
            (command, symbol, value) => Err(HackError::FromStrError(format!(
                "invalid functional operation: \"{command} {symbol} {value}\""
            ))),
        }
    }
}

impl FromStr for Functional {
    type Err = HackError;

    fn from_str(s: &str) -> Result<Self, HackError> {
        match s {
            Self::RETURN => Ok(Self::Return),
            _ => Err(HackError::FromStrError(format!(
                "invalid functional operation: \"{s}\""
            ))),
        }
    }
}

impl Display for Functional {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Call { symbol, value } | Self::Function { symbol, value } => {
                write!(f, "{} {} {}", self.name(), symbol, value)
            }
            Self::Return => write!(f, "{}", self.name()),
        }
    }
}

/// Arithmetic and logic instructions.
#[derive(Debug, Clone, Copy, Hash)]
pub(crate) enum Arithmetic {
    /// Pop two values off the stack, add them, and push the sum back.
    Add,
    /// Pop two values off the stack, subtract them, and push the difference
    /// back.
    Subtract,
    /// Pop a value off the stack, make it negative, and push the result back.
    Negative,
    /// Pop two values off the stack, compare them for equality, and push the
    /// result back.
    Equal,
    /// Pop two values off the stack, do a greater than comparison, and push the
    /// result back.
    GreaterThan,
    /// Pop two values off the stack, do a less than comparison, and push the
    /// result back.
    Lessthan,
    /// Pop two values off the stack, perform a bitwise AND on them, and push
    /// the result back.
    And,
    /// Pop two values off the stack, perform a bitwise OR on them, and push the
    /// result back.
    Or,
    /// Pop a value off the stack, perform a bitwise NOT on it, and push the
    /// result back.
    Not,
}

impl Arithmetic {
    /// The string representation of an add command, and the associated
    /// operator.
    const ADD: [&str; 2] = ["add", "+"];
    /// The string representation of a subtract command, and the associated
    /// operator.
    const SUBTRACT: [&str; 2] = ["sub", "-"];
    /// The string representation of an arithmetic negation command, and the
    /// associated operator.
    const NEGATIVE: [&str; 2] = ["neg", "-"];
    /// The string representation of an equality comparison command, and the
    /// associated "operator".
    const EQUAL: [&str; 2] = ["eq", "JEQ"];
    /// The string representation of a greater than comparison command, and the
    /// associated "operator".
    const GREATER_THAN: [&str; 2] = ["gt", "JGT"];
    /// The string representation of a less than comparison command, and the
    /// associated "operator".
    const LESS_THAN: [&str; 2] = ["lt", "JLT"];
    /// The string representation of a bitwise AND command, and the associated
    /// operator.
    const AND: [&str; 2] = ["and", "&"];
    /// The string representation of a bitwise OR command, and the associated
    /// operator.
    const OR: [&str; 2] = ["or", "|"];
    /// The string representation of a bitwise NOT command, and the associated
    /// operator.
    const NOT: [&str; 2] = ["not", "!"];

    /// Get the string representation of the base command of this [`Arithmetic`]
    /// instruction and the associated operator. Note that the "operator" for
    /// comparisons is the respective assembly jump command, i.e. "JLT" for less
    /// than.
    pub(crate) const fn identify(self) -> [&'static str; 2] {
        match self {
            Self::Add => Self::ADD,
            Self::Subtract => Self::SUBTRACT,
            Self::Negative => Self::NEGATIVE,
            Self::Equal => Self::EQUAL,
            Self::GreaterThan => Self::GREATER_THAN,
            Self::Lessthan => Self::LESS_THAN,
            Self::And => Self::AND,
            Self::Or => Self::OR,
            Self::Not => Self::NOT,
        }
    }
}

impl FromStr for Arithmetic {
    type Err = HackError;

    fn from_str(s: &str) -> Result<Self, HackError> {
        match s {
            add if Self::ADD[0] == add => Ok(Self::Add),
            sub if Self::SUBTRACT[0] == sub => Ok(Self::Subtract),
            neg if Self::NEGATIVE[0] == neg => Ok(Self::Negative),
            eq if Self::EQUAL[0] == eq => Ok(Self::Equal),
            gt if Self::GREATER_THAN[0] == gt => Ok(Self::GreaterThan),
            lt if Self::LESS_THAN[0] == lt => Ok(Self::Lessthan),
            and if Self::AND[0] == and => Ok(Self::And),
            or if Self::OR[0] == or => Ok(Self::Or),
            not if Self::NOT[0] == not => Ok(Self::Not),
            _ => Err(HackError::FromStrError(format!(
                "invalid arithmetic operation: \"{s}\""
            ))),
        }
    }
}

impl Display for Arithmetic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.identify()[0])
    }
}
