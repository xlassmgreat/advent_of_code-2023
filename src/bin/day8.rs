use std::{collections::HashMap, io::stdin};

use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

type Point = (char, char, char);
type TPoints = (Point, Point);

fn lcm(a: u64, b: u64) -> u64 {
    let (mut x, mut y) = (a, b);
    let gcd = loop {
        if y == 0 {
            break x;
        } else {
            let rem = x % y;
            x = y;
            y = rem;
        }
    };

    a * b / gcd
}

fn p1(dir: &Vec<bool>, points: &HashMap<Point, TPoints>) {
    let mut count = 0u32;
    dir.iter()
        .cycle()
        .fold_while(points[&('A', 'A', 'A')], |point, &r| {
            count += 1;
            let key = if r { point.1 } else { point.0 };

            if key == ('Z', 'Z', 'Z') {
                Done(point)
            } else {
                Continue(points[&key])
            }
        });

    println!("1: {count}");
}

fn p2(dir: &Vec<bool>, points: &HashMap<Point, TPoints>) {
    let count = points
        .into_iter()
        .filter(|((_, _, e), _)| *e == 'A')
        .map(|(a, _)| *a)
        .map(|start| {
            let mut count = 0u64;

            dir.iter().cycle().fold_while(points[&start], |point, &r| {
                count += 1;
                let key = if r { point.1 } else { point.0 };

                if key.2 == 'Z' {
                    Done(point)
                } else {
                    Continue(points[&key])
                }
            });
            count
        })
        .reduce(|acc, x| lcm(acc, x))
        .unwrap();

    println!("2: {count}");
}

fn main() {
    let mut lines = stdin().lines();
    let dir = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| if c == 'R' { true } else { false })
        .collect_vec();

    let points = lines
        .skip(1)
        .map(|line| {
            let (src, left, right): (Point, Point, Point) = line
                .unwrap()
                .chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .chunks(3)
                .into_iter()
                .map(|point| point.collect_tuple().unwrap())
                .collect_tuple()
                .unwrap();
            (src, (left, right))
        })
        .collect::<HashMap<_, _>>();

    p1(&dir, &points);
    p2(&dir, &points);
}
