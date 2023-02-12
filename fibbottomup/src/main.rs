use handle_input::*;
use std::io::{self, prelude::*};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    let n: u128 = scan.input();
    let res = fibbottomup(n);
    write!(out, "{res}").ok();
    out.flush().unwrap();
}

fn fibbottomup(n: u128) -> u128 {
    if n < 2 {
        return n;
    }

    let mut fib_cache = vec![0; (n + 1) as usize];

    fib_cache[0] = 0;
    fib_cache[1] = 1;

    for i in 2..(n + 1) {
        fib_cache[i as usize] = fib_cache[(i - 1) as usize] + fib_cache[(i - 2) as usize];
    }

    fib_cache[n as usize]
}
