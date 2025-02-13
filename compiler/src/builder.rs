use std::collections::HashMap;

use log::{error, trace};
use vm::isa::{Instruction, Mnemonic, OpCode, Operand};

use crate::parser::{Expr, LexInstruction, Parser};

/// Represents a compile state
#[derive(Debug, Clone)]
pub enum CompileState {
    /// The object is compiled
    Compiled {
        offset: usize,
        lexi: LexInstruction,
        instruction: Instruction,
        buf: Vec<u8>,
    },
    /// The object is pending offset resolution
    UnresolvedOffsetOf {
        offset: usize,
        label: String,
        lexi: LexInstruction,
        instruction: Instruction,
        buf: Vec<u8>,
    },
    /// The object is pending label resolution
    UnresolvedLabel {
        offset: usize,
        label: String,
        lexi: LexInstruction,
        instruction: Instruction,
    },
}

/// Represents a builder for a input source
#[derive(Debug, Default)]
pub struct Builder {
    /// Compile state for each instructions
    pub state: Vec<CompileState>,
    /// Label definitions
    pub labels: HashMap<String, u64>,
    /// Current position in bytes
    pub cursor: usize,
}

impl Builder {
    /// Make an new [`Builder`] instance
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Finally dump the VM bytecode.
    pub fn dump(&mut self) -> Result<Vec<u8>, String> {
        let mut vec = Vec::new();

        for state in &self.state {
            match state {
                CompileState::Compiled {
                    offset: _,
                    lexi: _,
                    instruction: _,
                    buf,
                } => {
                    vec.extend(buf);
                }
                x => return Err(format!("Unresolved compile state: {x:?}")),
            }
        }

        Ok(vec)
    }

    /// Resolves pending offset or label resolution
    pub fn finalize(&mut self) -> Result<(), String> {
        let mut final_state = Vec::<CompileState>::new();

        for state in &mut self.state {
            match state {
                CompileState::Compiled {
                    offset,
                    lexi,
                    instruction: _,
                    buf,
                } => {
                    trace!(
                        "{offset}: {} => ({}) {:02x?}",
                        lexi.mnemonic,
                        buf.len(),
                        buf
                    );

                    final_state.push(state.to_owned());
                }
                CompileState::UnresolvedOffsetOf {
                    offset,
                    label,
                    lexi,
                    instruction,
                    buf: _,
                } => match self.labels.get(label) {
                    Some(label_loc) => {
                        let mut buf = Vec::new();

                        trace!(
                            "{offset}: {} => ({}) {:02x?}",
                            lexi.mnemonic,
                            buf.len(),
                            buf
                        );

                        instruction.set_immediate(*label_loc);
                        instruction.encode(&mut buf).unwrap();

                        final_state.push(CompileState::Compiled {
                            offset: *offset,
                            lexi: lexi.to_owned(),
                            instruction: instruction.to_owned(),
                            buf,
                        });
                    }
                    None => return Err(format!("Unresolved label: {label}")),
                },
                CompileState::UnresolvedLabel {
                    offset,
                    label,
                    lexi,
                    instruction,
                } => match self.labels.get(label) {
                    Some(label_loc) => {
                        let mut buf = Vec::new();
                        let offset = (*offset) as u64;

                        instruction.encode(&mut buf).unwrap();
                        let target = *label_loc as i64 - offset as i64 - buf.len() as i64;
                        buf.clear();

                        instruction.set_branch_target(target.into());
                        instruction.encode(&mut buf).unwrap();

                        trace!(
                            "{offset}: {} => ({}) {:02x?}",
                            lexi.mnemonic,
                            buf.len(),
                            buf
                        );

                        final_state.push(CompileState::Compiled {
                            offset: offset as usize,
                            lexi: lexi.to_owned(),
                            instruction: instruction.to_owned(),
                            buf,
                        });
                    }
                    None => return Err(format!("Unresolved label: {label}")),
                },
            }
        }

        self.state = final_state;

        Ok(())
    }

