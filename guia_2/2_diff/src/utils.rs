use std::result::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
///Reads the lines from the path at ```path_string``` and returns an vec containing said lines on success or a string representing the error.
/// 
///  # Examples
/// ```
/// lines = match read_file_lines_into_vec(&"file".as_str()){
///        Ok(lines) => lines,
///        Err(error_message) => println!(error_message),
///     };
/// ```
pub fn read_file_lines_into_vec(path_string: &str) -> Result<Vec<String>, String> {
    let file = match File::open(path_string) {
        Ok(file) => file,
        Err(_) => return Err("Could not open the specified path".to_string()),
    };

    let lines = BufReader::new(file).lines();
    let mut vec: Vec<String> = Vec::new();

    for line in lines {
        match line {
            Ok(ln) => vec.push(ln),
            Err(_) => {
                return Err("An error ocurred while trying to read the file lines".to_string())
            }
        }
    }
    Ok(vec)
}

#[cfg(test)]
mod test {

    use super::read_file_lines_into_vec;
    use std::{
        fs::{remove_file, File},
        io::Write,
    };

    #[test]
    fn reading_from_invalid_path_fails() -> Result<(), String> {
        let invalid_path = "invalid".to_owned();
        let result = read_file_lines_into_vec(invalid_path.as_str());
        match result {
            Ok(_) => Err("Diff ran with invalid filepaths".to_owned()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn reading_from_valid_path_works() -> Result<(), String> {
        let path = "lorem_ipsum1.txt";
        init_lorep_ipsum_file(path);
        let result = read_file_lines_into_vec(&path);
        remove_file(path);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Diff ran with invalid filepaths".to_owned()),
        }
    }

    //Fails somehow sometimes lines.len() returns 5(actual number of lines) and sometimes 0
    #[test]
    fn amount_of_read_lines_is_correct() -> Result<(), ()> {
        let path = "lorem_ipsum2.txt";
        init_lorep_ipsum_file(path);
        let lines = match read_file_lines_into_vec(&path) {
            Ok(lines) => lines,
            Err(_) => return Err(()),
        };
        remove_file(path);
        assert_eq!(lines.len(), 3);
        Ok(())
    }

    fn init_lorep_ipsum_file(path: &str) -> File {
        let lorem_ipsum: &str =
            "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
        quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo";

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", path, why),
            Ok(file) => file,
        };
        match file.write_all(lorem_ipsum.as_bytes()) {
            Err(why) => panic!("couldn't create {}: {}", path, why),
            Ok(file) => file,
        };
        file
    }
}
