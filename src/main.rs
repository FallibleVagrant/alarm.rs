use std::env;
use std::thread;
use::std::time::{Duration, Instant};
use std::num::ParseFloatError;

#[derive(Debug)]
struct ParseError {
    message: String,
}

impl From<ParseFloatError> for ParseError {
    fn from(_err: ParseFloatError) -> Self {
        return ParseError {
            message: "float err".to_string(),
        };
    }
}

fn str_to_f64(input: &str) -> Result<f64, ParseError> {
    let mut output = String::new();
    for c in input.chars() {
        if c.is_ascii_digit() || c == '.' {
            output.push(c);
        }
        else {
            break;
        }
    }

    return Ok(output.parse::<f64>()?);
}

fn get_suffix(input: &str) -> Option<char> {
    for c in input.chars() {
        if c.is_ascii_digit() || c == '.' {
            continue;
        }
        match c {
            's' => return Some('s'),
            'm' => return Some('m'),
            'h' => return Some('h'),
            'd' => return Some('d'),
            _ => {
                return None;
            },
        }
    }

    return Some('s');
}

fn parse_to_seconds(time_suffixed: &str) -> f64 {
    let possible_float = str_to_f64(time_suffixed);
    let mut output: f64;
    match possible_float {
        Ok(f) => output = f,
        Err(_) => {
            eprintln!("Could not parse arg: {:?}; did not start with float.", time_suffixed);
            return 0.0;
        },
    }

    match get_suffix(time_suffixed) {
        Some('s') => {},
        Some('m') => output *= 60.0,
        Some('h') => output *= 60.0 * 60.0,
        Some('d') => output *= 60.0 * 60.0 * 24.0,
        None => {
            eprintln!("Could not parse arg: {:?}; unknown suffix.", time_suffixed);
            return 0.0;
        },
        _ => panic!("Should never reach this state."),
    }

    return output;
}

fn is_deadline_passed(current_time: Instant, deadline: Instant) -> bool {
    return current_time.checked_duration_since(deadline).is_some();
}

fn sleep_for_a_time(previous_time: Instant, current_time: Instant, remaining_duration: Duration, duration_to_sleep: &mut Duration) {
    let min_sleep_duration = Duration::from_secs_f64(0.1);
    let baseline_sleep_duration = Duration::from_secs_f64(0.5);
    let max_sleep_duration = Duration::from_secs(60);
    let grace_period = Duration::from_secs_f64(0.5);

    //Test if the system was suspended while this process was asleep.
	eprintln!("current_time - previous_time: {:?}.", current_time - previous_time);
	eprintln!("*duration_to_sleep + grace_period: {:?}.", *duration_to_sleep + grace_period);
    if current_time - previous_time > *duration_to_sleep + grace_period {
        println!("System was detected asleep; resetting sleep timer.");
        eprintln!("System was detected asleep; resetting sleep timer.");
        *duration_to_sleep = baseline_sleep_duration;
    }

    *duration_to_sleep *= 2;

    if *duration_to_sleep > max_sleep_duration {
		eprintln!("Hit above {:?} secs sleep period, setting to {:?}.", max_sleep_duration, max_sleep_duration);
        *duration_to_sleep = max_sleep_duration;
    }

	//Test if alarm_time is close to current_time.
    if *duration_to_sleep >= remaining_duration {
        if *duration_to_sleep - remaining_duration >= min_sleep_duration {
            *duration_to_sleep = remaining_duration / 2;
			eprintln!("duration_to_sleep is close to remaining_duration, setting to {:?}.", *duration_to_sleep);
        }
        else {
			eprintln!("duration_to_sleep is close to remaining_duration, setting to 0.");
            *duration_to_sleep = Duration::from_secs(0);
        }
    }

    eprintln!("Sleeping for: {:?} seconds.", *duration_to_sleep);
    thread::sleep(*duration_to_sleep);
}

//Probably implemented in the stdlib by the time you are reading this.
fn sleep_until(deadline: Instant) {
    let mut current_time = Instant::now();
    let mut previous_time: Instant;
    let mut remaining_duration: Duration;

    let init_sleep_duration = Duration::from_secs_f64(0.5);

    let mut duration_to_sleep = init_sleep_duration;

    eprintln!("Alarm time is: {:?}", deadline);
    eprintln!("Current time is: {:?}", current_time);

    while !is_deadline_passed(current_time, deadline) {
        previous_time = current_time;
        current_time = Instant::now();
        remaining_duration = deadline.checked_duration_since(current_time)
                                    .unwrap_or_else(|| Duration::from_secs(0));

        eprintln!("-------------------------------------------------remaining_duration is: {:?}.", remaining_duration);
        eprintln!("current_time: {:?}.", current_time);
        eprintln!("previous_time: {:?}.", previous_time);

        sleep_for_a_time(previous_time, current_time, remaining_duration, &mut duration_to_sleep);
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments.");
        std::process::exit(-1);
    }

    let mut time_to_wait: f64 = 0.0;

    for arg in args.into_iter().skip(1) {
        time_to_wait += parse_to_seconds(&arg);
    }

    eprintln!("time_to_wait: {:?}", time_to_wait);
    sleep_until(Instant::now() + Duration::from_secs_f64(time_to_wait));
}
