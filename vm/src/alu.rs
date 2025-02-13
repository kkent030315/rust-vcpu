//! This module implements sequential Arithmetic Logic Unit (ALU) in software.
//!
//! A hardware ALU generally performs multiple arithmetic and logical operations
//! in parallel, such as addition, subtraction, bitwise operations, and
//! comparisons. In hardware, ALUs are fundamental components of CPUs,
//! responsible for executing instructions efficiently.
//!
//! This software-based ALU simulates these operations sequentially. Those
//! handlers are not thread-safe and may cause data races if accessed
//! concurrently.

use crate::{
    emulator::{Emulator, Register},
    exception::Exception,
    isa::{Instruction, Operand, OperandSize},
};

macro_rules! define_handler_trait {
    ($trait_name:ident, $fn_name:ident) => {
        pub trait $trait_name {
            fn $fn_name(&mut self, insn: &Instruction) -> Result<(), Exception>;
        }
    };
}

fn filter_special_reg(reg: Register) -> Option<Register> {
    (!matches!(reg, Register::IP)).then_some(reg)
}

fn handle_memop_read(
    emulator: &mut Emulator,
    op: usize,
    insn: &Instruction,
) -> Result<u64, Exception> {
    match insn.operands[op] {
        Operand::Memory {
            size,
            displacement,
            scale,
            index_reg,
            base_reg,
        } => {
            let mut address = displacement;
            if let Some(base_reg) = base_reg {
                address += emulator.regs.read(base_reg);
            }
            if let Some(index_reg) = index_reg {
                if !matches!(scale, 1 | 2 | 4 | 8) {
                    return Err(Exception::IllegalInstruction);
                }
                address += emulator.regs.read(index_reg) * (scale as u64);
            }

            Ok(match size {
                OperandSize::Byte => emulator.dram.read_u8(address as usize)? as u64,
                OperandSize::Word => emulator.dram.read_u16le(address as usize)? as u64,
                OperandSize::DWord => emulator.dram.read_u32le(address as usize)? as u64,
                OperandSize::QWord => emulator.dram.read_u64le(address as usize)?,
            })
        }
        _ => return Err(Exception::IllegalInstruction),
    }
}

fn handle_memop_write(
    emulator: &mut Emulator,
    op: usize,
    insn: &Instruction,
    value: u64,
) -> Result<(), Exception> {
    match insn.operands[op] {
        Operand::Memory {
            size,
            displacement,
            scale,
            index_reg,
            base_reg,
        } => {
            let mut address = displacement;
            if let Some(base_reg) = base_reg {
                address += emulator.regs.read(base_reg);
            }
            if let Some(index_reg) = index_reg {
                if !matches!(scale, 1 | 2 | 4 | 8) {
                    return Err(Exception::IllegalInstruction);
                }
                address += emulator.regs.read(index_reg) * (scale as u64);
            }

            Ok(match size {
                OperandSize::Byte => emulator.dram.write_u8(address as usize, value as u8)?,
                OperandSize::Word => emulator.dram.write_u16le(address as usize, value as u16)?,
                OperandSize::DWord => emulator.dram.write_u32le(address as usize, value as u32)?,
                OperandSize::QWord => emulator.dram.write_u64le(address as usize, value)?,
            })
        }
        _ => return Err(Exception::IllegalInstruction),
    }
}

// Memory operators
define_handler_trait!(MovRIMM, handle_mov_r_imm);
define_handler_trait!(MovRR, handle_mov_r_r);
define_handler_trait!(MovRRM, handle_mov_r_rm);
define_handler_trait!(MovRMR, handle_mov_rm_r);

// Binary operators
define_handler_trait!(AddRIMM, handle_add_r_imm);
define_handler_trait!(AddRR, handle_add_r_r);
define_handler_trait!(SubRIMM, handle_sub_r_imm);
define_handler_trait!(SubRR, handle_sub_r_r);
define_handler_trait!(AndRIMM, handle_and_r_imm);
define_handler_trait!(AndRR, handle_and_r_r);
define_handler_trait!(OrRIMM, handle_or_r_imm);
define_handler_trait!(OrRR, handle_or_r_r);
define_handler_trait!(XorRIMM, handle_xor_r_imm);
define_handler_trait!(XorRR, handle_xor_r_r);
define_handler_trait!(XchgRR, handle_xchg_r_r);
define_handler_trait!(ImulRIMM, handle_imul_r_imm);
define_handler_trait!(ImulRR, handle_imul_r_r);

// Unary operators
define_handler_trait!(IncR, handle_inc_r);
define_handler_trait!(DecR, handle_dec_r);

// Branch operators
define_handler_trait!(Jmp, handle_jmp);
define_handler_trait!(Jz, handle_jz);
define_handler_trait!(Jnz, handle_jnz);
define_handler_trait!(Jle, handle_jle);
define_handler_trait!(Jg, handle_jg);
define_handler_trait!(Jge, handle_jge);
define_handler_trait!(Jb, handle_jb);

// Comparison operators
define_handler_trait!(TestRIMM, handle_test_r_imm);
define_handler_trait!(TestRR, handle_test_r_r);
define_handler_trait!(CmpRIMM, handle_cmp_r_imm);
define_handler_trait!(CmpRR, handle_cmp_r_r);

