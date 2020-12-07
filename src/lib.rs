// Days

mod day01;
pub fn noop(_inp: String) -> Option<String> {
    None
}

// A Day function must return an Option<String>. The string is used for submission via aocf
pub type DayFn = fn(String) -> Option<String>;

// returns the day functions for the given day number
pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => (day01::part1, day01::part2),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
