use handle_input::*;
use std::{io::{self, prelude::*}, collections::HashMap};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    // The provided value to add coins to sum up to
    let n = scan.input::<isize>();

    // The number of coins available
    let m = scan.input::<isize>();

    // The available coins to be taken 
    let c: Vec<isize> = (0..m).map(|_| scan.input::<isize>()).collect();

    let mut cache: HashMap<isize, isize> = HashMap::new();

    let result = minimum_coins(n, &c, &mut cache);

    writeln!(out, "The minimum number of coins that sum up to {n} are: {result}").ok();
}

fn minimum_coins(n: isize, c: &Vec<isize>, cache: &mut HashMap<isize, isize>) -> isize {
    if let Some(val) = cache.get(&n) {
        return *val;
    }

    let mut res = isize::MAX;

    if n == 0 {
        return 0;
    } else {
        for coin in c {
            let subproblem = if (n - coin) >= 0 { n - coin } else { 0 };

            res = std::cmp::min(res, minimum_coins(subproblem, c, cache) + 1);
        }
    }

    cache.insert(n, res);

    res
}
