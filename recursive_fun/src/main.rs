// Using the local crate for handling inputs
use handle_input::*;
// Using the standard input library
use std::io::{self, prelude::*};

fn main() {
    // Define the input and output streams
    let (stdin, stdout) = (io::stdin(), io::stdout());

    // Defining the `in` Scanner from `handle_input` crate
    let mut scan = Scanner::new(stdin.lock());

    // Defining the `out` buffer sink
    let mut out = io::BufWriter::new(stdout.lock());

    // Write to the `out` sink
    write!(out, "Enter a number to recursive add all smaller numbers to it: ").ok();

    // The `write!` and `writeln!` macros need to be flushed 
    // to put output on the defined sink
    out.flush().unwrap();

    // Take the number input
    let num = scan.input::<usize>();

    // res will hold the sum of all the numbers from the value `num` to `1`
    let res = recursive_sum(num);

    // Write the result of recursive sum to the output stream (sink)
    writeln!(out, "Recursive Sum is : {res}").ok();
}

// Defining the function to add numbers recursively
fn recursive_sum(n: usize) -> usize {
    // Self Explanatory
    if n == 0 {
        return n
    }

    // If `n != 0`, then we call the function again with `n - 1`
    n + recursive_sum(n - 1)
}
