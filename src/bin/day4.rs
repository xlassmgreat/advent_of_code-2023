use std::{
    io::{stdin, BufRead},
    iter,
};

use itertools::Itertools;

fn p1(dat: &Vec<String>) {
    let sum = dat
        .into_iter()
        .map(|line| {
            let (win, have) = line
                .splitn(2, ": ")
                .last()
                .unwrap()
                .splitn(2, " | ")
                .map(|sec| {
                    sec.split_ascii_whitespace()
                        .fold(0, |acc, n| acc | (1 << n.parse::<u32>().unwrap()))
                })
                .collect_tuple()
                .unwrap();

            2u32.pow((win & have as u128).count_ones() - 1)
        })
        .sum::<u32>();

    println!("{sum}");
}

fn p2(dat: &Vec<String>) {
    let mut count = iter::repeat(1).take(dat.len()).collect_vec();

    for (i, line) in dat.into_iter().enumerate() {
        let (win, have) = line
            .splitn(2, ": ")
            .last()
            .unwrap()
            .splitn(2, " | ")
            .map(|sec| {
                sec.split_ascii_whitespace()
                    .fold(0, |acc, n| acc | (1 << n.parse::<u32>().unwrap()))
            })
            .collect_tuple()
            .unwrap();

        let wins = (win & have as u128).count_ones() as usize;
        let add = count[i];

        for n in (i + 1)..=(i + wins) {
            count.get_mut(n).map(|c| *c += add);
        }
    }

    let sum = count.iter().sum::<usize>();
    println!("2: {sum}");
}

fn main() {
    let dat = stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    p1(&dat);
    p2(&dat);
}
