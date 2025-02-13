//! This module implements a virtual Central Processing Unit (CPU) for executing
//! instructions and handling exceptions.
//!
//! The virtual CPU emulates the behavior of a hardware processor, executing
//! instructions from a defined Instruction Set Architecture (ISA) in a software
//! environment. It performs operations such as arithmetic, logic, memory
//! access, and control flow, and manages the system state (including
//! registers and memory). The virtual CPU also handles exceptions like illegal
//! instructions and memory access violations, making it useful for testing,
//! debugging, and exploring low-level system behavior.

use core::fmt;

use strum_macros::FromRepr;

use crate::{
    alu::*,
    exception::Exception,
    isa::{Instruction, OpCode, Operand, OperandSize},
    ram::Dram,
};

/// Represents the state of a virtual CPU (Emulator).
///
/// This struct holds the current state of the emulator, including its registers
/// and DRAM (memory). It simulates the behavior of a CPU, handling the
/// manipulation of registers and memory, while allowing for the execution of
/// instructions in a software-based environment.
#[derive(Debug, Clone)]
pub struct Emulator {
    /// A set of registers representing the CPU state.
    pub regs: Registers,
    /// The Dynamic Random-Access Memory (DRAM) of the emulator.
    pub dram: Dram,
    /// The clock cycle state
    pub cycle: u64,
}

impl Default for Emulator {
    fn default() -> Self {
        Self {
            regs: Default::default(),
            dram: Default::default(),
            cycle: 0,
        }
    }
}

impl Emulator {
    /// Make an new instance of [`Emulator`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Make an new instance of [`Emulator`] with a VM bytecode
    #[must_use]
    pub fn with_bytecode<S: Into<Vec<u8>>>(bytecode: S) -> Self {
        Self {
            dram: Dram::with_data(bytecode),
            ..Default::default()
        }
    }

    /// Reset the CPU state
    pub fn reset(&mut self) {
        self.regs.reset();
        self.cycle = 0;
    }

    /// Fetches an 8-bit unsigned integer from DRAM at the current
    /// instruction pointer (IP) and increments IP if succeeded.
    ///
    /// # Returns
    /// - `Ok(u8)`: The value read from the current IP.
    /// - `Err(Exception::AccessViolation)`: If the read operation exceeds
    ///   memory bounds.
    pub fn fetch_u8(&mut self) -> Result<u8, Exception> {
        self.dram.read_u8(self.ip() as usize).and_then(|x| {
            self.increment_ip(core::mem::size_of_val(&x) as u64);
            Ok(x)
        })
    }

    /// Fetches an 16-bit unsigned integer from DRAM at the current
    /// instruction pointer (IP) and increments IP if succeeded.
    ///
    /// # Returns
    /// - `Ok(u16)`: The value read from the current IP.
    /// - `Err(Exception::AccessViolation)`: If the read operation exceeds
    ///   memory bounds.
    pub fn fetch_u16le(&mut self) -> Result<u16, Exception> {
        self.dram.read_u16le(self.ip() as usize).map(|x| {
            self.increment_ip(core::mem::size_of_val(&x) as u64);
            x
        })
    }

    /// Fetches an 32-bit unsigned integer from DRAM at the current
    /// instruction pointer (IP) and increments IP if succeeded.
    ///
    /// # Returns
    /// - `Ok(u32)`: The value read from the current IP.
    /// - `Err(Exception::AccessViolation)`: If the read operation exceeds
    ///   memory bounds.
    pub fn fetch_u32le(&mut self) -> Result<u32, Exception> {
        self.dram.read_u32le(self.ip() as usize).map(|x| {
            self.increment_ip(core::mem::size_of_val(&x) as u64);
            x
        })
    }

