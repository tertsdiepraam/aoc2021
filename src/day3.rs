const INPUT: &'static str = include_str!("../input/3.txt");

pub fn first() -> u64 {
    let mut lines: Vec<_> = INPUT.lines().map(|l| l.chars()).collect();

    let mut a = [0; 12];

    for i in 0..12 {
        let mut count = 0;
        for l in &mut lines {
            count += (l.next_back() == Some('1')) as usize;
        }
        a[(2 * count >= lines.len()) as usize] += 1 << i;
    }

    a[0] * a[1]
}

pub fn second() -> u64 {
    let v: Vec<_> = INPUT
        .lines()
        .map(|l| -> Vec<_> { l.chars().map(|c| c == '1').collect() })
        .collect();

    let mut vec = v.clone();
    for i in 0..vec[0].len() {
        let to_retain = 2 * vec.iter().filter(|x| x[i]).count() >= vec.len();
        vec.retain(|x| x[i] == to_retain);
        if vec.len() == 1 {
            break;
        }
    }

    let mut a = 0;
    for (i, &elem) in vec[0].iter().rev().enumerate() {
        a += (elem as u64) << i;
    }

    let mut vec = v;
    for i in 0..vec[0].len() {
        let to_retain = 2 * vec.iter().filter(|x| x[i]).count() < vec.len();
        vec.retain(|x| x[i] == to_retain);
        if vec.len() == 1 {
            break;
        }
    }

    let mut b = 0;
    for (i, &elem) in vec[0].iter().rev().enumerate() {
        b += (elem as u64) << i;
    }

    a * b
}
