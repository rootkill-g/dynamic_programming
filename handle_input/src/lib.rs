// Using the standard library's io and str modules
use std::{io::prelude::*, str};

// Definig the Scanner struct
pub struct Scanner<R> {
    // Reader
    pub reader: R,
    // Buffered String
    pub bufstr: Vec<u8>,
    // Buffered Iterator
    pub bufiter: str::SplitWhitespace<'static>,
}

// Implementing for Scanner Struct
impl<R: BufRead> Scanner<R> {
    // `new` function to initialize a new Scanner
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            bufstr: vec![],
            bufiter: "".split_whitespace(),
        }
    }

    // `input` handles the input
    pub fn input<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(input) = self.bufiter.next() {
                return input.parse().ok().expect("Failed to parse input");
            }

            self.bufstr.clear();

            self.reader
                .read_until(b'\n', &mut self.bufstr)
                .expect("Failed to read input");

            self.bufiter = unsafe {
                let slice = str::from_utf8_unchecked(&self.bufstr);

                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}
