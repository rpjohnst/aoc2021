#![feature(array_windows, iter_zip)]

use std::{cmp, collections::BinaryHeap, error::Error, fs, iter};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day15")?;
    let mut len = 0;
    let risk = Vec::from_iter(input.split_terminator('\n').flat_map(|line| {
        len = line.len();
        line.bytes().map(|x| x - b'0')
    }));
    let grid = Vec::from_iter(risk.chunks(len));

    let mut cost = Vec::from_iter(iter::repeat(usize::MAX).take(len * len));
    let mut cost = Vec::from_iter(cost.chunks_mut(len));
    let mut heap = BinaryHeap::default();

    cost[0][0] = 0;
    heap.push(cmp::Reverse(Path { mx: len - 1, my: len - 1, c: 0 }));

    while let Some(cmp::Reverse(Path { mx, my, c })) = heap.pop() {
        let (x, y) = (len - 1 - mx, len - 1 - my);
        if (x, y) == (len - 1, len - 1) { println!("{}", c); break; }
        if cost[y][x] < c { continue; }

        for (dx, dy) in [(1, 0), (0, -1), (-1, 0), (0, 1)] {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if nx < 0 || nx >= len as isize || ny < 0 || ny >= len as isize { continue; }
            let (nx, ny) = (nx as usize, ny as usize);

            let d = c + grid[ny][nx] as usize;
            if d < cost[ny][nx] {
                cost[ny][nx] = d;
                heap.push(cmp::Reverse(Path { mx: len - 1 - nx, my: len - 1 - ny, c: d }));
            }
        }
    }

    Ok(())
}

#[derive(Eq, PartialEq)]
struct Path { mx: usize, my: usize, c: usize }

impl Ord for Path {
    fn cmp(&self, other: &Path) -> cmp::Ordering {
        let d = self.c + self.mx + self.my;
        let e = other.c + other.mx + other.my;
        usize::cmp(&d, &e)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Path) -> Option<cmp::Ordering> { Some(Ord::cmp(self, other)) }
}
