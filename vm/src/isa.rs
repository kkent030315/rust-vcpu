//! This module implements Instruction Set Architecture (ISA).
//!
//! An ISA defines the set of instructions that a processor can execute,
//! including arithmetic, logic, control flow, and memory operations. It serves
//! as an interface between hardware and software, dictating how machine code is
//! interpreted and executed.
//!
//! This module provides a software implementation of an ISA, enabling
//! instruction decoding, execution, and eventual execution. It is designed
//! for use in the emulator.

use core::fmt;
use std::io::Write;

use strum_macros::FromRepr;

use crate::{emulator::Register, error};

/// Represents the size classes of operands in a instruction.
#[repr(u8)]
#[derive(FromRepr, Debug, Copy, Clone)]
pub enum OperandSize {
    /// 8-bit operand (byte).
    Byte,
    /// 16-bit operand (word).
    Word,
    /// 32-bit operand (double word).
    DWord,
    /// 64-bit operand (quad word).
    QWord,
}

impl fmt::Display for OperandSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Byte => write!(f, "Byte"),
            Self::Word => write!(f, "Word"),
            Self::DWord => write!(f, "DWord"),
            Self::QWord => write!(f, "QWord"),
        }
    }
}

impl OperandSize {
    /// Reinterpret the [`OperandSize`] from [`str`]
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<Self> {
        match &*s.as_ref().to_lowercase() {
            "byte" | "byte ptr" => Some(Self::Byte),
            "word" | "word ptr" => Some(Self::Word),
            "dword" | "dword ptr" => Some(Self::DWord),
            "qword" | "qword ptr" => Some(Self::QWord),
            _ => None,
        }
    }

    /// Converts [`OperandSize`] into the actual size
    pub fn to_size(&self) -> usize {
        match self {
            Self::Byte => 1,
            Self::Word => 2,
            Self::DWord => 4,
            Self::QWord => 8,
        }
    }
}

/// Represents various operand kinds used in the [`Instruction`].
#[derive(Debug, Clone, Default)]
pub enum Operand {
    /// Indicates no operand.
    ///
    /// This variant represents an instruction with no operands, such as an
    /// instruction that simply performs an operation without requiring
    /// data.
    #[default]
    None,
    /// Indicates a register operand.
    ///
    /// A register operand takes a single [`Register`]. This variant is used
    /// when the operand is a register, such as in register-to-register
    /// operations.
    Register(Register),
    /// Indicates a memory operand.
    ///
    /// Memory operands are used when an instruction needs to access memory.
    /// This variant contains the following fields:
    ///
    /// - `size`: The size of memory to be read or written, represented as an
    ///   [`OperandSize`].
    /// - `displacement`: The offset from a base address to calculate the memory
    ///   location.
    /// - `scale`: A multiplier used for indexed memory access, typically for
    ///   array indexing.
    /// - `index_reg`: An optional register used as an index in indexed
    ///   addressing.
    /// - `base_reg`: An optional register used as the base address.
    Memory {
        size: OperandSize,
        displacement: u64,
        scale: u8,
        index_reg: Option<Register>,
        base_reg: Option<Register>,
    },
    /// Indicates an immediate 64-bit value operand.
    ///
    /// This variant is used for operands that are constant 64-bit values
    /// directly embedded in the instruction, often used in arithmetic or
    /// load operations.
    Immediate64(u64),
    /// Indicates a branch operand.
    ///
    /// A branch operand is typically used for jump or branch instructions, and
    /// contains a signed 64-bit value representing the target address
    /// relative to the current instruction pointer.
    Branch(i64),
}

/// Represents a CPU instruction.
///
/// An instruction is made up of an [`OpCode`] and a set of [`Operand`]s which
/// define the operation and the data on which the operation will be performed.
///
/// The operands can be registers, memory locations, immediate values, or branch
/// addresses, depending on the type of instruction. As per our design
/// principle, we only accept instructions up to two operands.
#[derive(Debug, Clone)]
pub struct Instruction {
    /// The [`OpCode`] for the instruction.
    ///
    /// The opcode represents the operation to be performed, such as an
    /// arithmetic operation, a jump, or a comparison. It is an enumeration
    /// of possible operations supported by the CPU.
    pub opcode: OpCode,
    /// The [`Operand`]s for the instruction.
    ///
    /// The operands are the values or locations on which the instruction
    /// operates. An instruction can have up to two operands.
    pub operands: [Operand; 2],
}

