use std::collections::HashMap;
use std::fs;

fn parse_line(s: &str) -> (i32, i32) {
    let mut iter = s.split_whitespace();
    (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    )
}

fn part_1() -> Result<(), std::io::Error> {
    let s = fs::read_to_string("src/input.txt")?;
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    s.lines()
        .map(parse_line)
        .map(|(a, b)| {
            l1.push(a);
            l2.push(b)
        })
        .count(); // O(n)
    l1.sort(); // O(n*log(n))
    l2.sort(); // O(n*log(n))
    let difference: i32 = l1
        .iter()
        .zip(l2.iter())
        .map(|(&a, &b)| i32::abs(a - b))
        .sum(); // O(n)
    println!("Difference: {difference}");
    Ok(())
}

fn part_2() -> Result<(), std::io::Error> {
    let s = fs::read_to_string("src/input.txt")?;
    let mut l1 = Vec::new();
    let mut h2 = HashMap::new();
    s.lines()
        .map(parse_line)
        .map(|(a, b)| {
            l1.push(a);
            h2.insert(b, h2.get(&b).unwrap_or(&0i32) + 1)
        })
        .count(); // O(n)
    let similarity: i32 = l1.iter().map(|a| h2.get(a).unwrap_or(&0) * a).sum(); // O(n)
    println!("Similarity: {similarity}");
    Ok(())
}

// Could use bufreader to optimize large file reads,
// but input is only 1000 short lines so no point
fn main() -> Result<(), std::io::Error> {
    part_1()?;
    part_2()?;
    Ok(())
}
