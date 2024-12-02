use std::fs;

fn parse_line(s: &str) -> (i32, i32) {
    let mut iter = s.split_whitespace();
    (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    )
}

// could use bufreader to optimize large file reads,
// but input is only 1000 lines so no point
fn main() -> Result<(), std::io::Error> {
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