    /// Fetches an 64-bit unsigned integer from DRAM at the current
    /// instruction pointer (IP) and increments IP if succeeded.
    ///
    /// # Returns
    /// - `Ok(u64)`: The value read from the current IP.
    /// - `Err(Exception::AccessViolation)`: If the read operation exceeds
    ///   memory bounds.
    pub fn fetch_u64le(&mut self) -> Result<u64, Exception> {
        self.dram.read_u64le(self.ip() as usize).map(|x| {
            self.increment_ip(core::mem::size_of_val(&x) as u64);
            x
        })
    }

    /// Returns the current instruction pointer (IP)
    pub fn ip(&self) -> u64 {
        self.regs.read(Register::IP).into()
    }

    /// Increments the current instruction pointer (IP) with the given value
    pub fn increment_ip(&mut self, value: u64) {
        self.regs.write(Register::IP, self.ip() + value);
    }

    /// Sets the current instruction pointer (IP) to the given value
    pub fn set_ip(&mut self, value: u64) {
        self.regs.write(Register::IP, value);
    }

    /// Decodes Op/R/IMM pattern instructions.
    fn decode_r_imm(&mut self, insn: &mut Instruction) -> Result<(), Exception> {
        let reg = Register::from_repr(self.fetch_u8()?).ok_or(Exception::IllegalInstruction)?;
        let imm = self.fetch_u64le()?;
        insn.set_op0_reg(reg);
        insn.set_immediate(imm.into());

        Ok(())
    }

    /// Decodes Op/R/RM pattern instructions.
    fn decode_r_rm(&mut self, insn: &mut Instruction) -> Result<(), Exception> {
        let reg = Register::from_repr(self.fetch_u8()?).ok_or(Exception::IllegalInstruction)?;
        insn.set_op0_reg(reg);

        let size = OperandSize::from_repr(self.fetch_u8()?).ok_or(Exception::IllegalInstruction)?;
        let displacement = self.fetch_u64le()?;
        let scale = self.fetch_u8()?;
        let index_reg = Register::from_repr(self.fetch_u8()?);
        let base_reg = Register::from_repr(self.fetch_u8()?);
        insn.set_op1_mem(Operand::Memory {
            size,
            displacement,
            scale,
            index_reg,
            base_reg,
        });

        Ok(())
    }

    /// Decodes Op/RM/R pattern instructions.
    fn decode_rm_r(&mut self, insn: &mut Instruction) -> Result<(), Exception> {
        let size = OperandSize::from_repr(self.fetch_u8()?).ok_or(Exception::IllegalInstruction)?;
        let displacement = self.fetch_u64le()?;
        let scale = self.fetch_u8()?;
        let index_reg = Register::from_repr(self.fetch_u8()?);
        let base_reg = Register::from_repr(self.fetch_u8()?);
        insn.set_op0_mem(Operand::Memory {
            size,
            displacement,
            scale,
            index_reg,
            base_reg,
        });

        let reg = Register::from_repr(self.fetch_u8()?).ok_or(Exception::IllegalInstruction)?;
        insn.set_op1_reg(reg);

        Ok(())
    }

    /// Decodes Op/R/R pattern instructions.
    fn decode_r_r(&mut self, insn: &mut Instruction) -> Result<(), Exception> {
        let reg0 = Register::from_repr(self.fetch_u8()?).ok_or(Exception::IllegalInstruction)?;
        let reg1 = Register::from_repr(self.fetch_u8()?).ok_or(Exception::IllegalInstruction)?;
        insn.set_op0_reg(reg0);
        insn.set_op1_reg(reg1);

        Ok(())
    }

    /// Decodes Op/R pattern instructions.
    fn decode_r(&mut self, insn: &mut Instruction) -> Result<(), Exception> {
        let reg0 = Register::from_repr(self.fetch_u8()?).ok_or(Exception::IllegalInstruction)?;
        insn.set_op0_reg(reg0);

        Ok(())
    }

    /// Decodes Op/IMM pattern instructions.
    fn decode_branch(&mut self, insn: &mut Instruction) -> Result<(), Exception> {
        let target = self.fetch_u64le()?;
        insn.set_branch_target(target as i64);

        Ok(())
    }

