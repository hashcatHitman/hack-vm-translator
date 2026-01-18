// SPDX-FileCopyrightText: Copyright © 2025 hashcatHitman
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Hack VM Translator
//!
//! A VM translator that parses Hack VM commands and generates Hack assembly.
//! Based on the nand2tetris course.

use core::ops::RangeInclusive;

use crate::error::HackError;
use crate::parser::{Arithmetic, Constant, Instruction, Symbol};

/// Each Segment is a virtual memory location, represented by predefined
/// symbols.
pub(crate) enum Segment {
    /// Constant is for constants. You can push a constant on to the stack, but
    /// you can't pop something off the stack into constant.
    Constant,
    /// TODO: DOC
    Local,
    /// TODO: DOC
    Argument,
    /// TODO: DOC
    This,
    /// TODO: DOC
    That,
    /// TODO: DOC
    Static,
    /// TODO: DOC
    Temp,
    /// TODO: DOC
    Pointer,
}

impl Segment {
    /// Some segments have special predefined symbols which point to the memory
    /// which points to their location. This returns that symbol, if it exists.
    pub(crate) const fn base(&self) -> Result<&str, HackError> {
        match self {
            Self::Local => Ok("LCL"),
            Self::Argument => Ok("ARG"),
            Self::This => Ok("THIS"),
            Self::That => Ok("THAT"),
            _ => Err(HackError::Internal),
        }
    }
}

impl TryFrom<Symbol> for Segment {
    type Error = HackError;

    fn try_from(value: Symbol) -> Result<Self, Self::Error> {
        match value.literal_representation() {
            "constant" => Ok(Self::Constant),
            "local" => Ok(Self::Local),
            "argument" => Ok(Self::Argument),
            "this" => Ok(Self::This),
            "that" => Ok(Self::That),
            "static" => Ok(Self::Static),
            "temp" => Ok(Self::Temp),
            "pointer" => Ok(Self::Pointer),
            bad => Err(HackError::FromStrError(format!(
                "\"{bad}\" is not a recognized segment"
            ))),
        }
    }
}

impl TryFrom<&Symbol> for Segment {
    type Error = HackError;

    fn try_from(value: &Symbol) -> Result<Self, Self::Error> {
        match value.literal_representation() {
            "constant" => Ok(Self::Constant),
            "local" => Ok(Self::Local),
            "argument" => Ok(Self::Argument),
            "this" => Ok(Self::This),
            "that" => Ok(Self::That),
            "static" => Ok(Self::Static),
            "temp" => Ok(Self::Temp),
            "pointer" => Ok(Self::Pointer),
            bad => Err(HackError::FromStrError(format!(
                "\"{bad}\" is not a recognized segment"
            ))),
        }
    }
}

/// An empty enum with associated methods for translating Hack VM instructions
/// into Hack assembly.
pub(crate) enum Translator {}

impl Translator {
    /// The temp segment starts at RAM[5].
    const TEMP_BASE: u16 = 5;
    /// The temp segment ends at RAM[12].
    const TEMP_MAX: u16 = 12;
    /// The general use registers are 13-15.
    const GENERAL_REGISTERS: RangeInclusive<u8> = 13..=15;

    /// Translate the Hack VM instruction given into Hack assembly.
    pub(crate) fn translate(
        line_number: usize,
        instruction: &Instruction,
        file_name: &str,
    ) -> Result<Vec<String>, HackError> {
        match instruction {
            Instruction::StackManipulation(stack_manipulation) => {
                match stack_manipulation {
                    crate::parser::StackManipulation::Push {
                        symbol,
                        value,
                    } => {
                        let seg: Segment = Segment::try_from(symbol)?;
                        Self::push(&seg, *value, file_name)
                    }
                    crate::parser::StackManipulation::Pop { symbol, value } => {
                        let seg: Segment = Segment::try_from(symbol)?;
                        Self::pop(&seg, *value, file_name)
                    }
                }
            }
            Instruction::Branching(_branching) => todo!(),
            Instruction::Functional(_functional) => todo!(),
            Instruction::Arithmetic(arithmetic) => {
                Ok(Self::arithmetic(*arithmetic, line_number))
            }
        }
    }

