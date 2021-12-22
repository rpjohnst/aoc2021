use std::{cmp, error::Error, fs, iter, str::FromStr};
use aoc2021::day22::Step;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day22")?;
    let steps = <Result<Vec<_>, _>>::from_iter(input.split_terminator('\n').map(Step::from_str))?;

    let len = (-50..=50).count();
    let mut reactor = Vec::from_iter(iter::repeat(false).take(len * len * len));
    let planes = reactor.chunks_exact_mut(len * len);
    let mut cells = Vec::from_iter(planes.map(|r| Vec::from_iter(r.chunks_exact_mut(len))));

    for Step { on, x, y, z } in steps {
        for z in cmp::max(-50, z.start)..=cmp::min(z.end, 50) {
            for y in cmp::max(-50, y.start)..=cmp::min(y.end, 50) {
                for x in cmp::max(-50, x.start)..=cmp::min(x.end, 50) {
                    cells[(z + 50) as usize][(y + 50) as usize][(x + 50) as usize] = on;
                }
            }
        }
    }
    println!("{}", reactor.into_iter().filter(|&x| x).count());

    Ok(())
}
