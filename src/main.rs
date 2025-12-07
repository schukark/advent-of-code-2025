use std::{env, time::Instant};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num = match args[1].parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("An error occured: {e}");
            return;
        }
    };

    let _timer = Timer::new();

    match num {
        1 => println!("{}", day01::solve()),
        2 => println!("{}", day02::solve()),
        3 => println!("{}", day03::solve()),
        4 => println!("{}", day04::solve()),
        5 => println!("{}", day05::solve()),
        6 => println!("{}", day06::solve()),
        7 => println!("{}", day07::solve()),
        _ => eprintln!("{num} is not a valid problem number"),
    }
}

struct Timer {
    start_timer: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start_timer: Instant::now(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!(
            "Timer measured: {}ms",
            (Instant::now() - self.start_timer).as_millis()
        );
    }
}
