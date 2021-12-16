use std::{error::Error, fmt};

pub fn decode_chars(chars: &[u8]) -> impl Iterator<Item = Result<u8, ParseBitsError>> + '_ {
    chars.array_chunks().map(|&[a, b]| {
        let a = decode_char(a)?;
        let b = decode_char(b)?;
        Ok(u8::reverse_bits(a) >> 4 | u8::reverse_bits(b))
    })
}

fn decode_char(char: u8) -> Result<u8, ParseBitsError> {
    match char {
        b'0'..=b'9' => Ok(0x0 + char - b'0'),
        b'A'..=b'F' => Ok(0xA + char - b'A'),
        _ => Err(ParseBitsError),
    }
}

pub struct Bits<'a> { next: usize, rest: &'a [u8] }

impl Bits<'_> {
    pub fn from_bytes(rest: &[u8]) -> Bits<'_> { Bits { next: 0, rest } }

    pub fn len(&self) -> usize { 8 * self.rest.len() - self.next }

    pub fn number(&mut self, bits: u8) -> Result<usize, ParseBitsError> {
        let mut number = 0;
        for _ in 0..bits { number = number << 1 | self.bit()?; }
        Ok(number)
    }

    fn bit(&mut self) -> Result<usize, ParseBitsError> {
        if self.rest.len() == 0 { return Err(ParseBitsError); }

        let bit = (self.rest[0] >> self.next) & 0b1;

        self.next += 1;
        if self.next == 8 {
            self.rest = &self.rest[1..];
            self.next = 0;
        }

        Ok(bit as usize)
    }
}

pub struct Packet { pub version: usize, pub type_id: usize, pub data: Data }

pub enum Data { Literal(usize), Operator(Vec<Packet>) }

impl Packet {
    pub fn from_bits(bits: &mut Bits<'_>) -> Result<Packet, ParseBitsError> {
        let version = bits.number(3)?;
        let type_id = bits.number(3)?;
        let data = if type_id == 4 {
            let mut literal = 0;
            loop {
                let group = bits.number(5)?;
                literal = literal << 4 | (group & 0b0_1111);
                if group < 0b1_0000 { break; }
            }
            Data::Literal(literal)
        } else {
            let mut packets = Vec::default();
            let length_type_id = bits.number(1)?;
            if length_type_id == 0 {
                let mut len = bits.number(15)?;
                while len > 0 {
                    let start = bits.len();
                    packets.push(Packet::from_bits(bits)?);
                    len -= start - bits.len();
                }
            } else if length_type_id == 1 {
                let len = bits.number(11)?;
                for _ in 0..len { packets.push(Packet::from_bits(bits)?); }
            } else {
                return Err(ParseBitsError);
            }
            Data::Operator(packets)
        };
        Ok(Packet { version, type_id, data })
    }
}

#[derive(Debug)]
pub struct ParseBitsError;

impl Error for ParseBitsError {}

impl fmt::Display for ParseBitsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseBitsError")
    }
}