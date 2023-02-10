use handle_input::*;
use std::io::{prelude::*, self};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());

    let mut scan = Scanner::new(stdin.lock());

    let mut out = io::BufWriter::new(stdout.lock());

    write!(out, "Enter a number : ").ok();

    out.flush().unwrap();

    let num = scan.input::<u128>();

    let res = fibtopdown(num);

    writeln!(out, "The number in fibonacci series at position : {num} is :\n=> {res}").ok();
}

fn fibtopdown(num: u128) -> u128 {
    if num < 2 {
        return num;
    }

    let mut fib_cache = vec![0; (num + 1) as usize];

    fib_cache[0] = 0;
    fib_cache[1] = 1;

    for i in 2..(num + 1) {
        fib_cache[i as usize] = fib_cache[(i - 1) as usize] + fib_cache[(i - 2) as usize];
    }

    fib_cache[num as usize]
}