    /// Decodes an opcode into an [`Instruction`].
    ///
    /// This function takes an [`OpCode`], which represents a machine
    /// instruction's binary encoding, and decodes it into a more usable
    /// [`Instruction`] structure. The decoding process maps the raw opcode to
    /// its corresponding operation, operands, and instruction type. If an
    /// invalid opcode is provided, an [`Exception::IllegalInstruction`]
    /// exception is returned.
    ///
    /// # Arguments
    /// - `opcode`: The raw opcode defined by ISA.
    ///
    /// # Returns
    /// - `Ok(Instruction)`: The decoded [`Instruction`].
    /// - `Err(Exception::IllegalInstruction)`: If the provided opcode is
    ///   invalid or unsupported.
    pub fn decode(&mut self, opcode: OpCode) -> Result<Instruction, Exception> {
        let mut insn = Instruction::with_opcode(opcode);

        match opcode {
            OpCode::Exit => {} // No operands
            OpCode::Ud => {}   // No operands
            OpCode::MovRIMM => self.decode_r_imm(&mut insn)?,
            OpCode::MovRR => self.decode_r_r(&mut insn)?,
            OpCode::MovRRM => self.decode_r_rm(&mut insn)?,
            OpCode::MovRMR => self.decode_rm_r(&mut insn)?,
            OpCode::AddRIMM => self.decode_r_imm(&mut insn)?,
            OpCode::AddRR => self.decode_r_r(&mut insn)?,
            OpCode::SubRIMM => self.decode_r_imm(&mut insn)?,
            OpCode::SubRR => self.decode_r_r(&mut insn)?,
            OpCode::AndRIMM => self.decode_r_imm(&mut insn)?,
            OpCode::AndRR => self.decode_r_r(&mut insn)?,
            OpCode::OrRIMM => self.decode_r_imm(&mut insn)?,
            OpCode::OrRR => self.decode_r_r(&mut insn)?,
            OpCode::XorRIMM => self.decode_r_imm(&mut insn)?,
            OpCode::XorRR => self.decode_r_r(&mut insn)?,
            OpCode::XchgRR => self.decode_r_r(&mut insn)?,
            OpCode::ImulRIMM => self.decode_r_imm(&mut insn)?,
            OpCode::ImulRR => self.decode_r_r(&mut insn)?,
            OpCode::IncR => self.decode_r(&mut insn)?,
            OpCode::DecR => self.decode_r(&mut insn)?,
            OpCode::TestRIMM => self.decode_r_imm(&mut insn)?,
            OpCode::TestRR => self.decode_r_r(&mut insn)?,
            OpCode::CmpRIMM => self.decode_r_imm(&mut insn)?,
            OpCode::CmpRR => self.decode_r_r(&mut insn)?,
            OpCode::Jmp => self.decode_branch(&mut insn)?,
            OpCode::Jz => self.decode_branch(&mut insn)?,
            OpCode::Jnz => self.decode_branch(&mut insn)?,
            OpCode::Jle => self.decode_branch(&mut insn)?,
            OpCode::Jg => self.decode_branch(&mut insn)?,
            OpCode::Jge => self.decode_branch(&mut insn)?,
            OpCode::Jb => self.decode_branch(&mut insn)?,
        };

        Ok(insn)
    }

