const INPUT: &'static str = include_str!("../input/7.txt");

pub fn main() -> (u64, u64) {
    let mut input: Vec<_> = INPUT.trim_end().split(',').flat_map(|x| x.parse::<i64>()).collect();
    
    let index = input.len() / 2;
    let (_, &mut median, _) = input.select_nth_unstable(index);
    let a: u64 = input.iter().map(|x| (x - median).abs()).sum::<i64>() as u64;
    
    let average = input.iter().sum::<i64>() / input.len() as i64;
    let b = input.iter().map(|x| (x - average).abs()).map(|x| x*(1+x)/2).sum::<i64>() as u64;
    
    (a, b)
}