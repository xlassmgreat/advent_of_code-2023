use std::io::stdin;

use itertools::Itertools;

fn p1(dat: &Vec<Vec<Vec<i32>>>) {
    let sum = dat
        .iter()
        .map(|tri| {
            tri.iter()
                .rev()
                .fold(0, |acc, step| step.last().unwrap() + acc)
        })
        .sum::<i32>();

    println!("1: {sum}");
}

fn p2(dat: &Vec<Vec<Vec<i32>>>) {
    let sum = dat
        .iter()
        .map(|tri| {
            tri.iter()
                .rev()
                .fold(0, |acc, step| step.first().unwrap() - acc)
        })
        .sum::<i32>();

    println!("2: {sum}");
}

fn main() {
    let dat = stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect_vec()
        })
        .map(|seq| {
            let mut acc = vec![seq.clone()];

            let mut i = 0;
            loop {
                let new = acc[i]
                    .iter()
                    .tuple_windows()
                    .map(|(&x, &y)| y - x)
                    .collect_vec();
                if !new.iter().all(|&v| v == 0) {
                    acc.push(new);
                } else {
                    break;
                }
                i += 1;
            }

            acc
        })
        .collect_vec();

    p1(&dat);
    p2(&dat);
}