    /// Compile an intermediate [`LexInstruction`] into a compilation state
    pub fn compile_instruction(&mut self, lexi: &LexInstruction) -> Result<(), String> {
        let op = &lexi.operands;
        let mnemonic = lexi.mnemonic;

        if op.len() < mnemonic.min_operands() {
            return Err(format!("Too few operands ({}) for {mnemonic}", op.len()));
        }
        if op.len() > mnemonic.max_operands() {
            return Err(format!("Too many operands ({}) for {mnemonic}", op.len()));
        }

        let mut insn = Instruction::new();
        let mut buf = Vec::<u8>::new();

        match mnemonic {
            Mnemonic::Exit | Mnemonic::Ud => {
                match mnemonic {
                    Mnemonic::Exit => insn.set_opcode(OpCode::Exit),
                    Mnemonic::Ud => insn.set_opcode(OpCode::Ud),
                    _ => unreachable!(),
                };

                insn.encode(&mut buf).map_err(|e| e.to_string())?;
                self.state.push(CompileState::Compiled {
                    offset: self.cursor,
                    lexi: lexi.to_owned(),
                    instruction: insn.to_owned(),
                    buf: buf.clone(),
                });
            }
            Mnemonic::Mov
            | Mnemonic::Add
            | Mnemonic::Sub
            | Mnemonic::And
            | Mnemonic::Or
            | Mnemonic::Xor
            | Mnemonic::Xchg
            | Mnemonic::Imul => {
                let mut offsetof = None;

                match &op[0] {
                    Expr::RegisterOp(reg) => {
                        insn.set_op0_reg(*reg);
                    }
                    Expr::MemoryOp {
                        size,
                        displacement,
                        scale,
                        index_reg,
                        base_reg,
                    } => {
                        insn.set_op0_mem(Operand::Memory {
                            size: *size,
                            displacement: *displacement,
                            scale: *scale,
                            index_reg: *index_reg,
                            base_reg: *base_reg,
                        });
                    }
                    Expr::Immediate(_) => {
                        return Err("Unexpected immediate at first operand".into())
                    }
                    x => return Err(format!("Unexpected operand: {x:?}")),
                };
                match &op[1] {
                    Expr::RegisterOp(reg) => {
                        insn.set_op1_reg(*reg);
                    }
                    Expr::MemoryOp {
                        size,
                        displacement,
                        scale,
                        index_reg,
                        base_reg,
                    } => {
                        insn.set_op1_mem(Operand::Memory {
                            size: *size,
                            displacement: *displacement,
                            scale: *scale,
                            index_reg: *index_reg,
                            base_reg: *base_reg,
                        });
                    }
                    Expr::Immediate(imm) => {
                        insn.set_immediate(*imm);
                    }
                    Expr::LabelRef(id) => {
                        insn.set_immediate(0);
                        offsetof = Some(id);
                    }
                    x => return Err(format!("Unexpected operand: {x:?}")),
                };

                match (&op[0], &op[1]) {
                    (Expr::RegisterOp(_), Expr::RegisterOp(_)) => match mnemonic {
                        Mnemonic::Mov => insn.set_opcode(OpCode::MovRR),
                        Mnemonic::Add => insn.set_opcode(OpCode::AddRR),
                        Mnemonic::Sub => insn.set_opcode(OpCode::SubRR),
                        Mnemonic::And => insn.set_opcode(OpCode::AndRR),
                        Mnemonic::Or => insn.set_opcode(OpCode::OrRR),
                        Mnemonic::Xor => insn.set_opcode(OpCode::XorRR),
                        Mnemonic::Xchg => insn.set_opcode(OpCode::XchgRR),
                        Mnemonic::Imul => insn.set_opcode(OpCode::ImulRR),
                        _ => unreachable!(),
                    },
                    (Expr::RegisterOp(_), Expr::Immediate(_))
                    | (Expr::RegisterOp(_), Expr::LabelRef(_)) => match mnemonic {
                        Mnemonic::Mov => insn.set_opcode(OpCode::MovRIMM),
                        Mnemonic::Add => insn.set_opcode(OpCode::AddRIMM),
                        Mnemonic::Sub => insn.set_opcode(OpCode::SubRIMM),
                        Mnemonic::And => insn.set_opcode(OpCode::AndRIMM),
                        Mnemonic::Or => insn.set_opcode(OpCode::OrRIMM),
                        Mnemonic::Xor => insn.set_opcode(OpCode::XorRIMM),
                        Mnemonic::Xchg => unreachable!(),
                        Mnemonic::Imul => insn.set_opcode(OpCode::ImulRIMM),
                        _ => unreachable!(),
                    },
                    (
                        Expr::RegisterOp(_),
                        Expr::MemoryOp {
                            size: _,
                            displacement: _,
                            scale: _,
                            index_reg: _,
                            base_reg: _,
                        },
                    ) => match mnemonic {
                        Mnemonic::Mov => insn.set_opcode(OpCode::MovRRM),
                        _ => unimplemented!(),
                    },
                    (
                        Expr::MemoryOp {
                            size: _,
                            displacement: _,
                            scale: _,
                            index_reg: _,
                            base_reg: _,
                        },
                        Expr::RegisterOp(_),
                    ) => match mnemonic {
                        Mnemonic::Mov => insn.set_opcode(OpCode::MovRMR),
                        _ => unimplemented!(),
                    },
                    _ => todo!(),
                }

                insn.encode(&mut buf).map_err(|e| e.to_string())?;
                if let Some(label) = offsetof {
                    self.state.push(CompileState::UnresolvedOffsetOf {
                        offset: self.cursor,
                        label: label.to_owned(),
                        lexi: lexi.to_owned(),
                        instruction: insn.to_owned(),
                        buf: buf.clone(),
                    });
                } else {
                    self.state.push(CompileState::Compiled {
                        offset: self.cursor,
                        lexi: lexi.to_owned(),
                        instruction: insn.to_owned(),
                        buf: buf.clone(),
                    });
                }
            }
            Mnemonic::Inc | Mnemonic::Dec => {
                match &op[0] {
                    Expr::RegisterOp(reg) => {
                        insn.set_op0_reg(*reg);

                        match mnemonic {
                            Mnemonic::Inc => insn.set_opcode(OpCode::IncR),
                            Mnemonic::Dec => insn.set_opcode(OpCode::DecR),
                            _ => unreachable!(),
                        }
                    }
                    Expr::MemoryOp {
                        size,
                        displacement,
                        scale,
                        index_reg,
                        base_reg,
                    } => {
                        insn.set_op0_mem(Operand::Memory {
                            size: *size,
                            displacement: *displacement,
                            scale: *scale,
                            index_reg: *index_reg,
                            base_reg: *base_reg,
                        });
                    }
                    Expr::Immediate(_) => {
                        return Err("Unexpected immediate at first operand".into())
                    }
                    x => return Err(format!("Unexpected operand: {x:?}")),
                };

                insn.encode(&mut buf).map_err(|e| e.to_string())?;
                self.state.push(CompileState::Compiled {
                    offset: self.cursor,
                    lexi: lexi.to_owned(),
                    instruction: insn.to_owned(),
                    buf: buf.clone(),
                });
            }
            Mnemonic::Test | Mnemonic::Cmp => {
                match &op[0] {
                    Expr::RegisterOp(reg) => {
                        insn.set_op0_reg(*reg);
                    }
                    Expr::MemoryOp {
                        size: _,
                        displacement: _,
                        scale: _,
                        index_reg: _,
                        base_reg: _,
                    } => todo!(),
                    x => return Err(format!("Unexpected operand: {x:?}")),
                };
                match &op[1] {
                    Expr::RegisterOp(reg) => {
                        insn.set_op1_reg(*reg);
                    }
                    Expr::MemoryOp {
                        size: _,
                        displacement: _,
                        scale: _,
                        index_reg: _,
                        base_reg: _,
                    } => todo!(),
                    Expr::Immediate(imm) => {
                        insn.set_immediate(*imm);
                    }
                    x => return Err(format!("Unexpected operand: {x:?}")),
                };

                match (&op[0], &op[1]) {
                    (Expr::RegisterOp(_), Expr::RegisterOp(_)) => match mnemonic {
                        Mnemonic::Test => insn.set_opcode(OpCode::TestRR),
                        Mnemonic::Cmp => insn.set_opcode(OpCode::CmpRR),
                        _ => unreachable!(),
                    },
                    (Expr::RegisterOp(_), Expr::Immediate(_)) => match mnemonic {
                        Mnemonic::Test => insn.set_opcode(OpCode::TestRIMM),
                        Mnemonic::Cmp => insn.set_opcode(OpCode::CmpRIMM),
                        _ => unreachable!(),
                    },
                    _ => todo!(),
                }

                insn.encode(&mut buf).map_err(|e| e.to_string())?;
                self.state.push(CompileState::Compiled {
                    offset: self.cursor,
                    lexi: lexi.to_owned(),
                    instruction: insn.to_owned(),
                    buf: buf.clone(),
                });
            }
            Mnemonic::Jmp
            | Mnemonic::Jz
            | Mnemonic::Jnz
            | Mnemonic::Jle
            | Mnemonic::Jg
            | Mnemonic::Jge
            | Mnemonic::Jb => {
                match mnemonic {
                    Mnemonic::Jmp => insn.set_opcode(OpCode::Jmp),
                    Mnemonic::Jz => insn.set_opcode(OpCode::Jz),
                    Mnemonic::Jnz => insn.set_opcode(OpCode::Jnz),
                    Mnemonic::Jle => insn.set_opcode(OpCode::Jle),
                    Mnemonic::Jg => insn.set_opcode(OpCode::Jg),
                    Mnemonic::Jge => insn.set_opcode(OpCode::Jge),
                    Mnemonic::Jb => insn.set_opcode(OpCode::Jge),
                    _ => unreachable!(),
                }

                let label;
                match &op[0] {
                    Expr::LabelRef(id) => {
                        label = id.to_owned();
                    }
                    x => return Err(format!("Unexpected operand: {x:?}")),
                };

                const DUMMY: i64 = 0;
                insn.set_branch_target(DUMMY);
                insn.encode(&mut buf).map_err(|e| e.to_string())?;
                self.state.push(CompileState::UnresolvedLabel {
                    offset: self.cursor,
                    label,
                    lexi: lexi.to_owned(),
                    instruction: insn.to_owned(),
                });
            }
            Mnemonic::Db | Mnemonic::Dw | Mnemonic::Dd | Mnemonic::Dq => {
                match &op[0] {
                    Expr::Immediate(imm) => match mnemonic {
                        Mnemonic::Db => {
                            if *imm > u8::MAX as u64 {
                                return Err(format!("Db overflows: {imm}"));
                            }
                            buf.extend_from_slice(&(*imm as u8).to_le_bytes().to_vec());
                            Ok::<(), String>(())
                        }
                        Mnemonic::Dw => {
                            if *imm > u16::MAX as u64 {
                                return Err(format!("Dw overflows: {imm}"));
                            }
                            buf.extend_from_slice(&(*imm as u16).to_le_bytes().to_vec());
                            Ok(())
                        }
                        Mnemonic::Dd => {
                            if *imm > u32::MAX as u64 {
                                return Err(format!("Dd overflows: {imm}"));
                            }
                            buf.extend_from_slice(&(*imm as u32).to_le_bytes().to_vec());
                            Ok(())
                        }
                        Mnemonic::Dq => {
                            if *imm > u64::MAX as u64 {
                                return Err(format!("Dq overflows: {imm}"));
                            }
                            buf.extend_from_slice(&(*imm as u64).to_le_bytes().to_vec());
                            Ok(())
                        }
                        _ => unreachable!(),
                    },
                    x => return Err(format!("Unexpected token: {x:?}")),
                }?;

                self.state.push(CompileState::Compiled {
                    offset: self.cursor,
                    lexi: lexi.to_owned(),
                    instruction: insn.to_owned(),
                    buf: buf.clone(),
                });
            }
        };

        assert_ne!(buf.len(), 0);
        self.cursor += buf.len();

        Ok(())
    }
}

