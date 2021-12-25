#![feature(array_chunks, let_else, int_abs_diff)]

use std::{cmp, collections::{BinaryHeap, HashMap}, error::Error, fmt, fs, mem};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day23")?;
    let mut lines = input.split_terminator('\n');
    match lines.next() { Some("#############") => (), _ => Err(ParseRoomError)? }
    match lines.next() { Some("#...........#") => (), _ => Err(ParseRoomError)? }
    let row = lines.next().ok_or(ParseRoomError)?;
    let top = match *row.as_bytes() {
        [b'#', b'#', b'#', w, b'#', x, b'#', y, b'#', z, b'#', b'#', b'#'] => [w, x, y, z],
        _ => Err(ParseRoomError)?,
    };
    let row = lines.next().ok_or(ParseRoomError)?;
    let bot = match *row.as_bytes() {
        [b' ', b' ', b'#', w, b'#', x, b'#', y, b'#', z, b'#'] => [w, x, y, z],
        _ => Err(ParseRoomError)?,
    };
    match lines.next() { Some("  #########") => (), _ => Err(ParseRoomError)? }
    match lines.next() { None => (), _ => Err(ParseRoomError)? }

    let mut cells = [b'.'; YS * 4 + 7];
    for (r, p) in top.into_iter().enumerate() { cells[YS * r + 0] = p; }
    #[cfg(feature = "part2")]
    {
        for (r, p) in (*b"DCBA").into_iter().enumerate() { cells[YS * r + 1] = p; }
        for (r, p) in (*b"DBAC").into_iter().enumerate() { cells[YS * r + 2] = p; }
    }
    for (r, p) in bot.into_iter().enumerate() { cells[YS * r + YS - 1] = p; }

    let mut cost = HashMap::new();
    let mut heap = BinaryHeap::default();

    cost.insert(cells, 0);
    heap.push(cmp::Reverse(State { cells, c: 0 }));

    while let Some(cmp::Reverse(State { cells, c })) = heap.pop() {
        #[cfg(not(feature = "part2"))]
        const DONE: &[u8; YS * 4] = b"AABBCCDD";
        #[cfg(feature = "part2")]
        const DONE: &[u8; YS * 4] = b"AAAABBBBCCCCDDDD";
        if &cells[..YS * 4] == DONE { println!("{}", c); break; }
        if cost[&cells] < c { continue; }

        for r in 0..4 {
            let Some(y) = (0..YS).position(|y| cells[YS * r + y] != b'.') else { continue; };
            let p = (cells[YS * r + y] - b'A') as u32;
            for h in Iterator::chain(
                (0..2 + r).rev().take_while(|&h| cells[YS * 4 + h] == b'.'),
                (2 + r..7).take_while(|&h| cells[YS * 4 + h] == b'.'),
            ) {
                let mut cells = cells;
                cells[YS * 4 + h] = mem::replace(&mut cells[YS * r + y], b'.');
                let dx = u64::abs_diff(RX[r], HX[h]);
                let dy = 1 + y as u64;
                let c = c + u64::pow(10, p) * (dx + dy);
                if c < *cost.get(&cells).unwrap_or(&u64::MAX) {
                    cost.insert(cells, c);
                    heap.push(cmp::Reverse(State { cells, c }));
                }
            }
        }
        for h in (0..7).filter(|&h| cells[YS * 4 + h] != b'.') {
            let p = cells[YS * 4 + h];
            let r = (p - b'A') as usize;
            let Some(y) = (0..YS).rev().find(|&y| cells[YS * r + y] == b'.') else { continue; };
            if cells[YS * r + y + 1..YS * r + YS].iter().any(|&c| c != p) { continue; }
            if HX[h] < RX[r] && cells[YS * 4..][h + 1..2 + r].iter().any(|&c| c != b'.') { continue; }
            if RX[r] < HX[h] && cells[YS * 4..][2 + r..h].iter().any(|&c| c != b'.') { continue; }

            let mut cells = cells;
            cells[YS * r + y] = mem::replace(&mut cells[YS * 4 + h], b'.');
            let dx = u64::abs_diff(HX[h], RX[r]);
            let dy = 1 + y as u64;
            let c = c + u64::pow(10, r as u32) * (dx + dy);
            if c < *cost.get(&cells).unwrap_or(&u64::MAX) {
                cost.insert(cells, c);
                heap.push(cmp::Reverse(State { cells, c }));
            }
        }
    }

    Ok(())
}

const HX: [u64; 7] = [0, 1, 3, 5, 7, 9, 10];
const RX: [u64; 4] = [2, 4, 6, 8];
#[cfg(not(feature = "part2"))]
const YS: usize = 2;
#[cfg(feature = "part2")]
const YS: usize = 4;

#[derive(Eq, PartialEq)]
struct State { cells: [u8; 4 * YS + 7], c: u64 }

impl State {
    fn h(&self) -> u64 {
        let mut estimate = 0;
        for r in 0..4 {
            for y in (0..2).skip_while(|&y| self.cells[2 * r + y] == b'.') {
                let p = self.cells[2 * r + y] - b'A';
                estimate += u64::pow(10, p as u32) * u64::abs_diff(RX[r], RX[p as usize]);
            }
        }
        for h in (0..7).filter(|&h| self.cells[2 * 4 + h] != b'.') {
            let p = self.cells[2 * 4 + h] - b'A';
            estimate += u64::pow(10, p as u32) * u64::abs_diff(HX[h], RX[p as usize]);
        }
        estimate
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let d = self.c + self.h();
        let e = other.c + other.h();
        u64::cmp(&d, &e)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> { Some(Ord::cmp(self, other)) }
}

#[derive(Debug)]
pub struct ParseRoomError;

impl Error for ParseRoomError {}

impl fmt::Display for ParseRoomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseRoomError")
    }
}
