use std::{error::Error, fmt, iter};

pub struct Image { pub pixels: Vec<bool>, pub len: isize, pub outside: bool }

impl Image {
    pub fn pixel(&self, x: isize, y: isize) -> bool {
        if 0 <= x && x < self.len && 0 <= y && y < self.len {
            return self.pixels[(self.len * y + x) as usize];
        } else {
            self.outside
        }
    }

    pub fn enhance(&self, algorithm: &[bool]) -> Image {
        let len = self.len + 2;
        let mut pixels = Vec::from_iter(iter::repeat(false).take((len * len) as usize));
        for y in -1..self.len + 1 {
            for x in -1..self.len + 1 {
                let mut square = 0;
                for dy in -1..2 {
                    for dx in -1..2 {
                        square = square << 1 | self.pixel(x + dx, y + dy) as usize;
                    }
                }

                let (x, y) = (x + 1, y + 1);
                pixels[(len * y + x) as usize] = algorithm[square];
            }
        }

        let mut square = 0;
        for _ in -1..2 { for _ in -1..2 { square = square << 1 | self.outside as usize; } }
        let outside = algorithm[square];

        Image { pixels, len, outside }
    }
}

pub fn pixel_from_byte(b: u8) -> Result<bool, ParseImageError> {
    match b {
        b'.' => Ok(false),
        b'#' => Ok(true),
        _ => Err(ParseImageError),
    }
}

#[derive(Debug)]
pub struct ParseImageError;

impl Error for ParseImageError {}

impl fmt::Display for ParseImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseImageError")
    }
}
