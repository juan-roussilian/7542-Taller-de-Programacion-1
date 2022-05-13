use diff_app::app::run;
use std::env;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("Two files are required to generate diff");
    }

    let first_path = &args[1].as_str();
    let second_path = &args[2].as_str();

    run(first_path, second_path)
}
