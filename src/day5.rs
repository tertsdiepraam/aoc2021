const INPUT: &'static str = include_str!("../input/5.txt");
const SIZE: usize = 1000;

fn parse_coordinates(s: &str) -> (usize, usize) {
    let (a, b) = s.split_once(',').unwrap();
    (a.trim().parse().unwrap(), b.trim().parse().unwrap())
}

fn parse() -> Vec<((usize, usize), (usize, usize))> {
    INPUT
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("->").unwrap();
            (parse_coordinates(a), parse_coordinates(b))
        })
        .collect()
}

pub fn first() -> u64 {
    let mut b = [[0u8; SIZE]; SIZE];
    let mut twos = 0;
    for ((x1, y1), (x2, y2)) in parse() {
        if x1 == x2 || y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                for y in y1.min(y2)..=y1.max(y2) {
                    b[y as usize][x as usize] += 1;
                    twos += (b[y as usize][x as usize] == 2) as u64;
                }
            }
        }
    }
    twos
}

pub fn second() -> u64 {
    let mut b = [[0u8; SIZE]; SIZE];
    let mut twos = 0;
    for ((x1, y1), (x2, y2)) in parse() {
        let ((x1, y1), (x2, y2)) = ((x1 as i32, y1 as i32), (x2 as i32, y2 as i32));
        let diff = (x2 - x1).abs().max((y2 - y1).abs());
        let x_sign = (x2 - x1).signum();
        let y_sign = (y2 - y1).signum();
        for i in 0..=diff {
            let x = (x1 + x_sign * i) as usize;
            let y = (y1 + y_sign * i) as usize;
            b[y][x] += 1;
            twos += (b[y][x] == 2) as u64;
        }
    }
    twos
}