    /// Translate arithmetic/logic Hack VM instructions into Hack assembly.
    pub(crate) fn arithmetic(
        op: Arithmetic,
        line_number: usize,
    ) -> Vec<String> {
        match op {
            Arithmetic::Negative | Arithmetic::Not => [
                "@SP".to_owned(),
                "A=M-1".to_owned(),
                format!("M={}M", op.identify()[1]),
            ]
            .to_vec(),
            Arithmetic::Add
            | Arithmetic::Subtract
            | Arithmetic::Equal
            | Arithmetic::GreaterThan
            | Arithmetic::Lessthan
            | Arithmetic::And
            | Arithmetic::Or => {
                let common: Vec<String> = [
                    "@SP".to_owned(),
                    "AM=M-1".to_owned(),
                    "D=M".to_owned(),
                    "A=A-1".to_owned(),
                ]
                .to_vec();
                let impossible: &str = "[`Arithmetic::Not`] and \
                [`Arithmetic::Negative`] should have already been matched";
                let unique = match op {
                    Arithmetic::Lessthan
                    | Arithmetic::GreaterThan
                    | Arithmetic::Equal => [
                        "D=M-D".to_owned(),
                        format!("@CRASH_{line_number}"),
                        format!("D;{}", op.identify()[1]),
                        "@SP".to_owned(),
                        "A=M-1".to_owned(),
                        "M=0".to_owned(),
                        format!("@BURN_{line_number}"),
                        "0;JMP".to_owned(),
                        format!("(CRASH_{line_number})"),
                        "@SP".to_owned(),
                        "A=M-1".to_owned(),
                        "M=-1".to_owned(),
                        format!("(BURN_{line_number})"),
                    ]
                    .to_vec(),
                    Arithmetic::And | Arithmetic::Add | Arithmetic::Or => {
                        [format!("M=D{}M", op.identify()[1])].to_vec()
                    }
                    Arithmetic::Subtract => {
                        [format!("M=M{}D", op.identify()[1])].to_vec()
                    }
                    Arithmetic::Not | Arithmetic::Negative => {
                        unreachable!("{impossible}")
                    }
                };

                let mut common: Vec<String> = common;
                common.extend(unique);
                let common: Vec<String> = common;

                common
            }
        }
    }

    /// Helper function. Returns the Hack assembly to push the current value of
    /// the data register onto the stack.
    pub(crate) fn push_from_data_register() -> [String; 5] {
        [
            // RAM[SP] <- D
            "@SP".to_owned(),
            "A=M".to_owned(),
            "M=D".to_owned(),
            // SP++
            "@SP".to_owned(),
            "M=M+1".to_owned(),
        ]
    }

    /// Push a value  from the chosen segment onto the stack.
    pub(crate) fn push(
        segment: &Segment,
        i: Constant,
        file_name: &str,
    ) -> Result<Vec<String>, HackError> {
        let unique: Vec<String> = match segment {
            Segment::Constant => {
                [
                    // D = i
                    format!("@{i}"),
                    "D=A".to_owned(),
                ]
                .to_vec()
            }
            Segment::Argument
            | Segment::This
            | Segment::That
            | Segment::Local => {
                [
                    // D = segment[i]
                    format!("@{i}"),
                    "D=A".to_owned(),
                    format!("@{}", segment.base()?),
                    "A=D+M".to_owned(),
                    "D=M".to_owned(),
                ]
                .to_vec()
            }
            Segment::Static => {
                [
                    // D = RAM[Xxx.i]
                    format!("@{file_name}.{i}"),
                    "D=M".to_owned(),
                ]
                .to_vec()
            }
            Segment::Temp => {
                let address: u16 = i.literal_representation() + Self::TEMP_BASE;
                if (Self::TEMP_BASE..=Self::TEMP_MAX).contains(&address) {
                    [
                        // D = RAM[5 + i]
                        format!("@{address}"),
                        "D=M".to_owned(),
                    ]
                    .to_vec()
                } else {
                    return Err(HackError::IllegalInstruction(format!(
                        "\"{i}\" is not a valid index for temp, must be {} <= \
                        i <= {}",
                        0,
                        Self::TEMP_MAX - Self::TEMP_BASE
                    )));
                }
            }
            Segment::Pointer => {
                match i.literal_representation() {
                    0 => {
                        [
                            // D = RAM[3]
                            "@THIS".to_owned(),
                            "D=M".to_owned(),
                        ]
                        .to_vec()
                    }
                    1 => {
                        [
                            // D = RAM[4]
                            "@THAT".to_owned(),
                            "D=M".to_owned(),
                        ]
                        .to_vec()
                    }
                    i => {
                        return Err(HackError::IllegalInstruction(format!(
                            "\"{i}\" is not a valid index for temp, must be {} \
                            <= i <= {}",
                            0,
                            Self::TEMP_MAX - Self::TEMP_BASE
                        )));
                    }
                }
            }
        };

        let mut unique: Vec<String> = unique;
        unique.extend(Self::push_from_data_register());
        let unique: Vec<String> = unique;

        Ok(unique)
    }

