use std::env;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_id = &args[1];

    if day_id == "1" {
        day1::day1();
    } else if day_id == "2" {
        day2::day2();
    }
}
