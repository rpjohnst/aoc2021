use std::{error::Error, fs};
use aoc2021::day16::{decode_chars, Bits, Packet, Data, ParseBitsError};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day16")?;
    let chars = input.trim_end();
    let bytes = <Result<Vec<_>, ParseBitsError>>::from_iter(decode_chars(chars.as_bytes()))?;

    let packet = Packet::from_bits(&mut Bits::from_bytes(&bytes[..]))?;
    let value = visit(&packet);
    fn visit(packet: &Packet) -> usize {
        match packet.data {
            Data::Literal(value) => { value }
            Data::Operator(ref packets) => match packet.type_id {
                0 => { packets.iter().map(visit).sum() }
                1 => { packets.iter().map(visit).product() }
                2 => { packets.iter().map(visit).min().unwrap() }
                3 => { packets.iter().map(visit).max().unwrap() }
                5 => { (visit(&packets[0]) > visit(&packets[1])) as usize }
                6 => { (visit(&packets[0]) < visit(&packets[1])) as usize }
                7 => { (visit(&packets[0]) == visit(&packets[1])) as usize }
                _ => { unreachable!() }
            }
        }
    }
    println!("{}", value);

    Ok(())
}
