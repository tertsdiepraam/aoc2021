// Day 8
//   Part 1: 445
//   Part 2: 1043101

const INPUT: &'static str = include_str!("../input/8.txt");

#[derive(Default, PartialEq, Eq)]
struct Signal([bool; 7]);

impl Signal {
    fn count(&self) -> usize {
        self.0.iter().filter(|&&x| x).count()
    }

    fn contains(&self, other: &Signal) -> bool {
        self.0.iter().zip(other.0).all(|(&s, o)| !o || s)
    }

    fn overlap(&self, other: &Signal) -> usize {
        self.0.iter().zip(other.0).filter(|&(&s, o)| o && s).count()
    }
}

impl From<&str> for Signal {
    fn from(s: &str) -> Signal {
        let mut a = [false; 7];
        for c in s.bytes() {
            a[(c - b'a') as usize] = true;
        }
        Signal(a)
    }
}


fn parse() -> Vec<([Signal; 10], [Signal; 4])> {
    INPUT
        .lines()
        .map(|l| {
            let mut signal: [Signal; 10] = Default::default();
            let mut output: [Signal; 4] = Default::default();
            let (s, o) = l.split_once(" | ").unwrap();
            s.split(' ').enumerate().for_each(|(i, x)| {
                signal[i] = x.into();
            });
            o.split(' ').enumerate().for_each(|(i, x)| output[i] = x.into());
            (signal, output)
        })
        .collect()
}

pub fn main() -> (u64, u64) {
    let input = parse();
    
    let a = input
        .iter()
        .flat_map(|(_, o)| o.iter().filter(|d| matches!(d.count(), 2 | 4 | 3 | 7)))
        .count() as u64;

    let b = input
        .iter()
        .map(|(seq, out)| {
            let mut numbers = [0; 10];
            let mut one = 0;
            let mut four = 0;
            for i in 0..10 {
                numbers[i] = match seq[i].count() {
                    2 => {
                        one = i;
                        1
                    }
                    3 => 7,
                    4 => {
                        four = i;
                        4
                    }
                    5 => 2, // 3, 5
                    6 => 0, // 6, 9
                    7 => 8,
                    _ => unreachable!(),
                };
            }

            for i in 0..10 {
                let s = &seq[i];
                let n = numbers[i];
                numbers[i] = match n {
                    2 => {
                        if s.contains(&seq[one]) {
                            3
                        } else if s.overlap(&seq[four]) == 3 {
                            5
                        } else {
                            2
                        }
                    }
                    0 => {
                        if !s.contains(&seq[one]) {
                            6
                        } else if s.contains(&seq[four]) {
                            9
                        } else {
                            0
                        }
                    }
                    _ => n,
                }
            }

            let mut res = 0;
            for (i, o) in out.iter().enumerate() {
                res += numbers[seq.iter().position(|x| x == o).unwrap()] * 10u64.pow(3 - i as u32);
            }

            res
        })
        .sum::<u64>();
    (a, b)
}
