use std::{io::{BufRead, BufReader, Error}, fs::File};

pub fn read_file_lines_into_vec(path_string: &str) -> Result<Vec<String>, Error> {

    let file= File::open(path_string)?;

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