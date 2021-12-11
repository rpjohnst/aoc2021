use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day10")?;
    let lines = Vec::from_iter(input.split_terminator('\n'));

    let mut stack = Vec::default();
    let mut score = 0;
    for line in lines {
        for delim in line.bytes() {
            match delim {
                b'(' => { stack.push(b')'); }
                b'[' => { stack.push(b']'); }
                b'{' => { stack.push(b'}'); }
                b'<' => { stack.push(b'>'); }

                b')' if stack.last() == Some(&b')') => { stack.pop(); }
                b']' if stack.last() == Some(&b']') => { stack.pop(); }
                b'}' if stack.last() == Some(&b'}') => { stack.pop(); }
                b'>' if stack.last() == Some(&b'>') => { stack.pop(); }

                b')' => { score += 3; break; }
                b']' => { score += 57; break; }
                b'}' => { score += 1197; break; }
                b'>' => { score += 25137; break; }

                _ => { unreachable!(); }
            }
        }
        stack.clear();
    }
    println!("{}", score);

    Ok(())
}