impl Instruction {
    /// Make an new [`Instruction`] instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            opcode: OpCode::from_repr(0).unwrap(),
            operands: Default::default(),
        }
    }

    /// Make an new [`Instruction`] instance with specified [`OpCode`]
    #[must_use]
    pub fn with_opcode(opcode: OpCode) -> Self {
        Self {
            opcode,
            operands: Default::default(),
        }
    }

    /// Sets the [`OpCode`] of this instruction
    pub fn set_opcode(&mut self, opcode: OpCode) {
        self.opcode = opcode;
    }

    /// Returns the [`Register`] of the first operand
    pub fn op0_reg(&self) -> Register {
        match self.operands[0] {
            Operand::Register(reg) => reg,
            _ => unreachable!(),
        }
    }

    /// Sets the [`Register`] of the first operand
    pub fn set_op0_reg(&mut self, reg: Register) {
        self.operands[0] = Operand::Register(reg);
    }

    /// Sets the [`Operand::Memory`] of the first operand
    pub fn set_op0_mem(&mut self, operand: Operand) {
        match operand {
            Operand::Memory {
                size: _,
                displacement: _,
                scale,
                index_reg: _,
                base_reg: _,
            } => {
                assert!(
                    matches!(scale, 1 | 2 | 4 | 8),
                    "Scale must be one of 1, 2, 4, 8"
                );
            }
            _ => unreachable!(),
        }
        self.operands[0] = operand;
    }

    /// Returns the [`Register`] of the second operand
    pub fn op1_reg(&self) -> Register {
        match self.operands[1] {
            Operand::Register(reg) => reg,
            _ => unreachable!(),
        }
    }

    /// Sets the [`Register`] of the second operand
    pub fn set_op1_reg(&mut self, reg: Register) {
        self.operands[1] = Operand::Register(reg);
    }

    /// Sets the [`Operand::Memory`] of the second operand
    pub fn set_op1_mem(&mut self, operand: Operand) {
        match operand {
            Operand::Memory {
                size: _,
                displacement: _,
                scale,
                index_reg: _,
                base_reg: _,
            } => {
                assert!(
                    matches!(scale, 1 | 2 | 4 | 8),
                    "Scale must be one of 1, 2, 4, 8"
                );
            }
            _ => unreachable!(),
        }
        self.operands[1] = operand;
    }

    /// Returns the [`Operand::Immediate64`] of the second operand
    pub fn immediate(&self) -> u64 {
        match &self.operands[1] {
            Operand::Immediate64(imm) => *imm,
            _ => unreachable!(),
        }
    }

    /// Sets the [`Operand::Immediate64`] of the second operand
    pub fn set_immediate(&mut self, imm: u64) {
        self.operands[1] = Operand::Immediate64(imm.into());
    }

    /// Returns the [`OperandSize`] of the specified operand
    pub fn mem_size(&self, op: usize) -> OperandSize {
        match &self.operands[op] {
            Operand::Memory {
                size,
                displacement: _,
                scale: _,
                index_reg: _,
                base_reg: _,
            } => *size,
            _ => unreachable!(),
        }
    }

    /// Sets the [`OperandSize`] of the specified operand
    pub fn set_mem_size(&mut self, op: usize, op_size: OperandSize) {
        match &mut self.operands[op] {
            Operand::Memory {
                size,
                displacement: _,
                scale: _,
                index_reg: _,
                base_reg: _,
            } => {
                *size = op_size;
            }
            _ => unreachable!(),
        }
    }

    /// Returns the [`Operand::Branch`] of the first operand
    pub fn branch_target(&self) -> i64 {
        match &self.operands[0] {
            Operand::Branch(target) => *target,
            _ => unreachable!(),
        }
    }

    /// Sets the [`Operand::Branch`] of the first operand
    pub fn set_branch_target(&mut self, target: i64) {
        self.operands[0] = Operand::Branch(target);
    }

    /// Encodes the instruction into a bytecode and writes it to the
    /// provided writer.
    ///
    /// # Parameters
    /// - `writer`: The output stream where the bytecode will be written. It
    ///   must implement the `Write` trait, such as a file, buffer, or other
    ///   writable object.
    ///
    /// # Returns
    /// - Returns a result indicating success or failure. If successful, it
    ///   returns `Ok(())`. If an error occurs during encoding or writing, it
    ///   returns an [`error::Result`] containing the error.
    pub fn encode<W: Write>(&self, mut writer: W) -> error::Result<()> {
        writer.write(&[self.opcode as u8])?;

        for operand in &self.operands {
            match operand {
                Operand::None => break,
                Operand::Register(reg) => {
                    writer.write(&[*reg as u8])?;
                }
                Operand::Memory {
                    size,
                    displacement,
                    scale,
                    index_reg,
                    base_reg,
                } => {
                    writer.write(&[*size as u8])?;
                    writer.write(&displacement.to_le_bytes())?;
                    writer.write(&[*scale as u8])?;
                    match index_reg {
                        Some(reg) => {
                            writer.write(&[*reg as u8])?;
                        }
                        None => {
                            writer.write(&[u8::MAX])?;
                        }
                    };
                    match base_reg {
                        Some(reg) => {
                            writer.write(&[*reg as u8])?;
                        }
                        None => {
                            writer.write(&[u8::MAX])?;
                        }
                    };
                }
                Operand::Immediate64(imm) => {
                    writer.write(&imm.to_le_bytes())?;
                }
                Operand::Branch(target) => {
                    writer.write(&target.to_le_bytes())?;
                }
            }
        }

        Ok(())
    }
}