    /// Executes a single step of instruction in the emulator.
    ///
    /// This function emulates a single cycle of the virtual CPU by fetching,
    /// decoding, and executing single instruction. It updates the state of the
    /// emulator accordingly. If the instruction encounters any exceptions
    /// (e.g., illegal instruction or memory access violation), the function
    /// will return the appropriate [`Exception`].
    ///
    /// # Returns
    /// - `Ok(())`: If the instruction executes successfully without errors.
    /// - `Err(Exception)`: If an exception occurs during instruction execution
    ///   (e.g., [`Exception::IllegalInstruction`],
    ///   [`Exception::AccessViolation`]).
    pub fn single_step(&mut self) -> Result<(), Exception> {
        let opcode = OpCode::from_repr(self.fetch_u8()?).ok_or(Exception::IllegalInstruction)?;
        let insn = self.decode(opcode)?;

        match opcode {
            OpCode::Exit => return Err(Exception::Exit),
            OpCode::Ud => return Err(Exception::IllegalInstruction),
            OpCode::MovRIMM => self.handle_mov_r_imm(&insn)?,
            OpCode::MovRR => self.handle_mov_r_r(&insn)?,
            OpCode::MovRRM => self.handle_mov_r_rm(&insn)?,
            OpCode::MovRMR => self.handle_mov_rm_r(&insn)?,
            OpCode::AddRIMM => self.handle_add_r_imm(&insn)?,
            OpCode::AddRR => self.handle_add_r_r(&insn)?,
            OpCode::SubRIMM => self.handle_sub_r_imm(&insn)?,
            OpCode::SubRR => self.handle_sub_r_r(&insn)?,
            OpCode::AndRIMM => self.handle_and_r_imm(&insn)?,
            OpCode::AndRR => self.handle_and_r_r(&insn)?,
            OpCode::OrRIMM => self.handle_or_r_imm(&insn)?,
            OpCode::OrRR => self.handle_or_r_r(&insn)?,
            OpCode::XorRIMM => self.handle_xor_r_imm(&insn)?,
            OpCode::XorRR => self.handle_xor_r_r(&insn)?,
            OpCode::XchgRR => self.handle_xchg_r_r(&insn)?,
            OpCode::ImulRIMM => self.handle_imul_r_imm(&insn)?,
            OpCode::ImulRR => self.handle_imul_r_r(&insn)?,
            OpCode::IncR => self.handle_inc_r(&insn)?,
            OpCode::DecR => self.handle_dec_r(&insn)?,
            OpCode::TestRIMM => self.handle_test_r_imm(&insn)?,
            OpCode::TestRR => self.handle_test_r_r(&insn)?,
            OpCode::CmpRIMM => self.handle_cmp_r_imm(&insn)?,
            OpCode::CmpRR => self.handle_cmp_r_r(&insn)?,
            OpCode::Jmp => self.handle_jmp(&insn)?,
            OpCode::Jz => self.handle_jz(&insn)?,
            OpCode::Jnz => self.handle_jnz(&insn)?,
            OpCode::Jle => self.handle_jle(&insn)?,
            OpCode::Jg => self.handle_jg(&insn)?,
            OpCode::Jge => self.handle_jge(&insn)?,
            OpCode::Jb => self.handle_jb(&insn)?,
        }

        self.cycle += 1;

        Ok(())
    }

    /// Executes the program in a loop until an exit condition is met or an
    /// error occurs.
    ///
    /// This function continuously executes instructions one by one by calling
    /// [`Self::single_step`]. If [`Self::single_step`] completes successfully,
    /// the next instruction is executed.
    ///
    /// Whenever an exception occurs:
    /// - If the exception is [`Exception::Exit`], the execution loop terminates
    ///   successfully and returns `Ok(())`.
    /// - If any other exception is encountered (such as `IllegalInstruction` or
    ///   `AccessViolation`), the function returns the corresponding error.
    ///
    /// # Returns
    /// - `Ok(())`: If the execution completes successfully or is explicitly
    ///   exited by an [`Exception::Exit`] exception.
    /// - `Err(Exception)`: If an error occurs during execution (e.g., invalid
    ///   instruction or memory access violation).
    pub fn execute(&mut self) -> Result<(), Exception> {
        loop {
            match self.single_step() {
                Ok(_) => {}
                Err(ex) => match ex {
                    Exception::Exit => return Ok(()),
                    _ => return Err(ex),
                },
            }
        }
    }
}

