const INPUT: &'static str = include_str!("../input/1.txt");

pub fn parse() -> Vec<usize> {
    INPUT.lines().flat_map(str::parse).collect()
}

pub fn first() -> i32 {
    parse().windows(2).filter(|w| w[1] > w[0]).count() as i32
}

pub fn second() -> i32 {
    parse().windows(4).filter(|w| w[3] > w[0]).count() as i32
}