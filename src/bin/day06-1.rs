use std::{error::Error, fs, iter, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day06")?;
    let ages: Vec<_> = Result::from_iter(input.split_terminator(&[',', '\n']).map(u32::from_str))?;

    let mut ages = ages;
    for _ in 0..80 {
        let mut new = 0;
        for age in &mut ages[..] {
            if *age == 0 {
                *age = 6;
                new += 1;
            } else {
                *age -= 1;
            }
        }
        ages.extend(iter::repeat(8).take(new));
    }
    println!("{}", ages.len());

    Ok(())
}
