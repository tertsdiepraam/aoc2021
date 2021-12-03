use std::{env, num::ParseIntError, time::Instant};
use procs::{mods, exercises};

mods!();
exercises!();

fn main() -> Result<(), ParseIntError> {
    let args: Vec<String> = env::args().collect();

    let is: Vec<_> = match &args[1..] {
        [] => (1usize..=(EXERCISES.len()/2)).collect(),
        xs => xs.iter().map(|x| x.parse()).collect::<Result<_, _>>()?,
    };

    let now = Instant::now();
    let answers: Vec<_> = is.iter().map(|i| (i, EXERCISES[2*i-2](), EXERCISES[2*i-1]())).collect();
    let time = now.elapsed().as_micros();

    for (i, a, b) in answers {
        println!("Day {}", i);
        println!("  Part 1: {}", a);
        println!("  Part 2: {}", b);
        println!();
    }
    println!("Time: {}μs", time);
    Ok(())
}
