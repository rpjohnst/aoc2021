#![feature(iter_zip)]

use std::{error::Error, fs, iter, str::FromStr};
use aoc2021::day08::{Entry, ParseEntryError};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day08")?;
    let entries: Vec<_> = Result::from_iter(input.split_terminator('\n').map(Entry::from_str))?;

    let mut sum = 0;
    for Entry { combos, digits } in entries {
        let mut displays = [0; 10];

        let mut fives = [0; 7];
        let mut sixes = [0; 7];
        for &set in &combos[..] {
            match set.count_ones() {
                2 => { displays[1] = set; }
                4 => { displays[4] = set; }
                3 => { displays[7] = set; }
                7 => { displays[8] = set; }
                5 => for i in 0..7 { fives[i] += (set >> i) & 1; }
                6 => for i in 0..7 { sixes[i] += (set >> i) & 1; }
                _ => { unreachable!(); }
            }
        }

        let mut segments = [0; 7];
        for (i, (n, m)) in iter::zip(fives, sixes).enumerate() {
            let bit = 1 << i as u8;
            match (n, m) {
                (3, 3) => { segments[0] |= bit; segments[6] |= bit; }
                (1, 3) => { segments[1] = bit; }
                (2, 2) => { segments[2] = bit; }
                (3, 2) => { segments[3] = bit; }
                (1, 2) => { segments[4] = bit; }
                (2, 3) => { segments[5] = bit; }
                (_, _) => { unreachable!(); }
            }
        }

        displays[2] = segments[0] | segments[2] | segments[3] | segments[4];
        displays[3] = segments[0] | segments[2] | segments[3] | segments[5];
        displays[5] = segments[0] | segments[1] | segments[3] | segments[5];

        displays[6] = segments[0] | segments[1] | segments[3] | segments[4] | segments[5];
        displays[9] = segments[0] | segments[1] | segments[2] | segments[3] | segments[5];
        displays[0] = segments[0] | segments[1] | segments[2] | segments[4] | segments[5];

        sum += digits.into_iter()
            .map(|set| displays.iter().position(|&x| x == set))
            .try_fold(0, |a, digit| Some(10 * a + digit?))
            .ok_or(ParseEntryError)?;
    }
    println!("{}", sum);

    Ok(())
}
