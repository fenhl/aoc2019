use {
    std::{
        collections::HashMap,
        error::Error,
        fmt
    },
    itertools::Itertools as _
};

#[derive(Debug)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse orbit map")
    }
}

impl Error for ParseError {}

#[aoc_generator(day6)]
pub fn gen(input: &str) -> Result<HashMap<String, String>, ParseError> {
    let mut map = HashMap::default();
    for line in input.lines() {
        let (center, satellite) = line.split(')').map(ToString::to_string).collect_tuple().ok_or(ParseError)?;
        map.insert(satellite, center);
    }
    Ok(map)
}

fn orbit_chain<'a>(map: &'a HashMap<String, String>, mut obj: &'a str) -> Vec<&'a str> {
    let mut chain = Vec::default();
    while let Some(next_obj) = map.get(obj) {
        chain.push(&next_obj[..]);
        obj = &next_obj[..];
    }
    chain
}

#[aoc(day6, part1)]
pub fn part1(input: &HashMap<String, String>) -> usize {
    input.iter().map(|(sat, _)| orbit_chain(input, sat).len()).sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &HashMap<String, String>) -> usize {
    let you_chain = orbit_chain(input, "YOU");
    let san_chain = orbit_chain(input, "SAN");
    you_chain.iter().filter(|obj| !san_chain.contains(obj)).count() +
    san_chain.iter().filter(|obj| !you_chain.contains(obj)).count()
}