/// Compile a whole source
pub fn build_bytecode_s<I: AsRef<str>>(
    input: I,
    builder: &mut Builder,
) -> crate::error::Result<()> {
    input
        .as_ref()
        .split('\n')
        .try_for_each(|l| build_bytecode(format!("{l}\n"), builder))
}

/// Compile single line of source
pub fn build_bytecode<I: AsRef<str>>(input: I, builder: &mut Builder) -> crate::error::Result<()> {
    let mut parser = Parser::new(input.as_ref());

    match parser.parse_label() {
        Ok(Some(label)) => {
            builder.labels.insert(
                label.trim_end_matches(':').to_string(),
                builder.cursor as u64,
            );
            return Ok(());
        }
        Ok(None) => {}
        Err(err) => {
            error!("Error parsing label: {}", err);
            return Err(crate::error::Error::Compile(err));
        }
    }

    match parser.parse() {
        Ok(Some(insn)) => {
            log::info!("-> Expression parsed: {}", insn.mnemonic);
            for op in &insn.operands {
                log::info!("  op: {op:?}");
            }
            match builder.compile_instruction(&insn) {
                Ok(_) => {}
                Err(err) => return Err(crate::error::Error::Compile(err)),
            };
        }
        Ok(None) => {}
        Err(err) => {
            error!("Error parsing expression: {}", err);
            return Err(crate::error::Error::Compile(err));
        }
    };

    Ok(())
}
