use std::time::Duration;
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

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return String::from(format!("{}µs", micro_sec.round()));
    }
    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return String::from(format!("{}ms ", whole_ms) + &fmt_time(rem_ms));
    }
    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;
        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }
    let min: f64 = sec / 60.0;
    return format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0);
}

pub fn fmt_dur(dur: Duration) -> String {
    return fmt_time(dur.as_secs_f64() * 1000.0);
}
