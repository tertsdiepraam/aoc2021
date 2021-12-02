mod one;
mod two;

const EXERCISES: [fn() -> i32; 4] = [
    one::first,
    one::second,
    two::first,
    two::second,
];

fn main() {
    for (i, ex) in EXERCISES.chunks(2).enumerate() {
        for (j, e) in ex.iter().enumerate() {
            println!("{}.{}\t{}", i+1, j+1, e());
        }
    }
}
