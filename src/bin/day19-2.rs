use std::{collections::{HashSet, VecDeque}, error::Error, fmt, fs, str::FromStr, num::ParseIntError};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day19")?;
    let mut lines = input.split_terminator('\n');
    let mut scanners = Vec::default();
    while let Some(scanner) = lines.next() {
        let scanner = scanner.strip_prefix("--- scanner ").ok_or(ParseScannerError)?;
        let scanner = scanner.strip_suffix(" ---").ok_or(ParseScannerError)?;
        let scanner = usize::from_str(scanner)?;

        let mut beacons = Vec::default();
        while let Some(beacon) = lines.next() {
            if beacon.is_empty() { break; }

            let mut beacon = beacon.split(',');
            let x = i32::from_str(beacon.next().ok_or(ParseScannerError)?)?;
            let y = i32::from_str(beacon.next().ok_or(ParseScannerError)?)?;
            let z = i32::from_str(beacon.next().ok_or(ParseScannerError)?)?;
            beacons.push([x, y, z]);
        }

        if scanner != scanners.len() { Err(ParseScannerError)?; }
        scanners.push(beacons);
    }

    let orientations = (
        [(1, 0), (1, 1), (1, 2), (-1, 0), (-1, 1), (-1, 2)]
    ).into_iter().flat_map(|(s, i)| {
        let j = (i + 1) % 3;
        let k = (i + 2) % 3;
        [(s, i, 1, j, s, k), (s, i, 1, k, -s, j), (s, i, -1, j, -s, k), (s, i, -1, k, s, j)]
    });

    let mut positions = Vec::new();
    positions.push([0, 0, 0]);
    let mut pending = VecDeque::default();
    pending.push_back(<HashSet<_>>::from_iter(scanners.remove(0)));
    while let Some(cs) = pending.pop_front() {
        let mut b = 0;
        'scanners: while b < scanners.len() {
            let bs = &scanners[b];
            for (s, i, t, j, u, k) in orientations.clone() {
                let bs = Vec::from_iter(bs.iter().map(|b| [s * b[i], t * b[j], u * b[k]]));
                for [dx, dy, dz] in cs.iter().flat_map(|&[cx, cy, cz]|
                    bs.iter().map(move |&[bx, by, bz]| [cx - bx, cy - by, cz - bz])
                ) {
                    let bs = bs.iter().map(|&[bx, by, bz]| [bx + dx, by + dy, bz + dz]);
                    if bs.clone().filter(|b| cs.contains(b)).count() >= 12 {
                        pending.push_back(HashSet::from_iter(bs));
                        scanners.remove(b);
                        positions.push([dx, dy, dz]);
                        continue 'scanners;
                    }
                }
            }
            b += 1;
        }
    }

    let mut max = 0;
    for &[ax, ay, az] in &positions[..] {
        for &[bx, by, bz] in &positions[..] {
            let m = i32::abs(bx - ax) + i32::abs(by - ay) + i32::abs(bz - az);
            if m > max { max = m; }
        }
    }
    println!("{}", max);

    Ok(())
}

#[derive(Debug)]
pub struct ParseScannerError;

impl From<ParseIntError> for ParseScannerError {
    fn from(_: ParseIntError) -> Self { ParseScannerError }
}

impl Error for ParseScannerError {}

impl fmt::Display for ParseScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseScannerError")
    }
}
