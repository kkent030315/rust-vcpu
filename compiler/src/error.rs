use core::result;
use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Compile error: {0}")]
    Compile(String),
}

pub type Result<T> = result::Result<T, Error>;
