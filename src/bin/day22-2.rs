use std::{collections::VecDeque, error::Error, fs, str::FromStr};
use aoc2021::day22::{Step, Axis};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day22")?;
    let steps = <Result<Vec<_>, _>>::from_iter(input.split_terminator('\n').map(Step::from_str))?;

    let mut regions = VecDeque::default();
    let mut queue = VecDeque::default();
    for Step { on, x, y, z } in steps {
        let region = Region { axes: [x, y, z] };
        if on {
            queue.push_back(region);
            for region in &regions {
                for _ in 0..queue.len() {
                    queue.pop_front().unwrap().subtract(region, &mut queue);
                }
            }
            regions.extend(queue.drain(..));
        } else {
            for _ in 0..regions.len() {
                regions.pop_front().unwrap().subtract(&region, &mut regions);
            }
        }
    }

    let sum: i64 = regions.into_iter().map(|region| -> i64 {
        region.axes.into_iter()
            .map(|Axis { start, end }| { i64::try_from((start..=end).count()).unwrap() })
            .product()
    }).sum();
    println!("{}", sum);

    Ok(())
}

struct Region { axes: [Axis; 3] }

impl Region {
    fn subtract(mut self, other: &Region, out: &mut VecDeque<Region>) {
        for i in 0..3 {
            if
                other.axes[i].end < self.axes[i].start ||
                self.axes[i].end < other.axes[i].start
            {
                out.push_back(self);
                return;
            }
        }
        for i in 0..3 {
            if
                self.axes[i].start < other.axes[i].start &&
                other.axes[i].start <= self.axes[i].end
            {
                let mut rest = Region { axes: self.axes };
                rest.axes[i].end = other.axes[i].start - 1;
                self.axes[i].start = other.axes[i].start;
                out.push_back(rest);
            }
            if
                self.axes[i].start <= other.axes[i].end &&
                other.axes[i].end < self.axes[i].end
            {
                let mut rest = Region { axes: self.axes };
                self.axes[i].end = other.axes[i].end;
                rest.axes[i].start = other.axes[i].end + 1;
                out.push_back(rest);
            }
        }
    }
}
