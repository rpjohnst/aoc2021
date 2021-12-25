use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day25")?;
    let mut width = 0;
    let mut cucumbers = Vec::from_iter(input.split_terminator('\n').flat_map(|line| {
        width = line.len();
        line.bytes()
    }));
    let mut grid = Vec::from_iter(cucumbers.chunks_mut(width));
    let height = grid.len();

    let mut steps = 0;
    let mut easts = Vec::default();
    let mut souths = Vec::default();
    loop {
        let mut moves = 0;

        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == b'>' && grid[y][(x + 1) % width] == b'.' { easts.push((x, y)); }
            }
        }
        moves += easts.len();
        for (x, y) in easts.drain(..) {
            grid[y][x] = b'.';
            grid[y][(x + 1) % width] = b'>';
        }

        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == b'v' && grid[(y + 1) % height][x] == b'.' { souths.push((x, y)); }
            }
        }
        moves += souths.len();
        for (x, y) in souths.drain(..) {
            grid[y][x] = b'.';
            grid[(y + 1) % height][x] = b'v';
        }

        steps += 1;
        if moves == 0 { break; }
    }
    println!("{}", steps);

    Ok(())
}
