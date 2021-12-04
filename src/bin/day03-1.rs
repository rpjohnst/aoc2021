use std::{error::Error, fs, iter};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day03")?;
    let report: Vec<_> = Result::from_iter(
        input.split_terminator('\n').map(|n| usize::from_str_radix(n, 2)))?;

    let mut ones = Vec::from_iter(iter::repeat(0).take(12));
    for &diag in &report[..] {
        for i in 0..12 { ones[i] += (diag >> i) & 1; }
    }

    let mut gamma = 0;
    for (i, bit) in ones.into_iter().map(|ones| 2 * ones >= report.len()).enumerate() {
        gamma |= (bit as usize) << i;
    }

    let epsilon = !gamma & 0b1111_1111_1111;

    println!("{}", gamma * epsilon);

    Ok(())
}
