use vm::{
    emulator::Register,
    isa::{Mnemonic, OperandSize},
};

use crate::lexer::{Lexer, Token};

/// Represents an expression
#[derive(Debug, Clone)]
pub enum Expr {
    /// Indicates a register operand
    RegisterOp(Register),
    /// Indicates a memory operand
    MemoryOp {
        size: OperandSize,
        displacement: u64,
        scale: u8,
        index_reg: Option<Register>,
        base_reg: Option<Register>,
    },
    /// Indicates a 64-bit immediate
    Immediate(u64),
    /// Indicates a label definition
    Label(String),
    /// Indicates a label reference (in an operand)
    LabelRef(String),
}

/// Represents an intermediate representation of [`vm::isa::Instruction`]
#[derive(Debug, Clone)]
pub struct LexInstruction {
    /// A mnemonic of this instruction
    pub mnemonic: Mnemonic,
    /// Operands of this instruction
    pub operands: Vec<Expr>,
}

/// Represents a parser
#[derive(Debug)]
pub struct Parser {
    /// Tokens analyzed by [`Lexer`]
    tokens: Vec<Token>,
}

impl<'a> Parser {
    /// Make an new instance of [`Parser`] with the input source
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.by_ref().collect();

        Parser { tokens }
    }

    /// Parses a label definition
    pub fn parse_label(&mut self) -> Result<Option<String>, String> {
        // Not interested
        if self.tokens.is_empty() {
            return Ok(None);
        }

        match &self.tokens[0] {
            Token::Label(id) => return Ok(Some(id.to_owned())),
            _ => return Ok(None),
        }
    }

    /// Parses an instruction other than a label definition
    pub fn parse(&mut self) -> Result<Option<LexInstruction>, String> {
        // Not interested
        if self.tokens.is_empty() || !matches!(self.tokens[0], Token::Ident(_)) {
            return Ok(None);
        }

        let mut operands = Vec::new();
        let mnemonic = match &self.tokens[0] {
            Token::Ident(id) => match Mnemonic::from_str(id) {
                Some(id) => Ok(id),
                None => Err(format!("Unrecognized mnemonic: {id}")),
            },
            _ => Err("Expected an identifier".into()),
        }?
        .to_owned();

        // instruction without operands
        if self.tokens.len() == 1 {
            return Ok(Some(LexInstruction { mnemonic, operands }));
        }

        let parse_op = |i| match &self.tokens[i] {
            Token::OffsetOf => match &self.tokens[i + 1] {
                Token::Ident(id) => Ok(Expr::LabelRef(id.to_owned())),
                _ => return Err("Expected an identifier after `offsetof`".into()),
            },
            Token::SizeClass(_) => {
                match &self.tokens[i + 1] {
                    Token::LParen => {}
                    _ => return Err("Expected `[` after size class".into()),
                };

                // mov r1, byte [r0]
                let size = match &self.tokens[i] {
                    Token::SizeClass(id) => Ok::<OperandSize, String>(*id),
                    _ => Err("Expected size class".into()),
                }?;

                // Collect to `size [LParen, .., RParen]`
                let tok = self.tokens[i..]
                    .iter()
                    .take_while(|x| !matches!(x, Token::RParen))
                    .chain(
                        self.tokens[i..]
                            .iter()
                            .skip_while(|x| !matches!(x, Token::RParen))
                            .take(1),
                    )
                    .collect::<Vec<_>>();

                if tok.is_empty() {
                    return Err(format!("Expected `]`"));
                }
                if tok.len() < 4 {
                    return Err(format!("Expected an identifier after `[`"));
                }

                // mov r1, [r0]
                let base_reg = match &tok[2] {
                    Token::Ident(id) => match Register::from_str(id) {
                        Some(id) => Ok(Some(id)),
                        _ => Err(format!("Unrecognized register: {id}")),
                    },
                    _ => Err("Expected identifier".into()),
                }?;
                // mov r3, [r0+r1]
                let index_reg = if tok.len() > 5 {
                    match &tok[4] {
                        Token::Ident(id) => match Register::from_str(id) {
                            Some(id) => Ok(Some(id)),
                            _ => Err(format!("Unrecognized register: {id}")),
                        },
                        _ => Err(format!("Expected register: {:?}", &tok[4])),
                    }?
                } else {
                    None
                };
                // mov r3, [r0+r1*2]
                let scale = if tok.len() > 7 {
                    match &tok[6] {
                        Token::Number(num) => match num {
                            1 | 2 | 4 | 8 => Ok(*num as u8),
                            _ => Err(format!("Unrecognized scale: {num}")),
                        },
                        _ => Err(format!(
                            "Expected scale number 1 | 2 | 4 | 8: {:?}",
                            &tok[6]
                        )),
                    }?
                } else {
                    1
                };

                Ok(Expr::MemoryOp {
                    size,
                    displacement: 0,
                    scale,
                    index_reg,
                    base_reg,
                })
            }
            Token::Ident(id) => match mnemonic {
                Mnemonic::Jmp => Ok(Expr::LabelRef(id.to_owned())),
                Mnemonic::Jz => Ok(Expr::LabelRef(id.to_owned())),
                Mnemonic::Jnz => Ok(Expr::LabelRef(id.to_owned())),
                Mnemonic::Jle => Ok(Expr::LabelRef(id.to_owned())),
                Mnemonic::Jg => Ok(Expr::LabelRef(id.to_owned())),
                Mnemonic::Jge => Ok(Expr::LabelRef(id.to_owned())),
                Mnemonic::Jb => Ok(Expr::LabelRef(id.to_owned())),
                _ => match Register::from_str(id) {
                    Some(id) => Ok(Expr::RegisterOp(id)),
                    _ => Err(format!("Unrecognized register: {id}")),
                },
            },
            Token::Label(id) => Ok(Expr::Label(id.to_owned())),
            Token::Number(num) => Ok(Expr::Immediate(*num)),
            x => Err(format!("Unexpected token: {x:?}")),
        };

        let op0 = parse_op(1)?;
        operands.push(op0);

        if let Some(pos) = self.tokens[1..]
            .iter()
            .position(|t| matches!(t, Token::Comma))
        {
            if pos + 2 > self.tokens.len() {
                return Err("Expected an operand after `,`".into());
            }

            let op1 = parse_op(pos + 2)?;
            operands.push(op1);
        }

        Ok(Some(LexInstruction { mnemonic, operands }))
    }
}
