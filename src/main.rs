use std::env;
mod days;
mod io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if !(args.len() > 2) {
        eprintln!("Call must at least specify a day.");
        return;
    }

    let day: &str = &args[1];
    let args = &args[2..];

    match day {
        "day1" => days::day1::day1(args),
        "day2" => days::day2::day2(args),
        _ => eprintln!("Not a valid day: {day}"),
    }
}
