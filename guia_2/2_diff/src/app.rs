use super::diff::diff;
use super::lcs::lcs;
use super::utils::read_file_lines_into_vec;

pub fn run(first_path: &&str, second_path: &&str) -> Result<(), &'static str> {
    // Read lines from files
    let file_lines = (
        read_file_lines_into_vec(first_path),
        read_file_lines_into_vec(second_path),
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
            Ok(())
        }
        _ => Err("An error ocurred while trying to open the files that were being compared"),
    }
}
