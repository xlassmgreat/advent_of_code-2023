use itertools::{Itertools, MinMaxResult};
use std::io::{stdin, BufRead, BufReader};

// Wont bother with part 2 because the problem is essentially a subset of part 2
fn main() {
    let nums = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let i = BufReader::new(stdin())
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let rep = nums
                .iter()
                .enumerate()
                .map(|(i, n)| [(i, x.find(n)), (i, x.rfind(n))].into_iter())
                .flatten()
                .filter_map(|(i, n)| {
                    if let Some(n) = n {
                        Some((n, i as u32))
                    } else {
                        None
                    }
                });

            let it = x.chars().enumerate().filter_map(|(i, x)| {
                if let Some(x) = x.to_digit(10) {
                    Some((i, x))
                } else {
                    None
                }
            });

            match it.chain(rep).minmax_by_key(|(b, _)| *b) {
                MinMaxResult::NoElements => 0,
                MinMaxResult::OneElement((_, d)) => d * 10 + d,
                MinMaxResult::MinMax((_, da), (_, db)) => da * 10 + db,
            }
        })
        .sum::<u32>();

    println!("{i}");
}
