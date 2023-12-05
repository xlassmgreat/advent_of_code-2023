use itertools::Itertools;
use std::io::{stdin, BufRead};

fn p1(grid: &Vec<Vec<char>>) {
    let sum = grid
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .group_by(|(_, c)| c.is_numeric())
                .into_iter()
                .filter_map(|(b, c)| if b { Some(c.collect_vec()) } else { None })
                .filter(|num| {
                    num.iter()
                        .find(|(j, _)| {
                            (-1i32..=1)
                                .filter_map(|x| grid.get(i + x as usize))
                                .map(|vec| (-1i32..=1).filter_map(|x| vec.get(j + x as usize)))
                                .flatten()
                                .find(|&&c| c != '.' && !c.is_numeric())
                                .is_some()
                        })
                        .is_some()
                })
                .map(|num| {
                    num.into_iter()
                        .map(|(_, n)| n)
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap()
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("1: {sum}");
}

fn p2(grid: &Vec<Vec<char>>) {
    let numbers = grid
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .group_by(|c| c.is_numeric())
                .into_iter()
                .map(|(b, gr)| {
                    if b {
                        let st = gr.collect::<String>();
                        st.chars()
                            .map(|_| Some((st.parse::<u32>().unwrap(), i)))
                            .collect_vec()
                    } else {
                        gr.map(|_| None).collect_vec()
                    }
                })
                .flatten()
                .collect_vec()
        })
        .collect_vec();

    let sum = grid
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, ch)| **ch == '*')
                .filter_map(|(j, _)| {
                    let mut nums = (-1i32..=1)
                        .filter_map(|x| numbers.get(i + x as usize))
                        .map(|vec| (-1i32..=1).filter_map(|x| vec.get(j + x as usize)))
                        .flatten()
                        .filter_map(|x| *x)
                        .collect_vec();

                    nums.dedup();
                    if nums.len() == 2 {
                        Some(nums[0].0 * nums[1].0)
                    } else {
                        None
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("2: {sum}");
}

fn main() {
    let grid = stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().chars().collect_vec())
        .collect_vec();
    p1(&grid);
    p2(&grid);
}
