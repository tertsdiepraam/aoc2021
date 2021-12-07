use std::{env, num::ParseIntError, time::{Instant, Duration}};
use procs::{mods, bench_all};

mods!();

const NANOSECOND_THRESHOLD: Duration = Duration::from_micros(10);
const MICROSECOND_THRESHOLD: Duration = Duration::from_millis(10);

fn format_duration(t: Duration) -> String {
    match t {
        _ if t < NANOSECOND_THRESHOLD => format!("{} ns", t.as_nanos()),
        _ if t < MICROSECOND_THRESHOLD => format!("{} μs", t.as_micros()),
        _ => format!("{} ms", t.as_millis()),
    }
}

fn bench(n: usize, exercises: &Vec<usize>, f: fn() -> (u64, u64)) -> Option<Duration> {
    if !exercises.contains(&n) { return None; }
    let now = Instant::now();
    let (a, b) = f();
    let t = now.elapsed();
    println!("Day {}", n);
    println!("  Part 1: {}", a);
    println!("  Part 2: {}", b);
    println!("  Time: {}", format_duration(t));
    println!();
    Some(t)
}

fn main() -> Result<(), ParseIntError> {
    let args: Vec<String> = env::args().collect();

    let exercises: Vec<_> = match &args[1..] {
        [] => (1usize..=25).collect(),
        xs => xs.iter().map(|x| x.parse()).collect::<Result<_, _>>()?,
    };

    // This macro calls bench for all selected exercises
    let time = bench_all!();
    println!("Total time: {}", format_duration(time));
    Ok(())
}
