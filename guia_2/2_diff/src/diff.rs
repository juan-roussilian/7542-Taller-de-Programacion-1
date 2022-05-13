pub fn diff(grid: Vec<u8>, seq_1: &[String], seq_2: &[String], i: usize, j: usize) {
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
