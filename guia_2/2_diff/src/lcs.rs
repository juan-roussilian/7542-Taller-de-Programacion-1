use std::cmp::max;

///Returns a grid that represents the result of peforming the longest common subsecuence algorithm between ```seq_1``` and ```seq_2```
/// # Examples
/// ```
/// let mut first_seq = vec!["a","b","c"];
/// let mut second_seq = vec!["a","c","b"];
/// let grid_lcs = lcs(&first_seq, &second_seq)
/// ``` 
pub fn lcs(seq_1: &[String], seq_2: &[String]) -> Vec<u8> {
    let m = seq_1.len();
    let n = seq_2.len();

    //Index vec like a matrix  => index = (n * i) + j
    let mut grid = vec![0; (n + 1) * (m + 1)];
    for i in 0..m {
        for j in 0..n {
            if seq_1[i] == seq_2[j] {
                grid[(n * (i + 1)) + (j + 1)] = grid[n * i + j] + 1;
            } else {
                grid[(n * (i + 1)) + (j + 1) - 2] =
                    max(grid[(n * (i + 1)) + j], grid[(n * i) + (j + 1)])
            }
        }
    }
    grid
}
