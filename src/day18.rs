use std::{error::Error, fmt, ops::AddAssign, str::FromStr};

#[derive(Clone, Debug)]
pub struct Number { pub depths: Vec<u8>, pub number: Vec<u8> }

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Number) {
        self.depths.extend(rhs.depths);
        self.number.extend(rhs.number);
        for d in &mut self.depths[..] { *d += 1; }

        loop {
            if let Some(a) = self.depths.iter().position(|&d| d > 4) {
                let b = a + 1;
                if a > 0 { self.number[a - 1] += self.number[a]; }
                if b < self.number.len() - 1 { self.number[b + 1] += self.number[b]; }
                self.depths[a] -= 1;
                self.number[a] = 0;
                self.depths.remove(b);
                self.number.remove(b);
                continue;
            }

            if let Some(a) = self.number.iter().position(|&n| n > 9) {
                let x = self.number[a] as f32 / 2.0;
                let (x, y) = (f32::floor(x) as u8, f32::ceil(x) as u8);
                self.depths[a] += 1;
                self.number[a] = x;
                self.depths.insert(a + 1, self.depths[a]);
                self.number.insert(a + 1, y);
                continue;
            }

            break;
        }
    }
}

impl Number {
    pub fn magnitude(&self) -> usize {
        let (magnitude, &[..], &[..]) = visit(0, &self.depths[..], &self.number[..]);
        fn visit<'a>(
            depth: usize, depths: &'a [u8], number: &'a [u8]
        ) -> (usize, &'a [u8], &'a [u8]) {
            match (depths, number) {
                (&[d, ref depths @ ..], &[n, ref number @ ..]) if d as usize == depth => {
                    (n as usize, depths, number)
                }
                (&[d, ..], number) if d as usize > depth => {
                    let (a, depths, number) = visit(depth + 1, depths, number);
                    let (b, depths, number) = visit(depth + 1, depths, number);
                    (3 * a + 2 * b, depths, number)
                }
                (&[..], &[..]) => { unreachable!() }
            }
        }
        magnitude
    }
}

impl FromStr for Number {
    type Err = ParseNumberError;

    fn from_str(s: &str) -> Result<Number, ParseNumberError> {
        let mut depths = Vec::default();
        let mut number = Vec::default();

        let mut stack = Vec::default();
        for b in s.bytes() {
            match b {
                b'0'..=b'9' => {
                    match stack.last_mut() {
                        Some(b @ b'0') => { *b = b','; }
                        Some(b @ b'1') => { *b = b']'; }
                        None => {}
                        _ => { Err(ParseNumberError)? }
                    }

                    depths.push(stack.len() as u8);
                    number.push(b - b'0');
                }
                b'[' => {
                    match stack.last_mut() {
                        Some(b @ b'0') => { *b = b','; }
                        Some(b @ b'1') => { *b = b']'; }
                        None => {}
                        _ => { Err(ParseNumberError)? }
                    }

                    stack.push(b'0');
                }
                b',' => match stack.last_mut() {
                    Some(b @ b',') => { *b = b'1'; }
                    _ => { Err(ParseNumberError)? }
                }
                b']' => match stack.last_mut() {
                    Some(b']') => { stack.pop(); }
                    _ => { Err(ParseNumberError)? }
                }
                _ => { Err(ParseNumberError)? }
            }
        }

        Ok(Number { depths, number })
    }
}

#[derive(Debug)]
pub struct ParseNumberError;

impl Error for ParseNumberError {}

impl fmt::Display for ParseNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseNumberError")
    }
}
