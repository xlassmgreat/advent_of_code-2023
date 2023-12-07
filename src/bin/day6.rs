use std::io::{stdin, BufRead};

use itertools::Itertools;

fn main() {
    let dat = stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    p1(&dat);
    p2(&dat);
}

fn p1(dat: &Vec<String>) {
    let (time, distance) = dat
        .iter()
        .map(|x| {
            x.split_ascii_whitespace()
                .skip(1)
                .map(|s| s.parse::<f64>().unwrap())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();

    let ways = time
        .into_iter()
        .zip(distance)
        .map(|(t, d)| {
            let ubound = (t + (t.powi(2) - 4.0 * d).sqrt()) / 2.0;
            let (ubound, lbound) = ((ubound - f64::EPSILON).floor(), (t - ubound + f64::EPSILON).ceil());
            (ubound.floor() - lbound.ceil() + 1.0).floor() as u64
        })
        .fold(1, |acc, x| acc * x);

    println!("1: {ways}");
}

fn p2(dat: &Vec<String>) {
    let (time, distance) = dat
        .iter()
        .map(|x| {
            x.split_ascii_whitespace()
                .skip(1)
                .fold(String::new(), |acc, x| acc + x)
                .parse::<f64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let ubound = (time + (time.powi(2) - 4.0 * distance).sqrt()) / 2.0;
    let (ubound, lbound) = (
        (ubound - f64::EPSILON).floor(),
        (time - ubound + f64::EPSILON).ceil(),
    );
    let ways = (ubound.floor() - lbound.ceil() + 1.0).floor() as u64;

    println!("2: {ways}");
}
