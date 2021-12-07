const INPUT: &'static str = include_str!("../input/4.txt");
const CARD_SIZE: usize = 5;

// Day 4
//   Part 1: 58374
//   Part 2: 11377
//   Time: 145μs

#[derive(Clone)]
struct Card {
    fields: [usize; CARD_SIZE * CARD_SIZE],
    sums: [usize; 2 * CARD_SIZE],
}

pub fn main() -> (u64, u64) {
    let (numbers, cards) = parse();
    let ((_, min_score), (_, max_score)) =
        cards.fold(((usize::MAX, 0), (usize::MIN, 0)), |(mut min, mut max), mut c| {
            for (k, n) in numbers.iter().enumerate() {
                if let Some(i) = c.fields.iter().position(|f| f == n) {
                    let (x, y) = (i / CARD_SIZE, CARD_SIZE + (i % CARD_SIZE));

                    c.sums[x] -= n;
                    c.sums[y] -= n;

                    if c.sums[x] == 0 || c.sums[y] == 0 {
                        if k < min.0 {
                            min = (k, (c.sums[..CARD_SIZE].iter().sum::<usize>() * n) as u64)
                        }
                        if k > max.0 {
                            max = (k, (c.sums[..CARD_SIZE].iter().sum::<usize>() * n) as u64)
                        }
                        break;
                    }
                }
            }
            (min, max)
        });
    (min_score, max_score)
}

fn parse() -> (Vec<usize>, impl Iterator<Item = Card>) {
    let mut lines = INPUT.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .flat_map(str::parse)
        .collect();

    let cards = std::iter::from_fn(move || {
        lines.next()?;
        let mut fields = [0; CARD_SIZE * CARD_SIZE];
        let mut sums = [0; 2 * CARD_SIZE];
        for (i, val) in lines
            .by_ref()
            .take(5)
            .flat_map(|l| l.split_ascii_whitespace().flat_map(str::parse))
            .enumerate()
        {
            fields[i] = val;
            sums[i / CARD_SIZE] += val;
            sums[CARD_SIZE + i % CARD_SIZE] += val;
        }
        Some(Card { fields, sums })
    });

    (numbers, cards)
}
