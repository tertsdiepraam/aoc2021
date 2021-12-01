mod one;

const EXERCISES: [fn() -> usize; 2] = [
    one::first,
    one::second,
];

fn main() {
    for (i, ex) in EXERCISES.chunks(2).enumerate() {
        for (j, e) in ex.iter().enumerate() {
            println!("{}.{}\t{}", i+1, j+1, e());
        }
    }
}
