const INPUT: &'static str = include_str!("../input/2.txt");

pub fn main() -> (u64, u64) {
    let input: Vec<_> = INPUT
        .lines()
        .flat_map(|s| s.split_once(' '))
        .flat_map(|(d, n)| Some((d, n.parse().ok()?)))
        .collect();
    (first(&input), second(&input))
}

fn first(input: &Vec<(&str, i64)>) -> u64 {
    let (x, y) = input.iter()
        .map(|(d, n)| match *d {
            "up" => (0, -n),
            "down" => (0, *n),
            _ => (*n, 0),
        })
        .reduce(|(a, b), (x, y)| (a + x, b + y))
        .unwrap();
    (x * y) as u64
}

fn second(input: &Vec<(&str, i64)>) -> u64 {
    let (x, y, _) = input.iter().fold((0, 0, 0), |(x, y, a), (d, n)| match *d {
        "up" => (x, y, a - n),
        "down" => (x, y, a + n),
        _ => (x + n, y + a * n, a),
    });
    (x * y) as u64
}
