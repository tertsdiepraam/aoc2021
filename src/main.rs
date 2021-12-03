use std::{env, num::ParseIntError, time::Instant};

mod one;
mod two;
mod three;

const EXERCISES: [fn() -> i32; 6] = [one::first, one::second, two::first, two::second, three::first, three::second];

fn main() -> Result<(), ParseIntError> {
    let args: Vec<String> = env::args().collect();

    let (from, to) = match &args[1..] {
        [] => (0usize, EXERCISES.len() / 2),
        [x] => {
            let x = x.parse()?;
            (x - 1, x)
        }
        [a, b] => (a.parse::<usize>()?-1, b.parse::<usize>()?),
        _ => panic!()
    };

    let now = Instant::now();
    let answers: Vec<i32> = EXERCISES[2*from..2*to].iter().map(|e| e()).collect();
    let time = now.elapsed().as_millis();

    for (i, a) in answers.chunks(2).enumerate() {
        println!("Day {}", i + 1);
        println!("  Part 1: {}", a[0]);
        println!("  Part 2: {}", a[1]);
        println!();
    }
    println!("Time: {}ms", time);
    Ok(())
}
