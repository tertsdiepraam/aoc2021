const INPUT: &'static str = include_str!("../input/4.txt");

type Card = [[Field; 5]; 5];

#[derive(Default, Debug, Clone, Copy)]
struct Field {
    n: i32,
    crossed: bool,
}

fn parse() -> (Vec<i32>, Vec<Card>) {
    let mut lines = INPUT.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .flat_map(str::parse)
        .collect();

    let mut cards = Vec::new();
    // The whitespace
    while lines.next().is_some() {
        let mut card = [[Field::default(); 5]; 5];
        for i in 0..5 {
            let line = lines.next().unwrap();
            dbg!(line);
            for (j, val) in line.split(' ').flat_map(str::parse).enumerate() {
                card[i][j].n = val;
            }
        }
        cards.push(card);
    }

    (numbers, cards)
}

pub fn first() -> i32 {
    let (numbers, mut cards) = parse();
    for n in numbers {
        for c in &mut cards {
            for row in c {
                for field in row {
                    if field.n == n {
                        field.crossed = true;
                    }
                }
            }
        }
        for c in &cards {
            for i in 0..5 {
                if c[i].iter().all(|f| f.crossed) || c.iter().map(|row| row[i]).all(|f| f.crossed) {
                    let total: i32 = c
                        .iter()
                        .flat_map(|r| r.map(|f| if f.crossed { 0 } else { f.n }))
                        .sum();
                    return total * n;
                }
            }
        }
    }
    0
}

pub fn second() -> i32 {
    let (numbers, mut cards) = parse();
    for n in numbers {
        for c in &mut cards {
            for row in c {
                for field in row {
                    if field.n == n {
                        field.crossed = true;
                    }
                }
            }
        }
        if cards.len() > 1 {
            cards.retain(|c| {
                for i in 0..5 {
                    if c[i].iter().all(|f| f.crossed)
                        || c.iter().map(|row| row[i]).all(|f| f.crossed)
                    {
                        return false;
                    }
                }
                true
            });
        } else {
            let c = cards[0];
            for i in 0..5 {
                if c[i].iter().all(|f| f.crossed) || c.iter().map(|row| row[i]).all(|f| f.crossed) {
                    let total: i32 = cards[0]
                        .iter()
                        .flat_map(|r| r.map(|f| if f.crossed { 0 } else { f.n }))
                        .sum();
                    return total * n;
                }
            }
        }
    }
    0
}
