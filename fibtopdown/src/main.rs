use handle_input::*;
use std::{collections::HashMap, io::{prelude::*, self}};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());

    let mut scan = Scanner::new(stdin.lock());

    let mut out = io::BufWriter::new(stdout.lock());

    write!(out, "Enter a number : ").ok();

    out.flush().unwrap();

    let num = scan.input::<u128>();

    let mut cache: HashMap<u128, u128> = HashMap::new();

    cache.insert(1, 1);
    cache.insert(2, 1);

    let res = fibtopdown(num, &mut cache);

    writeln!(out, "The number in fibonacci series at position : {num} is :\n=> {res}").ok();
}

fn fibtopdown(num: u128, cache: &mut HashMap<u128, u128>) -> u128 {
    if num == 0 || num == 1 {
        return num;
    }

    if let Some(v) = cache.get(&num) {
        return *v
    }

    let res = fibtopdown(num - 1, cache) + fibtopdown(num - 2, cache);

    cache.insert(num, res);

    return cache[&num]
}