/// Represents the set of registers used by the virtual CPU.
#[repr(u8)]
#[derive(FromRepr, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    /// A 64-bit Instruction Pointer register.
    ///
    /// Holds the memory address of the instruction to be executed.
    IP,
    /// A 64-bit Flags register.
    ///
    /// Hold the value that each bit represents corresponding state. See
    /// [`RFlags`].
    RF,
    /// A 64-bit general purpose register.
    R0,
    /// A 64-bit general purpose register.
    R1,
    /// A 64-bit general purpose register.
    R2,
    /// A 64-bit general purpose register.
    R3,
    /// A 64-bit general purpose register.
    R4,
    /// A 64-bit general purpose register.
    R5,
    /// A 64-bit general purpose register.
    R6,
    /// A 64-bit general purpose register.
    R7,
    /// A 64-bit general purpose register.
    R8,
    /// A 64-bit general purpose register.
    R9,
    /// A 64-bit general purpose register.
    R10,
    /// A 64-bit general purpose register.
    R11,
    /// A 64-bit general purpose register.
    R12,
    /// A 64-bit general purpose register.
    R13,
    /// A 64-bit general purpose register.
    R14,
    /// A 64-bit general purpose register.
    R15,
}

impl Register {
    /// Reinterpret the [`Register`] from [`str`]
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<Self> {
        match &*s.as_ref().to_lowercase() {
            "ip" => Some(Self::IP),
            "rf" => Some(Self::RF),
            "r0" => Some(Self::R0),
            "r1" => Some(Self::R1),
            "r2" => Some(Self::R2),
            "r3" => Some(Self::R3),
            "r4" => Some(Self::R4),
            "r5" => Some(Self::R5),
            "r6" => Some(Self::R6),
            "r7" => Some(Self::R7),
            "r8" => Some(Self::R8),
            "r9" => Some(Self::R9),
            "r10" => Some(Self::R10),
            "r11" => Some(Self::R11),
            "r12" => Some(Self::R12),
            "r13" => Some(Self::R13),
            "r14" => Some(Self::R14),
            "r15" => Some(Self::R15),
            _ => None,
        }
    }
}

/// The number of [`Register`]s
pub const NUM_REGS: usize = 18;

impl fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IP => write!(f, "IP"),
            Self::RF => write!(f, "RF"),
            Self::R0 => write!(f, "R0"),
            Self::R1 => write!(f, "R1"),
            Self::R2 => write!(f, "R2"),
            Self::R3 => write!(f, "R3"),
            Self::R4 => write!(f, "R4"),
            Self::R5 => write!(f, "R5"),
            Self::R6 => write!(f, "R6"),
            Self::R7 => write!(f, "R7"),
            Self::R8 => write!(f, "R8"),
            Self::R9 => write!(f, "R9"),
            Self::R10 => write!(f, "R10"),
            Self::R11 => write!(f, "R11"),
            Self::R12 => write!(f, "R12"),
            Self::R13 => write!(f, "R13"),
            Self::R14 => write!(f, "R14"),
            Self::R15 => write!(f, "R15"),
        }
    }
}

/// Represents the set of [`Register`]s
#[derive(Debug, Clone, Copy, Default)]
pub struct Registers(pub [u64; NUM_REGS]);

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().enumerate().try_for_each(|(r, v)| {
            write!(f, "{}={v:016x} ", Register::from_repr(r as u8).unwrap(),)
        })
    }
}

impl Registers {
    /// Make an new [`Registers`] instance with default state
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets all registers to their default state (zero).
    pub fn reset(&mut self) {
        self.0.iter_mut().for_each(|r| *r = 0u64);
    }

    /// Reads the value from the specified register.
    pub fn read(&self, reg: Register) -> u64 {
        self.0[reg as usize]
    }

    /// Writes a value to the specified register.
    pub fn write(&mut self, reg: Register, value: u64) {
        self.0[reg as usize] = value;
    }

