const INPUT: &'static str = include_str!("../input/2.txt");

fn parse() -> impl Iterator<Item = (&'static str, i32)> {
    INPUT
        .lines()
        .flat_map(|s| s.split_once(' '))
        .flat_map(|(d, n)| Some((d, n.parse().ok()?)))
}

pub fn first() -> i32 {
    let (x, y) = parse()
        .map(|(d, n)| match d {
            "up" => (0, -n),
            "down" => (0, n),
            _ => (n, 0),
        })
        .reduce(|(a, b), (x, y)| (a + x, b + y))
        .unwrap();
    x * y
}

pub fn second() -> i32 {
    let (x, y, _) = parse().fold((0, 0, 0), |(x, y, a), (d, n)| match d {
        "up" => (x, y, a - n),
        "down" => (x, y, a + n),
        _ => (x + n, y + a * n, a),
    });
    x * y
}
