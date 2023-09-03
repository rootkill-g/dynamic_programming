use handle_input::*;
use std::io::{self, prelude::*};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());

    let mut scan = Scanner::new(stdin.lock());

    let mut out = io::BufWriter::new(stdout.lock());

    // Number of Rows in the Maze
    let n = scan.input::<usize>();
    
    // Number of Columns in the Maze
    let m = scan.input::<usize>();

    // Maze Grid Map
    let mut maze: Vec<Vec<usize>> = vec![vec![0; m]; n];

    let result = grid_paths(n, m, &mut maze);

    writeln!(out, "Number of ways the 🐇 can go from Top Left Corner to Bottom Right Corner are: {}", result).ok();
}

fn grid_paths(n: usize, m: usize, maze: &mut Vec<Vec<usize>>) -> usize {
    for i in 1..=n {
        maze[i - 1][0] = 1;
    }

    for j in 1..=m {
        maze[0][j - 1] = 1;
    }

    for i in 1..n {
        for j in 1..m {
            maze[i][j] = maze[i - 1][j] + maze[i][j - 1]
        }
    }

    return maze[n - 1][m - 1];
}
