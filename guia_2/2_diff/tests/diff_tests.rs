extern crate diff_app;

#[cfg(test)]

mod tests {

    use diff_app::app::run;
    use std::{
        fs::{remove_file, File},
        io::Write,
    };

    #[test]
    fn app_with_invalid_files_fails() -> Result<(), String> {
        let invalid_path = "invalid".to_owned();
        let result = run(&invalid_path.as_str(), &invalid_path.as_str());
        match result {
            Ok(()) => Err("Diff ran with invalid filepaths".to_owned()),
            _ => Ok(()),
        }
    }

    #[test]
    fn app_with_valid_files_works_corectly() -> Result<(), String> {
        let path = "lorem_ipsum.txt";
        init_lorep_ipsum_file(path);

        let result = run(&path, &path);
        remove_file(path);
        match result {
            Ok(()) => Ok(()),
            _ => Err("Diff could not run with valid filepaths".to_owned()),
        }
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
