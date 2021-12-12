use std::{collections::HashMap, error::Error, fmt, fs, iter, mem};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day12")?;
    let entries: Vec<_> = Option::from_iter(input.split_terminator('\n').map(|edge| {
        edge.split_once('-')
    })).ok_or(ParseMapError)?;

    let mut caves = HashMap::new();
    let mut edges = Vec::with_capacity(entries.len());
    for &(v, w) in &entries[..] {
        let cave = caves.len();
        let v = *caves.entry(v).or_insert(cave);

        let cave = caves.len();
        let w = *caves.entry(w).or_insert(cave);

        edges.push((v, w));
        edges.push((w, v));
    }

    let mut large = Vec::from_iter(iter::repeat(false).take(caves.len()));
    for (&name, &v) in caves.iter() {
        large[v] = name.bytes().all(|b| b.is_ascii_uppercase());
    }

    let mut rs = Vec::from_iter(iter::repeat(0).take(1 + caves.len()));
    for &(v, _) in &edges[..] { rs[1 + v] += 1; }
    let mut sum = 0;
    for v in &mut rs[1..] { sum += mem::replace(v, sum); }
    let mut js = Vec::from_iter(iter::repeat(0).take(sum));
    for &(v, w) in &edges[..] { js[rs[1 + v]] = w; rs[1 + v] += 1; }

    let mut visited = Vec::from_iter(iter::repeat(false).take(caves.len()));
    let paths = visit(&rs[..], &js[..], &large[..], &mut visited[..], caves["start"], caves["end"]);
    println!("{}", paths);

    Ok(())
}

fn visit(
    rs: &[usize], js: &[usize], large: &[bool],
    visited: &mut [bool],
    v: usize, w: usize
) -> usize {
    if v == w { return 1; }

    let mut paths = 0;
    if !large[v] { visited[v] = true; }
    for &u in &js[rs[v + 0]..rs[v + 1]] {
        if !visited[u] { paths += visit(rs, js, large, visited, u, w); }
    }
    if !large[v] { visited[v] = false; }

    paths
}

#[derive(Debug)]
pub struct ParseMapError;

impl Error for ParseMapError {}

impl fmt::Display for ParseMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseMapError")
    }
}
