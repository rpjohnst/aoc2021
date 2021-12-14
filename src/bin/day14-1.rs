#![feature(array_windows, iter_zip)]

use std::{collections::HashMap, error::Error, fmt, fs, iter};

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

    let mut polymer = Vec::from(template);
    for _ in 0..10 {
        let insertions = polymer.array_windows().map(|pair| rules[pair]);
        polymer = Vec::from_iter(Iterator::chain(
            iter::once(polymer[0]),
            iter::zip(insertions, &polymer[1..]).flat_map(|(i, &e)| [i, e]),
        ));
    }

    let mut elements = HashMap::new();
    for &element in &polymer[..] { *elements.entry(element).or_insert(0usize) += 1; }
    let &min = elements.values().min().unwrap_or(&0);
    let &max = elements.values().max().unwrap_or(&0);
    println!("{}", max - min);

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
