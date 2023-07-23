use std::{
    io::{BufRead, Read},
    string::{self, FromUtf8Error},
};

use thiserror::Error;

#[derive(Debug)]
pub struct MaybeUtf8Lines<B> {
    buf: B,
}

impl<B> MaybeUtf8Lines<B> {
    pub fn new(buf: B) -> Self {
        Self { buf }
    }
}

#[derive(Debug, Error)]
pub enum MaybeUtf8LinesError {
    #[error("Input is not valid UTF-8")]
    Utf8Error(#[from] FromUtf8Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

fn process_buf(buf: &mut Vec<u8>) -> Result<String, MaybeUtf8LinesError> {
    if buf.ends_with(&[b'\n']) {
        buf.pop();
        if buf.ends_with(&[b'\r']) {
            buf.pop();
        }
    }
    let string = string::String::from_utf8(buf.clone())?;

    Ok(string)
}

impl<B: BufRead> Iterator for MaybeUtf8Lines<B> {
    type Item = Result<String, MaybeUtf8LinesError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = vec![];

        match self.buf.read_until(b'\n', &mut buf) {
            Ok(0) => None,
            Ok(_n) => Some(process_buf(&mut buf)),
            Err(e) => Some(Err(e.into())),
        }
    }
}

pub(crate) trait IntoMaybeUt8Lines {
    fn into_maybe_utf8_lines(self) -> MaybeUtf8Lines<Self>
    where
        Self: Sized;
}

impl<Buf> IntoMaybeUt8Lines for Buf
where
    Buf: Read,
{
    fn into_maybe_utf8_lines(self) -> MaybeUtf8Lines<Self> {
        MaybeUtf8Lines::new(self)
    }
}