/// Represents the mnemonic of an assembly instruction.
///
/// A mnemonic is the human-readable name of an [`OpCode`] that corresponds
/// to a specific operation in machine code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mnemonic {
    /// Terminates execution.
    Exit,
    /// Executes an undefined instruction that typically used to trigger an
    /// exception.
    Ud,
    /// Moves data from one location to another.
    Mov,
    /// Performs addition.
    Add,
    /// Performs subtraction.
    Sub,
    /// Performs a bitwise AND operation.
    And,
    /// Performs a bitwise OR operation.
    Or,
    /// Performs a bitwise XOR operation.
    Xor,
    /// Exchanges the values of two operands.
    Xchg,
    /// Performs signed integer multiplication.
    Imul,
    /// Increments the value of an operand.
    Inc,
    /// Decrements the value of an operand.
    Dec,
    /// Performs a bitwise AND operation but does not store the result.
    Test,
    /// Compares two values by subtracting one from the other but does not store
    /// the result.
    Cmp,
    /// Unconditional jump to a different instruction.
    Jmp,
    /// Jumps if the zero flag (ZF) is set (equal to zero).
    Jz,
    /// Jumps if the zero flag (ZF) is not set (not equal to zero).
    Jnz,
    /// Jumps if less than or equal (ZF = 1 or SF ≠ OF).
    Jle,
    /// Jumps if greater (ZF = 0 and SF = OF).
    Jg,
    /// Jumps if greater than or equal (SF = OF).
    Jge,
    /// Jumps if below (CF = 1).
    Jb,

    /// Defines a byte (8-bit value).
    Db,
    /// Defines a word (16-bit value).
    Dw,
    /// Defines a double word (32-bit value).
    Dd,
    /// Defines a quad word (64-bit value).
    Dq,
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exit => write!(f, "Exit"),
            Self::Ud => write!(f, "Ud"),
            Self::Mov => write!(f, "Mov"),
            Self::Add => write!(f, "Add"),
            Self::Sub => write!(f, "Sub"),
            Self::And => write!(f, "And"),
            Self::Or => write!(f, "Or"),
            Self::Xor => write!(f, "Xor"),
            Self::Xchg => write!(f, "Xchg"),
            Self::Imul => write!(f, "imul"),
            Self::Inc => write!(f, "Inc"),
            Self::Dec => write!(f, "Dec"),
            Self::Test => write!(f, "Test"),
            Self::Cmp => write!(f, "Cmp"),
            Self::Jmp => write!(f, "Jmp"),
            Self::Jz => write!(f, "Jz"),
            Self::Jnz => write!(f, "Jnz"),
            Self::Jle => write!(f, "Jle"),
            Self::Jg => write!(f, "Jg"),
            Self::Jge => write!(f, "Jge"),
            Self::Jb => write!(f, "Jb"),

            Self::Db => write!(f, "Db"),
            Self::Dw => write!(f, "Dw"),
            Self::Dd => write!(f, "Dd"),
            Self::Dq => write!(f, "Dq"),
        }
    }
}

impl Mnemonic {
    /// Reinterpret the [`Mnemonic`] from [`str`]
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<Self> {
        match &*s.as_ref().to_lowercase() {
            "exit" => Some(Self::Exit),
            "ud" => Some(Self::Ud),
            "mov" => Some(Self::Mov),
            "add" => Some(Self::Add),
            "sub" => Some(Self::Sub),
            "and" => Some(Self::And),
            "or" => Some(Self::Or),
            "xor" => Some(Self::Xor),
            "xchg" => Some(Self::Xchg),
            "imul" => Some(Self::Imul),
            "inc" => Some(Self::Inc),
            "dec" => Some(Self::Dec),
            "test" => Some(Self::Test),
            "cmp" => Some(Self::Cmp),
            "jmp" => Some(Self::Jmp),
            "jz" => Some(Self::Jz),
            "jnz" => Some(Self::Jnz),
            "jle" => Some(Self::Jle),
            "jg" => Some(Self::Jg),
            "jge" => Some(Self::Jge),
            "jb" => Some(Self::Jb),

            "db" => Some(Self::Db),
            "dw" => Some(Self::Dw),
            "dd" => Some(Self::Dd),
            "dq" => Some(Self::Dq),

            _ => None,
        }
    }

