const INPUT: &'static str = include_str!("../input/1.txt");

pub fn main() -> (u64, u64) {
    let input: Vec<usize> = INPUT.lines().flat_map(str::parse).collect();
    (first(&input), second(&input))
}

fn first(input: &Vec<usize>) -> u64 {
    input.windows(2).filter(|w| w[1] > w[0]).count() as u64
}

fn second(input: &Vec<usize>) -> u64 {
    input.windows(4).filter(|w| w[3] > w[0]).count() as u64
}