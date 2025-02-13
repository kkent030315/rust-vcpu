use core::result;
use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}

pub type Result<T> = result::Result<T, Error>;
