const INPUT: &'static str = include_str!("../input/6.txt");

pub fn main() -> (u64, u64) {
    let input: Vec<_> = INPUT.trim().split(',').flat_map(str::parse).collect();
    (fish(80, &input), fish(256, &input))
}

fn fish(n: usize, initial: &Vec<usize>) -> u64 {
    let mut bins = [0u64; 7];
    for &f in initial {
        bins[f] += 1;
    }
    let mut new2 = 0;
    let mut new3 = 0;
    for i in 1..=n {
        let day = i % 7;
        let new1 = bins[day];
        bins[day] += new3;
        new3 = new2;
        new2 = new1;
    }
    bins.iter().sum::<u64>() + new3
}