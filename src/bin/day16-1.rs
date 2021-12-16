use std::{error::Error, fs};
use aoc2021::day16::{decode_chars, Bits, Packet, Data, ParseBitsError};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day16")?;
    let chars = input.trim_end();
    let bytes = <Result<Vec<_>, ParseBitsError>>::from_iter(decode_chars(chars.as_bytes()))?;

    let packet = Packet::from_bits(&mut Bits::from_bytes(&bytes[..]))?;
    let sum = visit(&packet);
    fn visit(packet: &Packet) -> usize {
        packet.version + match packet.data {
            Data::Literal(_) => { 0 }
            Data::Operator(ref packets) => { packets.iter().map(visit).sum() }
        }
    }
    println!("{}", sum);

    Ok(())
}
