use std::iter::zip;

use aoc24::get_lines;
use aoc24::ToInt;

fn main() -> std::io::Result<()> {
    let lines = get_lines("inputs/1.txt")?;
    part_one(&lines).unwrap();
    part_two(&lines).unwrap();
    Ok(())
}

fn part_one(lines: &[String]) -> std::io::Result<()> {
    let mut lists = lines
        .iter()
        .map(|line| line.split_whitespace())
        .map(|mut a| (a.next().unwrap().to_int(), a.next().unwrap().to_int()))
        .collect::<(Vec<_>, Vec<_>)>();

    lists.0.sort();
    lists.1.sort();

    let res = zip(lists.0, lists.1)
        .map(|(a, b)| (a).abs_diff(b))
        .sum::<u32>();

    println!("Part 1: {}", res);
    Ok(())
}

fn part_two(lines: &[String]) -> std::io::Result<()> {
    let mut lists = lines
        .iter()
        .map(|line| line.split_whitespace())
        .map(|mut a| (a.next().unwrap().to_int(), a.next().unwrap().to_int()))
        .collect::<(Vec<_>, Vec<_>)>();

    lists.0.sort();
    lists.1.sort();
    let res = lists
        .0
        .iter()
        .map(|x| lists.1.iter().filter(|y| (*y).eq(x)).count() as u32 * (*x))
        .sum::<u32>();

    println!("Part 2: {}", res);

    Ok(())
}
