use thiserror::Error;
use std::num::ParseIntError;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("the thread count must be a integer")]
    ThreadCountParse(#[from] ParseIntError),

    #[error("failed to open file")]
    FileOpen(std::io::Error),

    #[error("failed to create file")]
    FileCreate(std::io::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
