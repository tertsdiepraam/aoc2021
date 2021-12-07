const INPUT: &'static str = include_str!("../input/6.txt");

pub fn main() -> (u64, u64) {
    let mut bins = [0u64; 7];
    for f in INPUT.bytes().step_by(2).map(|b| b - b'0') {
        bins[f as usize] += 1;
    }
    let mut new2 = 0;
    let mut new3 = 0;

    for i in 1..=80 {
        let day = i % 7;
        let new1 = bins[day];
        bins[day] += new3;
        new3 = new2;
        new2 = new1;
    }
    
    let a = bins.iter().sum::<u64>() + new3;

    for i in 81..=256 {
        let day = i % 7;
        let new1 = bins[day];
        bins[day] += new3;
        new3 = new2;
        new2 = new1;
    }

    let b = bins.iter().sum::<u64>() + new3;
    (a, b)
}