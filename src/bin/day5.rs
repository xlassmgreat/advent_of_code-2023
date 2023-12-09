use itertools::Itertools;
use std::io::{stdin, Read};

struct SeedMap {
    src: i64,
    dest: i64,
    len: i64,
}

fn p1(dat: &Vec<&str>) {
    let seeds = dat[0]
        .split_ascii_whitespace()
        .filter_map(|c| c.parse().ok())
        .collect_vec();

    let grid = dat[1..]
        .iter()
        .map(|sec| {
            sec.lines()
                .skip(1)
                .map(|line| {
                    let (dest, src, len) = line
                        .split_ascii_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect_tuple()
                        .unwrap();
                    SeedMap { src, dest, len }
                })
                .collect_vec()
        })
        .collect_vec();

    let min = seeds
        .into_iter()
        .map(|seed| {
            grid.iter().fold(seed, |seed, sec| {
                sec.iter()
                    .find(|mp| (mp.src..=(mp.src + mp.len)).contains(&seed))
                    .map(|mp| mp.dest - mp.src + seed)
                    .unwrap_or(seed)
            })
        })
        .min()
        .unwrap();

    println!("1: {min}");
}

fn p2(dat: &Vec<&str>) {
    let seeds = dat[0]
        .split_ascii_whitespace()
        .filter_map(|c| c.parse::<i64>().ok())
        .chunks(2)
        .into_iter()
        .map(|mut sr| {
            let st = sr.next().unwrap();
            let len = sr.next().unwrap();
            (st, st + len)
        })
        .collect_vec();

    let grid = dat[1..]
        .iter()
        .map(|sec| {
            sec.lines()
                .skip(1)
                .map(|line| {
                    let (dest, src, len) = line
                        .split_ascii_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect_tuple()
                        .unwrap();
                    SeedMap { src, dest, len }
                })
                .collect_vec()
        })
        .collect_vec();

    let min = grid
        .into_iter()
        .fold(seeds, |seeds, sec| {
            seeds
                .iter()
                .map(|seed| {
                    let mut parts = sec
                        .iter()
                        .filter_map(|mp| {
                            let (a, b) = seed;
                            let (c, d) = (mp.src, mp.src + mp.len);

                            // overlap checking
                            let diff = mp.dest - mp.src;
                            let start = *a.max(&c);
                            let end = *b.min(&d);

                            if start < end {
                                Some(((start, end), (start + diff, end + diff)))
                            } else {
                                None
                            }
                        })
                        .collect_vec();

                    // required for the loop below to work correctly
                    parts.push(((seed.0, seed.0), (seed.0, seed.0)));
                    parts.push(((seed.1, seed.1), (seed.1, seed.1)));

                    parts.sort();
                    let mut parts = parts
                        .iter()
                        .tuple_windows()
                        .map(|(a, b)| {
                            let ((srca, _desta), (srcb, destb)) = (a, b);
                            let mut ret = Vec::new();

                            if srca.1 < srcb.0 {
                                ret.push((srca.1, srcb.0))
                            }

                            ret.push(destb.clone());
                            ret
                        })
                        .flatten()
                        .collect_vec();

                    // the last element with be a dummy that was inserted at the beginning
                    parts.pop();

                    parts.into_iter()
                })
                .flatten()
                .collect_vec()
        })
        .into_iter()
        .map(|(l, _)| l)
        .min()
        .unwrap();

    println!("2: {min}");
}

fn main() {
    let mut dat = Vec::new();
    stdin().read_to_end(&mut dat).unwrap();
    let dat = String::from_utf8_lossy(&dat);
    let dat = dat.split("\n\n").collect_vec();

    p1(&dat);
    p2(&dat);
}
