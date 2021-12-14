#![feature(array_windows, iter_zip)]

use std::{collections::HashMap, error::Error, fmt, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day14")?;
    let mut lines = input.split_terminator('\n');
    let templates = Vec::from_iter(lines.by_ref().take_while(|&line| !line.is_empty()));
    let template = match &templates[..] { &[t] => t.as_bytes(), _ => Err(ParseRuleError)? };
    let rules = <Result<HashMap<_, _>, ParseRuleError>>::from_iter(lines.map(|rule| {
        let (left, right) = rule.split_once(" -> ").ok_or(ParseRuleError)?;
        let left = match left.as_bytes() { &[a, b] => [a, b], _ => Err(ParseRuleError)? };
        let right = match right.as_bytes() { &[c] => c, _ => Err(ParseRuleError)? };
        Ok((left, right))
    }))?;

    let mut pairs = HashMap::new();
    for &pair in template.array_windows() { *pairs.entry(pair).or_insert(0usize) += 1; }

    for _ in 0..40 {
        let mut new = HashMap::new();
        for ([a, b], count) in pairs {
            let c = rules[&[a, b]];
            *new.entry([a, c]).or_insert(0) += count;
            *new.entry([c, b]).or_insert(0) += count;
        }
        pairs = new;
    }

    let mut elements = HashMap::new();
    for ([a, b], count) in pairs {
        *elements.entry(a).or_insert(0) += count;
        *elements.entry(b).or_insert(0) += count;
    }
    if let &[b, .., a] = template {
        *elements.entry(a).or_insert(0) += 1;
        *elements.entry(b).or_insert(0) += 1;
    }
    let &min = elements.values().min().unwrap_or(&0);
    let &max = elements.values().max().unwrap_or(&0);
    println!("{}", max / 2 - min / 2);

    Ok(())
}

#[derive(Debug)]
pub struct ParseRuleError;

impl Error for ParseRuleError {}

impl fmt::Display for ParseRuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseRuleError")
    }
}
