use std::{collections::VecDeque, error::Error, fs, iter, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day06")?;
    let ages: Vec<_> = Result::from_iter(input.split_terminator(&[',', '\n']).map(u32::from_str))?;

    let mut groups = VecDeque::from_iter(iter::repeat(0usize).take(9));
    for &age in &ages[..] { groups[age as usize] += 1; }
    for _ in 0..256 {
        if let Some(zero) = groups.pop_front() {
            groups[6] += zero;
            groups.push_back(zero);
        }
    }
    println!("{}", groups.into_iter().sum::<usize>());

    Ok(())
}
