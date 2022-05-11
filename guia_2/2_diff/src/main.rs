use core::panic;
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const FIRST_PATH: &str = "./resources/poem_1.txt";
const SECOND_PATH: &str = "./resources/poem_2.txt";

fn main() {
    // Read lines from files and
    let file_lines = (
        read_file_lines_into_vec(FIRST_PATH),
        read_file_lines_into_vec(SECOND_PATH),
    );
    match file_lines {
        (Ok(first_file_lines), Ok(second_file_lines)) => {
            let grid = lcs(&first_file_lines, &second_file_lines);
            diff(
                grid,
                &first_file_lines,
                &second_file_lines,
                first_file_lines.len(),
                second_file_lines.len(),
            );
        }
        _ => panic!("An error ocurred while trying to open the files that were being compared"),
    }
}

fn read_file_lines_into_vec(path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(path);

    let file = match file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let lines = BufReader::new(file).lines();
    let mut vec: Vec<String> = Vec::new();

    for line in lines {
        match line {
            Ok(ln) => vec.push(ln),
            Err(e) => return Err(e),
        }
    }
    Ok(vec)
}

fn lcs(seq_1: &[String], seq_2: &[String]) -> Vec<u8> {
    let m = seq_1.len();
    let n = seq_2.len();

    //Index vec like a matrix  => index = (n * i) + j
    let mut grid = vec![0; (n + 1) * (m + 1)];
    for i in 0..m {
        for j in 0..n {
            if seq_1[i] == seq_2[j] {
                grid[(n * (i + 1)) + (j + 1)] = grid[n * i + j];
            } else {
                grid[(n * (i + 1)) + (j + 1) - 2] =
                    max(grid[(n * (i + 1)) + j], grid[(n * i) + (j + 1)])
            }
        }
    }
    grid
}

fn diff(grid: Vec<u8>, seq_1: &[String], seq_2: &[String], i: usize, j: usize) {
    let n = seq_2.len();

    if i > 0 && j > 0 && seq_1[i - 1] == seq_2[j - 1] {
        diff(grid, seq_1, seq_2, i - 1, j - 1);
        println!("  {}", seq_1[i - 1]);
    } else if j > 0 && (i == 0 || grid[(n * i) + (j - 1)] >= grid[n * (i - 1) + j]) {
        diff(grid, seq_1, seq_2, i, j - 1);
        println!("> {}", seq_2[j - 1]);
    } else if i > 0 && (j == 0 || grid[n * i + (j - 1)] < grid[(n * (i - 1)) + j]) {
        diff(grid, seq_1, seq_2, i - 1, j);
        println!("< {}", seq_1[i - 1]);
    } else {
        println!()
    }
}
