use solver::Solver;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

mod solver;

fn solve_day(day: i32) {
    match day {
        1 => day1::Problem {}.solve(),
        2 => day2::Problem {}.solve(),
        3 => day3::Problem {}.solve(),
        4 => day4::Problem {}.solve(),
        5 => day5::Problem {}.solve(),
        6 => day6::Problem {}.solve(),
        7 => day7::Problem {}.solve(),
        d => println!("Day {} hasn't been solved yet :(", d),
    }
}

fn main() {
    let day = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("1"))
        .parse()
        .unwrap_or(1);
    solve_day(day);
}
