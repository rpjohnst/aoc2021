use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day03")?;
    let report: Vec<_> = Result::from_iter(
        input.split_terminator('\n').map(|n| usize::from_str_radix(n, 2)))?;

    let mut oxygen = report.clone();
    for i in (0..12).rev() {
        if oxygen.len() > 1 {
            let ones: usize = oxygen.iter().map(|&diag| (diag >> i) & 1).sum();
            let bit = 2 * ones >= oxygen.len();
            oxygen.retain(|&diag| ((diag >> i) & 1) == bit as usize);
        }
    }

    let mut co2 = report.clone();
    for i in (0..12).rev() {
        if co2.len() > 1 {
            let ones: usize = co2.iter().map(|&diag| (diag >> i) & 1).sum();
            let bit = 2 * ones < co2.len();
            co2.retain(|&diag| ((diag >> i) & 1) == bit as usize);
        }
    }

    println!("{}", oxygen[0] * co2[0]);

    Ok(())
}
