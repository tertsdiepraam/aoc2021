const INPUT: &'static str = include_str!("../input/3.txt");

pub fn first() -> i32 {
    let lines = INPUT.lines().collect::<Vec<_>>().len();
    let v = INPUT
        .lines()
        .map(|l| -> Vec<_> { l.chars().map(|c| (c == '1') as i32).collect() })
        .reduce(|mut a, b| {
            for i in 0..a.len() {
                a[i] += b[i];
            }
            a
        })
        .unwrap()
        .iter()
        .map(|&n| ((n as f64) / (lines as f64)).round())
        .collect::<Vec<_>>();

    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, &elem) in v.iter().rev().enumerate() {
        if elem == 1.0 {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }
    gamma * epsilon
}

pub fn second() -> i32 {
    let v: Vec<_> = INPUT
        .lines()
        .map(|l| -> Vec<_> { l.chars().map(|c| (c == '1') as i32).collect() })
        .collect();

    
    let mut vec = v.clone();
    for i in 0..vec[0].len() {
        let to_retain = (2 * vec.iter().filter(|x| x[i] == 1).count() >= vec.len()) as i32;
        vec.retain(|x| x[i] == to_retain);
        if vec.len() == 1 {
            break;
        }
    }

    let mut oxygen = 0;
    for (i, &elem) in vec[0].iter().rev().enumerate() {
        if elem as f64 == 1.0 {
            oxygen += 1 << i;
        }
    }

    let mut vec = v.clone();
    for i in 0..vec[0].len() {
        let to_retain = (2 * vec.iter().filter(|x| x[i] == 1).count() < vec.len()) as i32;
        vec.retain(|x| x[i] == to_retain);
        if vec.len() == 1 {
            break;
        }
    }

    let mut co2 = 0;
    for (i, &elem) in vec[0].iter().rev().enumerate() {
        if elem as f64 == 1.0 {
            co2 += 1 << i;
        }
    }

    oxygen * co2
}
