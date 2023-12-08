use std::io::stdin;

use itertools::Itertools;

fn p1(dat: &Vec<(String, u32)>) {
    //same as part 2, but base 15 is used

    let sum = dat
        .into_iter()
        .map(|(hand, bid)| {
            let hand = hand
                .chars()
                .map(|c| match c {
                    'A' => 'E',
                    'K' => 'D',
                    'Q' => 'C',
                    'J' => 'B',
                    'T' => 'A',
                    _ => c,
                })
                .collect::<String>();

            (hand, bid)
        })
        .sorted_by_cached_key(|(hand, _)| {
            let mut ord = hand
                .chars()
                .sorted_unstable()
                .dedup_with_count()
                .map(|(count, _)| count)
                .sorted_unstable()
                .rev()
                .take(2);

            let ord = ord.next().unwrap() * 15 + ord.next().unwrap_or(0);

            let tie = u32::from_str_radix(hand, 15).unwrap();

            ord as u32 * 15u32.pow(5) + tie
        })
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u32 + 1) * bid)
        .sum::<u32>();

    println!("1: {sum}");
}

fn p2(dat: &Vec<(String, u32)>) {
    let sum = dat
        .into_iter()
        .map(|(hand, bid)| {
            let hand = hand
                .chars()
                .map(|c| match c {
                    // corresponds to base 14
                    'A' => 'D',
                    'K' => 'C',
                    'Q' => 'B',
                    'J' => '0',
                    'T' => 'A',
                    _ => c,
                })
                .collect::<String>();

            (hand, bid)
        })
        .sorted_by_cached_key(|(hand, _)| {
            let mut counts = hand
                .chars()
                .sorted_unstable()
                .dedup_with_count()
                .collect_vec();

            // the only time j is not converted is if the entire hand is j, so map it to 0
            if let Some(j) = counts.iter().position(|(_, c)| *c == '0') {
                let max = counts
                    .iter()
                    .position_max_by_key(|(c, ch)| if *ch == '0' { 0u32 } else { *c as u32 })
                    .unwrap();

                let jv = counts[j].0;
                counts[max].0 += jv;
                counts[j].0 -= jv;
            }

            // not the most efficient way to get the two max values, but it works
            let mut ord = counts
                .into_iter()
                .map(|(count, _)| count)
                .sorted_unstable()
                .rev()
                .take(2);

            // take the first two most repeated digits as base 14
            let ord = ord.next().unwrap() * 14 + ord.next().unwrap_or(0);

            let tie = u32::from_str_radix(hand, 14).unwrap();

            ord as u32 * 14u32.pow(5) + tie // using base 14 to make sure ord is significant
        })
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u32 + 1) * bid)
        .sum::<u32>();

    println!("2: {sum}");
}
fn main() {
    let dat = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (hand, bid) = line.split_ascii_whitespace().collect_tuple().unwrap();
            (hand.to_string(), bid.parse().unwrap())
        })
        .collect_vec();

    p1(&dat);
    p2(&dat);
}