    /// Reads the value of the [`Register::RF`] and reinterprets it to a
    /// [`RFlags`] structure.
    pub fn read_rf(&self) -> RFlags {
        RFlags::new(self.read(Register::RF))
    }

    /// Writes the specified [`RFlags`] to the [`Register::RF`].
    pub fn write_rf(&mut self, rf: RFlags) {
        self.write(Register::RF, rf.0);
    }
}

/// A structure representing the flags stored in the [`Register::RF`].
///
/// This structure encapsulates the [`Register::RF`] register, which holds
/// various flags used in a CPU state, such as the carry flag, zero
/// flag, sign flag, overflow flag, and others. Each flag is represented by a
/// specific bit in the [`u64`] value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RFlags(pub u64);

impl RFlags {
    /// Make an new [`RFlags`] instance with zero
    #[must_use]
    pub fn new(value: u64) -> Self {
        Self { 0: value }
    }

    /// Reads the Carry Flag (CF)
    ///
    /// Indicates whether a carry occurred during an arithmetic operation (e.g.,
    /// addition or subtraction).
    pub fn read_cf(&self) -> u64 {
        (self.0 >> 0) & 1
    }

    /// Writes the specified Carry Flag (CF) value
    pub fn write_cf(&mut self, value: u64) {
        if value & 1 == 1 {
            self.0 |= 1 << 0;
        } else {
            self.0 &= !(1 << 0);
        }
    }

    /// Reads the Zero Flag (ZF)
    ///
    /// Indicates whether the result of the last operation was zero.
    pub fn read_zf(&self) -> u64 {
        (self.0 >> 6) & 1
    }

    /// Writes the specified Zero Flag (ZF) value
    pub fn write_zf(&mut self, value: u64) {
        if value & 1 == 1 {
            self.0 |= 1 << 6;
        } else {
            self.0 &= !(1 << 6);
        }
    }

    /// Reads the Sign Flag (SF)
    ///
    /// Indicates the sign of the result of the last operation (1 for negative,
    /// 0 for positive).
    pub fn read_sf(&self) -> u64 {
        (self.0 >> 7) & 1
    }

    /// Writes the specified Sign Flag (SF) value
    pub fn write_sf(&mut self, value: u64) {
        if value & 1 == 1 {
            self.0 |= 1 << 7;
        } else {
            self.0 &= !(1 << 7);
        }
    }

    /// Reads the Overflow Flag (OF)
    ///
    /// Indicates whether an overflow occurred in the last signed arithmetic
    /// operation.
    pub fn read_of(&self) -> u64 {
        (self.0 >> 11) & 1
    }

    /// Writes the specified Overflow Flag (OF) value
    pub fn write_of(&mut self, value: u64) {
        if value & 1 == 1 {
            self.0 |= 1 << 11;
        } else {
            self.0 &= !(1 << 11);
        }
    }

    /// Reads the Parity Flag (PF)
    ///
    /// Indicates whether the number of set bits in the result is even (0) or
    /// odd (1).
    pub fn read_pf(&self) -> u64 {
        (self.0 >> 2) & 1
    }

    /// Writes the specified Parity Flag (PF) value
    pub fn write_pf(&mut self, value: u64) {
        if value & 1 == 1 {
            self.0 |= 1 << 2;
        } else {
            self.0 &= !(1 << 2);
        }
    }

    /// Reads the Auxiliary Carry Flag (AF)
    ///
    /// Indicates whether a carry occurred between the lower and upper nibbles
    /// during an addition operation.
    pub fn read_af(&self) -> u64 {
        (self.0 >> 4) & 1
    }

    /// Writes the specified Auxiliary Carry Flag (AF) value
    pub fn write_af(&mut self, value: u64) {
        if value & 1 == 1 {
            self.0 |= 1 << 4;
        } else {
            self.0 &= !(1 << 4);
        }
    }
}
