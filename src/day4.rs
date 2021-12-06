const INPUT: &'static str = include_str!("../input/4.txt");
const CARD_SIZE: usize = 5;

#[derive(Clone)]
struct Card {
    fields: [usize; CARD_SIZE * CARD_SIZE],
    sums: [usize; 2 * CARD_SIZE],
}

pub fn main() -> (u64, u64) {
    let (numbers, cards) = parse();
    (first(&numbers, cards.clone()), second(&numbers, cards))
}

fn parse() -> (Vec<usize>, Vec<Card>) {
    let mut lines = INPUT.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .flat_map(str::parse)
        .collect();

    let mut cards = Vec::with_capacity(100);

    while lines.next().is_some() {
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
        cards.push(Card { fields, sums });
    }

    (numbers, cards)
}

fn first(numbers: &Vec<usize>, mut cards: Vec<Card>) -> u64 {
    for n in numbers {
        for c in &mut cards {
            if let Some(i) = c.fields.iter().position(|f| f == n) {
                let (x, y) = (i / CARD_SIZE, CARD_SIZE + (i % CARD_SIZE));

                c.sums[x] -= n;
                c.sums[y] -= n;

                if c.sums[x] == 0 || c.sums[y] == 0 {
                    return (c.sums[..CARD_SIZE].iter().sum::<usize>() * n) as u64;
                }
            }
        }
    }
    0
}

fn second(numbers: &Vec<usize>, mut cards: Vec<Card>) -> u64 {
    cards
        .iter_mut()
        .map(|c| {
            for (k, n) in numbers.iter().enumerate() {
                if let Some(i) = c.fields.iter().position(|f| f == n) {
                    let (x, y) = (i / CARD_SIZE, CARD_SIZE + (i % CARD_SIZE));

                    c.sums[x] -= n;
                    c.sums[y] -= n;

                    if c.sums[x] == 0 || c.sums[y] == 0 {
                        return (k, (c.sums[..CARD_SIZE].iter().sum::<usize>() * n) as u64);
                    }
                }
            }
            (0, 0)
        })
        .max_by_key(|t| t.0)
        .unwrap_or((0, 0))
        .1
}
