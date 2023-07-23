use std::{
    io::BufRead,
    string::{self, FromUtf8Error},
};

#[derive(Debug)]
pub struct Lines<B> {
    buf: B,
}

impl<B> Lines<B> {
    pub fn new(buf: B) -> Self {
        Self { buf }
    }
}

impl<B: BufRead> Iterator for Lines<B> {
    type Item = Result<Result<String, FromUtf8Error>, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = vec![];

        match self.buf.read_until(b'\n', &mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with(&[b'\n']) {
                    buf.pop();
                    if buf.ends_with(&[b'\r']) {
                        buf.pop();
                    }
                }
                let string = string::String::from_utf8(buf.clone());
                Some(Ok(string))
            }
            Err(e) => Some(Err(e)),
        }
    }
}
