const INPUT: &'static str = include_str!("../input/6.txt");

fn parse() -> Vec<u8> {
    INPUT.trim().split(',').flat_map(str::parse).collect()
}

fn fish(n: usize) -> u64 {
    let mut bins = [0u64; 7];
    for f in parse() {
        bins[f as usize] += 1;
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

pub fn first() -> u64 {
    fish(80)
}

pub fn second() -> u64 {
    fish(256)
}