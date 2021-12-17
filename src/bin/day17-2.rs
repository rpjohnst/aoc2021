use std::{cmp, error::Error, fs, fmt, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day17")?;
    let input = input.strip_prefix("target area: x=").ok_or(ParseTargetError)?;
    let (x1, input) = input.split_once("..").ok_or(ParseTargetError)?;
    let x1 = i32::from_str(x1)?;
    let (x2, input) = input.split_once(", y=").ok_or(ParseTargetError)?;
    let x2 = i32::from_str(x2)?;
    let (y1, input) = input.split_once("..").ok_or(ParseTargetError)?;
    let y1 = i32::from_str(y1)?;
    let (y2, input) = input.split_once("\n").ok_or(ParseTargetError)?;
    let y2 = i32::from_str(y2)?;
    match input { "" => {}, _ => Err(ParseTargetError)? };

    let mut count = 0;
    let vx1 = f32::ceil((f32::sqrt(1.0 + 8.0 * x1 as f32) - 1.0) / 2.0) as i32;
    let vx2 = x2;
    let vy1 = y1;
    let vy2 = -y1 - 1;
    for vy in vy1..=vy2 {
        for vx in vx1..=vx2 {
            let (mut x, mut y) = (0, 0);
            let (mut vx, mut vy) = (vx, vy);
            loop {
                x += vx;
                y += vy;
                vx = cmp::max(0, vx - 1);
                vy -= 1;
                if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
                    count += 1;
                    break;
                }
                if x > x2 || y < y1 { break; }
            }
        }
    }
    println!("{}", count);

    Ok(())
}

#[derive(Debug)]
pub struct ParseTargetError;

impl Error for ParseTargetError {}

impl fmt::Display for ParseTargetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseBitsError")
    }
}
