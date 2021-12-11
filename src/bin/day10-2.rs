use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day10")?;
    let lines = Vec::from_iter(input.split_terminator('\n'));

    let mut stack = Vec::default();
    let mut scores = Vec::default();
    'lines: for line in lines {
        let mut score = 0usize;
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

                b')' => { stack.clear(); continue 'lines; }
                b']' => { stack.clear(); continue 'lines; }
                b'}' => { stack.clear(); continue 'lines; }
                b'>' => { stack.clear(); continue 'lines; }

                _ => { unreachable!(); }
            }
        }
        for delim in stack.drain(..).rev() {
            score = 5 * score + match delim {
                b')' => 1,
                b']' => 2,
                b'}' => 3,
                b'>' => 4,

                _ => { unreachable!(); }
            };
        }
        scores.push(score);
    }
    scores.sort_unstable();
    println!("{}", scores[scores.len() / 2]);

    Ok(())
}