impl MovRIMM for Emulator {
    fn handle_mov_r_imm(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let imm = insn.immediate();

        self.regs.write(r, imm);

        Ok(())
    }
}

impl MovRR for Emulator {
    fn handle_mov_r_r(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let op1_r = insn.op1_reg();

        self.regs.write(op0_r, self.regs.read(op1_r));

        Ok(())
    }
}

impl MovRRM for Emulator {
    fn handle_mov_r_rm(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;

        let value = handle_memop_read(self, 1, insn)?;
        self.regs.write(op0_r, value);

        Ok(())
    }
}

impl MovRMR for Emulator {
    fn handle_mov_rm_r(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op1_r = Some(insn.op1_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let value = self.regs.read(op1_r);

        handle_memop_write(self, 0, insn, value)?;

        Ok(())
    }
}

impl AddRIMM for Emulator {
    fn handle_add_r_imm(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let imm = insn.immediate();

        let lhs = self.regs.read(r);
        let rhs = imm;
        self.regs.write(r, lhs.wrapping_add(rhs));

        Ok(())
    }
}

impl AddRR for Emulator {
    fn handle_add_r_r(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let op1_r = insn.op1_reg();

        let lhs = self.regs.read(op0_r);
        let rhs = self.regs.read(op1_r);
        self.regs.write(op0_r, lhs.wrapping_add(rhs));

        Ok(())
    }
}

impl SubRIMM for Emulator {
    fn handle_sub_r_imm(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let imm = insn.immediate();

        let lhs = self.regs.read(r);
        let rhs = imm;
        self.regs.write(r, lhs.wrapping_sub(rhs));

        Ok(())
    }
}

impl SubRR for Emulator {
    fn handle_sub_r_r(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let op1_r = insn.op1_reg();

        let lhs = self.regs.read(op0_r);
        let rhs = self.regs.read(op1_r);
        self.regs.write(op0_r, lhs.wrapping_sub(rhs));

        Ok(())
    }
}

impl AndRIMM for Emulator {
    fn handle_and_r_imm(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let imm = insn.immediate();

        let lhs = self.regs.read(r);
        let rhs = imm;
        self.regs.write(r, lhs & rhs);

        // println!("AND/R/IMM {lhs} & {rhs} = {}", lhs & rhs);

        Ok(())
    }
}

impl AndRR for Emulator {
    fn handle_and_r_r(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let op1_r = insn.op1_reg();

        let lhs = self.regs.read(op0_r);
        let rhs = self.regs.read(op1_r);
        self.regs.write(op0_r, lhs & rhs);

        Ok(())
    }
}

impl OrRIMM for Emulator {
    fn handle_or_r_imm(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let imm = insn.immediate();

        let lhs = self.regs.read(r);
        let rhs = imm;
        self.regs.write(r, lhs | rhs);

        Ok(())
    }
}

impl OrRR for Emulator {
    fn handle_or_r_r(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let op1_r = insn.op1_reg();

        let lhs = self.regs.read(op0_r);
        let rhs = self.regs.read(op1_r);
        self.regs.write(op0_r, lhs | rhs);

        Ok(())
    }
}

impl XorRIMM for Emulator {
    fn handle_xor_r_imm(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let imm = insn.immediate();

        let lhs = self.regs.read(r);
        let rhs = imm;
        self.regs.write(r, lhs ^ rhs);

        Ok(())
    }
}

impl XorRR for Emulator {
    fn handle_xor_r_r(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let op1_r = insn.op1_reg();

        let lhs = self.regs.read(op0_r);
        let rhs = self.regs.read(op1_r);
        self.regs.write(op0_r, lhs ^ rhs);

        Ok(())
    }
}

impl XchgRR for Emulator {
    fn handle_xchg_r_r(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let op1_r = insn.op1_reg();

        let lhs = self.regs.read(op0_r);
        let rhs = self.regs.read(op1_r);
        self.regs.write(op0_r, rhs);
        self.regs.write(op1_r, lhs);

        Ok(())
    }
}

impl ImulRIMM for Emulator {
    fn handle_imul_r_imm(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let imm = insn.immediate();

        let lhs = self.regs.read(r) as i64;
        let rhs = imm as i64;
        self.regs.write(r, lhs.wrapping_mul(rhs) as u64);

        Ok(())
    }
}

impl ImulRR for Emulator {
    fn handle_imul_r_r(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let op1_r = insn.op1_reg();

        let lhs = self.regs.read(op0_r) as i64;
        let rhs = self.regs.read(op1_r) as i64;
        self.regs.write(op0_r, lhs.wrapping_mul(rhs) as u64);

        Ok(())
    }
}

impl IncR for Emulator {
    fn handle_inc_r(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;

        let value = self.regs.read(op0_r).wrapping_add(1u64);
        self.regs.write(op0_r, value);

        Ok(())
    }
}

impl DecR for Emulator {
    fn handle_dec_r(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;

        let value = self.regs.read(op0_r).wrapping_sub(1u64);
        self.regs.write(op0_r, value);

        Ok(())
    }
}

impl TestRIMM for Emulator {
    fn handle_test_r_imm(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;

        let lhs = self.regs.read(op0_r);
        let rhs = insn.immediate();
        let value = lhs & rhs;

        let mut rf = self.regs.read_rf();
        rf.write_zf((value == 0u64).into());
        self.regs.write_rf(rf);

        Ok(())
    }
}

impl TestRR for Emulator {
    fn handle_test_r_r(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let op1_r = insn.op1_reg();

        let lhs = self.regs.read(op0_r);
        let rhs = self.regs.read(op1_r);
        let value = lhs & rhs;

        let mut rf = self.regs.read_rf();
        rf.write_zf((value == 0u64).into());
        self.regs.write_rf(rf);

        Ok(())
    }
}

impl CmpRIMM for Emulator {
    fn handle_cmp_r_imm(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;

        let lhs = self.regs.read(op0_r) as i64;
        let rhs = insn.immediate() as i64;
        let value = lhs.wrapping_sub(rhs);

        let mut rf = self.regs.read_rf();
        rf.write_zf((value == 0).into());
        rf.write_sf(((value as u64) >> 63).into());

        let lhs_sign = (lhs >> 63) & 1;
        let rhs_sign = (rhs >> 63) & 1;
        let res_sign = (value >> 63) & 1;
        let of = (lhs_sign != rhs_sign) && (lhs_sign != res_sign);
        rf.write_of(of.into());

        self.regs.write_rf(rf);
        Ok(())
    }
}

impl CmpRR for Emulator {
    fn handle_cmp_r_r(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let op0_r = Some(insn.op0_reg())
            .and_then(filter_special_reg)
            .ok_or(Exception::IllegalInstruction)?;
        let op1_r = insn.op1_reg();

        let lhs = self.regs.read(op0_r) as i64;
        let rhs = self.regs.read(op1_r) as i64;
        let value = lhs.wrapping_sub(rhs);

        let mut rf = self.regs.read_rf();
        rf.write_zf((value == 0).into());
        rf.write_sf(((value as u64) >> 63).into());

        let lhs_sign = (lhs >> 63) & 1;
        let rhs_sign = (rhs >> 63) & 1;
        let res_sign = (value >> 63) & 1;
        let of = (lhs_sign != rhs_sign) && (lhs_sign != res_sign);
        rf.write_of(of.into());

        self.regs.write_rf(rf);
        Ok(())
    }
}

impl Jmp for Emulator {
    fn handle_jmp(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let target = insn.branch_target();
        if target == 0 {
            return Err(Exception::IllegalInstruction);
        }

        let value = (self.ip() as i64) + target;
        self.set_ip(value as u64);

        Ok(())
    }
}

impl Jz for Emulator {
    fn handle_jz(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let target = insn.branch_target();
        if target == 0 {
            return Err(Exception::IllegalInstruction);
        }

        let value = (self.ip() as i64) + target;
        let rf = self.regs.read_rf();

        if rf.read_zf() & 1 == 1 {
            self.set_ip(value as u64);
        }

        Ok(())
    }
}

impl Jnz for Emulator {
    fn handle_jnz(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let target = insn.branch_target();
        if target == 0 {
            return Err(Exception::IllegalInstruction);
        }

        let value = (self.ip() as i64) + target;
        let rf = self.regs.read_rf();

        if rf.read_zf() & 1 == 0 {
            self.set_ip(value as u64);
        }

        Ok(())
    }
}

impl Jle for Emulator {
    fn handle_jle(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let target = insn.branch_target();
        if target == 0 {
            return Err(Exception::IllegalInstruction);
        }

        let value = (self.ip() as i64) + target;
        let rf = self.regs.read_rf();

        if rf.read_zf() & 1 == 1 || rf.read_sf() != rf.read_of() {
            self.set_ip(value as u64);
        }

        Ok(())
    }
}

impl Jg for Emulator {
    fn handle_jg(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let target = insn.branch_target();
        if target == 0 {
            return Err(Exception::IllegalInstruction);
        }

        let value = (self.ip() as i64) + target;
        let rf = self.regs.read_rf();

        if rf.read_zf() == 0 && rf.read_sf() == rf.read_of() {
            self.set_ip(value as u64);
        }

        Ok(())
    }
}

impl Jge for Emulator {
    fn handle_jge(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let target = insn.branch_target();
        if target == 0 {
            return Err(Exception::IllegalInstruction);
        }

        let value = (self.ip() as i64) + target;
        let rf = self.regs.read_rf();

        if rf.read_sf() == rf.read_of() {
            self.set_ip(value as u64);
        }

        Ok(())
    }
}

impl Jb for Emulator {
    fn handle_jb(&mut self, insn: &Instruction) -> Result<(), Exception> {
        let target = insn.branch_target();
        if target == 0 {
            return Err(Exception::IllegalInstruction);
        }

        let value = (self.ip() as i64) + target;
        let rf = self.regs.read_rf();

        if rf.read_cf() & 1 == 1 {
            self.set_ip(value as u64);
        }

        Ok(())
    }
}
