//! This module implements exceptions.
//!
//! Exceptions are unexpected or exceptional events that occur during program
//! execution. These may be triggered by invalid instructions, memory access
//! violations, or explicit program exits. Exception handling allows the system
//! to gracefully manage such events and take appropriate recovery actions.
//!
//! ## Exception Types
//! - [`Exception::Exit`]: Indicates a normal program termination.
//! - [`Exception::IllegalInstruction`]: Raised when an invalid or unsupported
//!   instruction is encountered.
//! - [`Exception::AccessViolation`]: Triggered when an attempt is made to
//!   access restricted or invalid memory.

use core::fmt;

/// Represents the different types of exceptions that can occur during
/// execution.
#[derive(Debug, Clone)]
pub enum Exception {
    /// Indicates the program or execution should terminate without an error
    Exit,
    /// Indicates that an invalid or unsupported instruction encountered during
    /// instruction decoding
    IllegalInstruction,
    /// Indicates a violation of memory access, such as accessing out-of-bounds
    /// memory
    AccessViolation,
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exit => write!(f, "Exit"),
            Self::IllegalInstruction => write!(f, "IllegalInstruction"),
            Self::AccessViolation => write!(f, "AccessViolation"),
        }
    }
}
