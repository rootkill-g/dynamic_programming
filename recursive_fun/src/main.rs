use std::{
    io::{self, prelude::*},
    str,
};

struct Scanner<R> {
    reader: R,
    bufstr: Vec<u8>,
    bufiter: str::SplitWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader, 
            bufstr: vec![],
            bufiter: "".split_whitespace(),
        }
    }

    fn input<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(input) = self.bufiter.next() {
                return input.parse().ok().expect("Failed to parse input");
            }

            self.bufstr.clear();

            self.reader.read_until(b'\n', &mut self.bufstr).expect("Failed to read input");

            self.bufiter = unsafe {
                let slice = str::from_utf8_unchecked(&self.bufstr);

                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());

    let mut scan = Scanner::new(stdin.lock());

    let mut out = io::BufWriter::new(stdout.lock());

    write!(out, "Enter a number to recursive add all smaller numbers to it: ").ok();

    out.flush().unwrap();

    let num = scan.input::<usize>();

    let res = recursive_sum(num);

    writeln!(out, "Recursive Sum is : {res}").ok();
}

fn recursive_sum(n: usize) -> usize {
    if n == 0 {
        return n
    }

    n + recursive_sum(n - 1)
}