    /// Helper function. Takes the current value in the data register and moves
    /// it into the general register selected.
    pub(crate) fn save_data_register_in_general(
        number: u8,
    ) -> Result<Vec<String>, HackError> {
        if Self::GENERAL_REGISTERS.contains(&number) {
            Ok([
                // RAM[R{number}] <- D
                format!("@R{number}"),
                "M=D".to_owned(),
            ]
            .to_vec())
        } else {
            Err(HackError::Internal)
        }
    }

    /// Helper function. Pops a value off the stack and stores it in the
    /// general register selected.
    pub(crate) fn pop_to_general(number: u8) -> Result<Vec<String>, HackError> {
        if Self::GENERAL_REGISTERS.contains(&number) {
            Ok([
                // SP--
                "@SP".to_owned(),
                "AM=M-1".to_owned(),
                // D=stack.pop!
                "D=M".to_owned(),
                // RAM[R{number}] <- stack.pop!
                format!("@R{number}"),
                "A=M".to_owned(),
                "M=D".to_owned(),
            ]
            .to_vec())
        } else {
            Err(HackError::Internal)
        }
    }

    /// Pops a value off the stack and into the selected segment.
    pub(crate) fn pop(
        segment: &Segment,
        i: Constant,
        file_name: &str,
    ) -> Result<Vec<String>, HackError> {
        let unique: Vec<String> = match segment {
            Segment::That
            | Segment::Local
            | Segment::Argument
            | Segment::This => {
                [
                    // D = RAM[segment_base] + i == segment[i].address
                    format!("@{i}"),
                    "D=A".to_owned(),
                    format!("@{}", segment.base()?),
                    "D=D+M".to_owned(),
                ]
                .to_vec()
            }
            Segment::Static => {
                [
                    // D = RAM[Xxx.i]
                    format!("@{file_name}.{i}"),
                    "D=A".to_owned(),
                ]
                .to_vec()
            }
            Segment::Temp => {
                let address = i.literal_representation() + Self::TEMP_BASE;
                if (Self::TEMP_BASE..=Self::TEMP_MAX).contains(&address) {
                    [
                        // D = RAM[5 + i]
                        format!("@{address}"),
                        "D=A".to_owned(),
                    ]
                    .to_vec()
                } else {
                    return Err(HackError::IllegalInstruction(format!(
                        "\"{i}\" is not a valid index for temp, must be {} <= \
                        i <= {}",
                        0,
                        Self::TEMP_MAX - Self::TEMP_BASE
                    )));
                }
            }
            Segment::Pointer => {
                match i.literal_representation() {
                    0 => {
                        [
                            // D = 3
                            "@THIS".to_owned(),
                            "D=A".to_owned(),
                        ]
                        .to_vec()
                    }
                    1 => {
                        [
                            // D = 4
                            "@THAT".to_owned(),
                            "D=A".to_owned(),
                        ]
                        .to_vec()
                    }
                    i => {
                        return Err(HackError::IllegalInstruction(format!(
                            "\"{i}\" is not a valid index for temp, must be {} \
                            <= i <= {}",
                            0,
                            Self::TEMP_MAX - Self::TEMP_BASE
                        )));
                    }
                }
            }
            Segment::Constant => {
                return Err(HackError::IllegalInstruction(
                    "\"pop constant n\" is never a valid instruction, \
                    regardless of the value of n"
                        .to_owned(),
                ));
            }
        };

        let mut unique: Vec<String> = unique;
        unique.extend(Self::save_data_register_in_general(13)?);
        unique.extend(Self::pop_to_general(13)?);
        let unique: Vec<String> = unique;

        Ok(unique)
    }
}
