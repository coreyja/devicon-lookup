use std::io::BufRead;

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
    type Item = Result<String, std::io::Error>;

    fn next(&mut self) -> Option<Result<String, std::io::Error>> {
        let mut buf = String::new();
        match self.buf.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with('\n') {
                    buf.pop();
                    if buf.ends_with('\r') {
                        buf.pop();
                    }
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e)),
        }
    }
}
