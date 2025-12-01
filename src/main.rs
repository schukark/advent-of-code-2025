use std::env;

pub mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num = match args[1].parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("An error occured: {e}");
            return;
        }
    };

    match num {
        1 => println!("{}", day01::solve()),
        _ => eprintln!("{num} is not a valid problem number"),
    }
}