    /// Returns whether the mnemonic is corresponds to the data identifiers
    pub fn is_data(&self) -> bool {
        matches!(self, Self::Db | Self::Dw | Self::Dd | Self::Dq)
    }

    /// Returns the minimum number of operands that this mnemonic supports
    pub fn min_operands(&self) -> usize {
        match self {
            Self::Exit => 0,
            Self::Ud => 0,
            Self::Mov => 2,
            Self::Add => 2,
            Self::Sub => 2,
            Self::And => 2,
            Self::Or => 2,
            Self::Xor => 2,
            Self::Xchg => 2,
            Self::Imul => 2,
            Self::Inc => 1,
            Self::Dec => 1,
            Self::Test => 2,
            Self::Cmp => 2,
            Self::Jmp => 1,
            Self::Jz => 1,
            Self::Jnz => 1,
            Self::Jle => 1,
            Self::Jg => 1,
            Self::Jge => 1,
            Self::Jb => 1,

            Self::Db => 1,
            Self::Dw => 1,
            Self::Dd => 1,
            Self::Dq => 1,
        }
    }

    /// Returns the maximum number of operands that this mnemonic supports
    pub fn max_operands(&self) -> usize {
        match self {
            Self::Exit => 0,
            Self::Ud => 0,
            Self::Mov => 2,
            Self::Add => 2,
            Self::Sub => 2,
            Self::And => 2,
            Self::Or => 2,
            Self::Xor => 2,
            Self::Xchg => 2,
            Self::Imul => 2,
            Self::Inc => 1,
            Self::Dec => 1,
            Self::Test => 2,
            Self::Cmp => 2,
            Self::Jmp => 1,
            Self::Jz => 1,
            Self::Jnz => 1,
            Self::Jle => 1,
            Self::Jg => 1,
            Self::Jge => 1,
            Self::Jb => 1,

            Self::Db => 1,
            Self::Dw => 1,
            Self::Dd => 1,
            Self::Dq => 1,
        }
    }
}

/// Represents the raw operation codes for each [`Mnemonic`]
#[repr(u8)]
#[derive(FromRepr, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Exit,
    Ud,
    MovRIMM,
    MovRR,
    MovRRM,
    MovRMR,
    AddRIMM,
    AddRR,
    SubRIMM,
    SubRR,
    AndRIMM,
    AndRR,
    OrRIMM,
    OrRR,
    XorRIMM,
    XorRR,
    XchgRR,
    ImulRIMM,
    ImulRR,
    IncR,
    DecR,
    TestRIMM,
    TestRR,
    CmpRIMM,
    CmpRR,
    Jmp,
    Jz,
    Jnz,
    Jle,
    Jg,
    Jge,
    Jb,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exit => write!(f, "Exit"),
            Self::Ud => write!(f, "Ud"),
            Self::MovRIMM => write!(f, "MovRIMM"),
            Self::MovRR => write!(f, "MovRR"),
            Self::MovRRM => write!(f, "MovRRM"),
            Self::MovRMR => write!(f, "MovRMR"),
            Self::AddRIMM => write!(f, "AddRIMM"),
            Self::AddRR => write!(f, "AddRR"),
            Self::SubRIMM => write!(f, "SubRIMM"),
            Self::SubRR => write!(f, "SubRR"),
            Self::AndRIMM => write!(f, "AndRIMM"),
            Self::AndRR => write!(f, "AndRR"),
            Self::OrRIMM => write!(f, "OrRIMM"),
            Self::OrRR => write!(f, "OrRR"),
            Self::XorRIMM => write!(f, "XorRIMM"),
            Self::XorRR => write!(f, "XorRR"),
            Self::XchgRR => write!(f, "XchgRR"),
            Self::ImulRIMM => write!(f, "ImulRIMM"),
            Self::ImulRR => write!(f, "ImulRR"),
            Self::IncR => write!(f, "IncR"),
            Self::DecR => write!(f, "DecR"),
            Self::TestRIMM => write!(f, "TestRIMM"),
            Self::TestRR => write!(f, "TestRR"),
            Self::CmpRIMM => write!(f, "CmpRIMM"),
            Self::CmpRR => write!(f, "CmpRR"),
            Self::Jmp => write!(f, "Jmp"),
            Self::Jz => write!(f, "Jz"),
            Self::Jnz => write!(f, "Jnz"),
            Self::Jle => write!(f, "Jle"),
            Self::Jg => write!(f, "Jg"),
            Self::Jge => write!(f, "Jge"),
            Self::Jb => write!(f, "Jb"),
        }
    }
}
