use std::{iter::Peekable, ops::DerefMut, str::Chars};

use vm::isa::OperandSize;

/// Represents a semantic token
#[derive(Debug, Clone)]
pub enum Token {
    /// Indicates an arbitrary identifier
    Ident(String),
    /// Indicates a label definition
    Label(String),
    /// Indicates a size class
    SizeClass(OperandSize),
    /// Indicates an `offsetof` keyword
    OffsetOf,
    /// Indicates an integer or hexdecimal number literal
    Number(u64),
    /// Indicates a comma (`,`)
    Comma,
    /// Indicates a comment anchor (`;`)
    Comment,
    /// Indicates a left parenthesis (`[`)
    LParen,
    /// Indicates a right parenthesis (`]`)
    RParen,
    /// Indicates the EOF
    EOF,
    /// Indicates operators (e.g., `+`, `-`, `*`)
    Op(char),
}

/// Represents a lexer
#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: Box<Peekable<Chars<'a>>>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Make an new instance of [`Lexer`] with the input source
    #[must_use]
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input,
            chars: Box::new(input.chars().peekable()),
            pos: 0,
        }
    }

    /// Analyze the given source into a sequence of semantic tokens
    pub fn lex(&mut self) -> Result<Token, String> {
        let chars = self.chars.deref_mut();
        let src = self.input;

        let mut pos = self.pos;

        // Skip whitespaces
        loop {
            {
                let ch = chars.peek();

                if ch.is_none() {
                    self.pos = pos;

                    return Ok(Token::EOF);
                }

                if !ch.unwrap().is_whitespace() {
                    break;
                }
            }

            chars.next();
            pos += 1;
        }

        let start = pos;
        let next = chars.next();

        if next.is_none() {
            return Ok(Token::EOF);
        }

        pos += 1;

        let result = match next.unwrap() {
            '[' => Ok(Token::LParen),
            ']' => Ok(Token::RParen),
            ',' => Ok(Token::Comma),

            ';' => {
                // Comment
                loop {
                    let ch = chars.next();
                    pos += 1;

                    if ch == Some('\n') {
                        break;
                    }
                }

                Ok(Token::Comment)
            }

            // Parse number literal
            '0'..='9' => {
                let mut is_hex = false;

                loop {
                    let ch = match chars.peek() {
                        Some(ch) => *ch,
                        None => return Ok(Token::EOF),
                    };

                    if ch == 'h' || !ch.is_ascii_hexdigit() {
                        if ch == 'h' {
                            is_hex = true;
                        }
                        break;
                    }

                    chars.next();
                    pos += 1;
                }

                Ok(Token::Number(
                    if is_hex {
                        u64::from_str_radix(&src[start..pos].trim_end_matches("h"), 16)
                    } else {
                        src[start..pos].parse()
                    }
                    .map_err(|e| format!("{e}: {}", src[start..pos].to_string()))?,
                ))
            }

            'a'..='z' | 'A'..='Z' | '_' | ':' => {
                // Parse identifier
                loop {
                    let ch = match chars.peek() {
                        Some(ch) => *ch,
                        None => return Ok(Token::EOF),
                    };

                    if ch != '_' && ch != ':' && !ch.is_alphanumeric() {
                        break;
                    }

                    chars.next();
                    pos += 1;
                }

                match &src[start..pos] {
                    ident => {
                        if ident.ends_with(':') {
                            Ok(Token::Label(ident.to_string()))
                        } else {
                            if let Some(size) = OperandSize::from_str(ident) {
                                Ok(Token::SizeClass(size))
                            } else if ident == "offsetof" {
                                Ok(Token::OffsetOf)
                            } else {
                                Ok(Token::Ident(ident.to_string()))
                            }
                        }
                    }
                }
            }

            op => {
                // Parse operator
                Ok(Token::Op(op))
            }
        };

        self.pos = pos;
        result
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lex() {
            Ok(Token::EOF) | Err(_) => None,
            Ok(token) => Some(token),
        }
    }
}
