use std::{io, result, str};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unsupprted cmd/arg now {name:?}")]
    Unsupprted { name: String },
    #[error("null ptr returned by {from:?}, args: {args:?}")]
    Nullptr { from: String, args: String },
    #[error(transparent)]
    StrError(#[from] str::Utf8Error),
    #[error(transparent)]
    IOError(#[from] io::Error),
}
pub type Result<T> = result::Result<T, Error>;
