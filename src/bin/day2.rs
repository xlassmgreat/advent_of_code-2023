use std::io::{stdin, BufRead, BufReader}; 
use itertools::Itertools;

fn p1(dat: &Vec<String>) {
    let red = 12;
    let green = 13;
    let blue = 14;

    let sum = dat.into_iter()
        .map(|line| {
            let mut l = line.splitn(2, ": ");
            let gid = l
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let sets = l
                .next()
                .unwrap()
                .split("; ")
                .map(|s| {
                    let mut r = 0;
                    let mut g = 0;
                    let mut b = 0;

                    for w in s.split(", ") {
                        let mut x = w.split(' ');
                        let n = x.next().unwrap().parse::<u32>().unwrap();
                        match x.next().unwrap() {
                            "red" => r += n,
                            "green" => g += n,
                            "blue" => b += n,
                            _ => panic!("Odd"),
                        }
                    }

                    (r, g, b)
                })
                .collect_vec();

            (gid, sets)
        })
        .filter(|(_, sets)| {
            sets.into_iter()
                .filter(|(r, g, b)| *r > red || *g > green || *b > blue)
                .count()
                == 0
        })
        .map(|(gid, _)| gid)
        .sum::<u32>();

    println!("1: {sum}");
}

fn p2(dat: &Vec<String>) {
    let sum = dat.into_iter()
        .map(|line| {
            let sets = line
                .splitn(2, ": ")
                .last()
                .unwrap()
                .split("; ")
                .map(|s| {
                    let mut r = 0;
                    let mut g = 0;
                    let mut b = 0;

                    for w in s.split(", ") {
                        let mut x = w.split(' ');
                        let n = x.next().unwrap().parse::<u32>().unwrap();
                        match x.next().unwrap() {
                            "red" => r += n,
                            "green" => g += n,
                            "blue" => b += n,
                            _ => panic!("Odd"),
                        }
                    }

                    (r, g, b)
                })
                .multiunzip();

            let max = |l: Vec<u32>| l.into_iter().max().unwrap();
            max(sets.0) * max(sets.1) * max(sets.2)
        })
        .sum::<u32>();

    println!("2: {sum}");
}

fn main() {
    let dat = stdin().lock().lines().collect::<Result<Vec<_>, _>>().unwrap();
    p1(&dat);
    p2(&dat);
}
