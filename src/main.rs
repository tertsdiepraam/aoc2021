use std::{env, num::ParseIntError, time::Instant};
use procs::{mods, bench_all};

mods!();

fn bench(n: usize, exercises: &Vec<usize>, f: fn() -> (u64, u64)) -> Option<u128> {
    if !exercises.contains(&n) { return None; }
    let now = Instant::now();
    let (a, b) = f();
    let t = now.elapsed().as_micros();
    println!("Day {}", n);
    println!("  Part 1: {}", a);
    println!("  Part 2: {}", b);
    println!("  Time: {}μs", t);
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
    println!("Total time: {}μs", time);
    Ok(())
}
