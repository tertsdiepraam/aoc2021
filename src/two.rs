const INPUT: &'static str = include_str!("../input/2.txt");

fn parse() -> Vec<(&'static str, i32)> {
    INPUT
        .lines()
        .flat_map(|s| s.split_once(' '))
        .map(|(d, n)| (d, n.parse().unwrap()))
        .collect()
}

pub fn first() -> i32 {
    let (x, y) = parse()
        .into_iter()
        .map(|(d, n): (_, i32)| match d {
            "forward" => (n, 0),
            "up" => (0, -n),
            "down" => (0, n),
            _ => (0, 0),
        })
        .reduce(|(a, b), (x, y)| (a + x, b + y))
        .unwrap();
    x * y
}

pub fn second() -> i32 {
    let (x, y, _) = parse()
        .into_iter()
        .fold((0, 0, 0), |(x, y, a), (d, n): (_, i32)| match d {
            "forward" => (x + n, y + a * n, a),
            "up" => (x, y, a - n),
            "down" => (x, y, a + n),
            _ => (x, y, a),
        });
    x * y
}
