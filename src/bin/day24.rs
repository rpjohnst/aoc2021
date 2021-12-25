use std::{error::Error, fmt, fs, str::FromStr, num::ParseIntError};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day24")?;
    let monad = <Result<Vec<_>, _>>::from_iter(input.split_terminator('\n').map(Inst::from_str))?;

    // x1 = z0 % 26 + 14 != i1 = 1
    // z1 = (z0 / 1) * (25 * x1 + 1) + (i1 + 12) * x1 = 26 * z0 + i1 + 12

    // x2 = z1 % 26 + 13 != i2 = 1
    // z2 = (z1 / 1) * (25 * x2 + 1) + (i2 + 6) * x2 = 26 * z1 + i2 + 6

    // x3 = z2 % 26 + 12 != i3 = 1
    // z3 = (z2 / 1) * (25 * x3 + 1) + (i3 + 4) * x3 = 26 * z2 + i3 + 4

    // x4 = z3 % 26 + 14 != i4 = 1
    // z4 = (z3 / 1) * (25 * x4 + 1) + (i4 + 5) * x4 = 26 * z3 + i4 + 5

    // x5 = z4 % 26 + 13 != i5 = 1
    // z5 = (z4 / 1) * (25 * x5 + 1) + (i5 + 0) * x5 = 26 * z4 + i5 + 0

    // x6 = z5 % 26 - 7 != i6 = 0 if i5 == i6 + 7
    // z6 = (z5 / 26) * (25 * x6 + 1) + (i6 + 4) * x6 = z4

    // x7 = z6 % 26 - 13 != i7 = 0 if i4 == i7 + 8
    // z7 = (z6 / 26) * (25 * x7 + 1) + (i7 + 15) * x7 = z3

    // x8 = z7 % 26 + 10 != i8 = 1
    // z8 = (z7 / 1) * (25 * x8 + 1) + (i8 + 14) * x8 = 26 * z3 + i8 + 14

    // x9 = z8 % 26 - 7 != i9 = 0 if i8 == i9 - 7
    // z9 = (z8 / 26) * (25 * x9 + 1) + (i9 + 6) * x9 = z3

    // x10 = z9 % 26 + 11 != i10 = 1
    // z10 = (z9 / 1) * (25 * x10 + 1) + (i10 + 14) * x10 = 26 * z3 + i10 + 14

    // x11 = z10 % 26 - 9 != i11 = 0 if i10 == i11 - 5
    // z11 = (z10 / 26) * (25 * x11 + 1) + (i11 + 8) * x11 = z3

    // x12 = z11 % 26 - 2 != i12 = 0 if i3 == i12 - 2
    // z12 = (z11 / 26) * (25 * x12 + 1) + (i12 + 5) * x12 = z2

    // x13 = z12 % 26 - 9 != i13 = 0 if i2 == i13 + 3
    // z13 = (z12 / 26) * (25 * x13 + 1) + (i13 + 14) * x13 = z1

    // x14 = z13 % 26 - 14 != i14 = 0 if i1 == i14 + 2
    // z14 = (z13 / 26) * (25 * x14 + 1) + (i14 + 4) * x14 = z0

    // i6 = i5 - 7
    // i7 = i4 - 8
    // i9 = i8 + 7
    // i11 = i10 + 5
    // i12 = i3 + 2
    // i13 = i2 - 3
    // i14 = i1 - 2

    let inp1 = [9, 9, 7, 9, 9, 2, 1, 2, 9, 4, 9, 9, 6, 7].into_iter();
    let inp2 = [3, 4, 1, 9, 8, 1, 1, 1, 8, 1, 6, 3, 1, 1].into_iter();
    for mut inp in [inp1, inp2] {
        let mut regs = [0, 0, 0, 0];
        for &Inst { op, a, b } in &monad[..] {
            let a = a as usize;
            let b = match b {
                Val::Reg(b) => regs[b as usize],
                Val::Num(x) => x,
            };
            regs[a] = match op {
                Op::Inp => { inp.next().unwrap_or(0) }
                Op::Add => { regs[a] + b }
                Op::Mul => { regs[a] * b }
                Op::Div => { regs[a] / b }
                Op::Mod => { regs[a] % b }
                Op::Eql => { (regs[a] == b) as _ }
            }
        }
        assert_eq!(regs[Reg::Z as usize], 0);
    }

    Ok(())
}

#[derive(Copy, Clone)]
struct Inst { op: Op, a: Reg, b: Val }

#[derive(Copy, Clone)]
enum Op { Inp, Add, Mul, Div, Mod, Eql }

#[derive(Copy, Clone)]
enum Reg { W, X, Y, Z }

#[derive(Copy, Clone)]
enum Val { Reg(Reg), Num(i32) }

impl FromStr for Inst {
    type Err = ParseInstError;

    fn from_str(s: &str) -> Result<Inst, ParseInstError> {
        let (op, s) = s.split_once(' ').ok_or(ParseInstError)?;
        let op = Op::from_str(op)?;
        let (a, b) = match op {
            Op::Inp => (s, "0"),
            _ => s.split_once(' ').ok_or(ParseInstError)?,
        };
        let (a, b) = (Reg::from_str(a)?, Val::from_str(b)?);
        Ok(Inst { op, a, b })
    }
}

impl FromStr for Op {
    type Err = ParseInstError;

    fn from_str(s: &str) -> Result<Op, ParseInstError> {
        let op = match s {
            "inp" => Op::Inp,
            "add" => Op::Add,
            "mul" => Op::Mul,
            "div" => Op::Div,
            "mod" => Op::Mod,
            "eql" => Op::Eql,
            _ => Err(ParseInstError)?
        };
        Ok(op)
    }
}

impl FromStr for Reg {
    type Err = ParseInstError;

    fn from_str(s: &str) -> Result<Reg, ParseInstError> {
        let reg = match s {
            "w" => Reg::W, "x" => Reg::X, "y" => Reg::Y, "z" => Reg::Z,
            _ => Err(ParseInstError)?
        };
        Ok(reg)
    }
}

impl FromStr for Val {
    type Err = ParseInstError;

    fn from_str(s: &str) -> Result<Val, ParseInstError> {
        let val = match Reg::from_str(s) {
            Ok(reg) => Val::Reg(reg),
            Err(_) => Val::Num(i32::from_str(s)?),
        };
        Ok(val)
    }
}

#[derive(Debug)]
struct ParseInstError;

impl From<ParseIntError> for ParseInstError {
    fn from(_: ParseIntError) -> Self { ParseInstError }
}

impl Error for ParseInstError {}

impl fmt::Display for ParseInstError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseInstError")
    }
}

