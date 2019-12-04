use {
    std::{
        collections::HashMap,
        error::Error,
        fmt,
        iter::FromIterator,
        num::ParseIntError,
        str::FromStr
    },
    derive_more::From,
    itertools::Itertools as _
};

type Point = (i32, i32);
type Wire = HashMap<Point, u32>;

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down
}

#[derive(Clone, Copy)]
pub struct WirePart {
    direction: Direction,
    length: u32
}

impl WirePart {
    fn add(&self, wire: &mut Wire, pos: &mut Point, len: &mut u32) {
        for _ in 0..self.length {
            match self.direction {
                Direction::Right => { pos.0 += 1; }
                Direction::Up => { pos.1 -= 1; }
                Direction::Left => { pos.0 -= 1; }
                Direction::Down => { pos.1 += 1; }
            }
            *len += 1;
            wire.entry(*pos).or_insert(*len);
        }
    }
}

#[derive(Debug, From)]
pub enum WirePartFromStrErr {
    LengthFormat(ParseIntError),
    MissingDirection,
    UnknownDirection(char)
}

impl fmt::Display for WirePartFromStrErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WirePartFromStrErr::LengthFormat(e) => e.fmt(f),
            WirePartFromStrErr::MissingDirection => write!(f, "empty wire part string, expected direction"),
            WirePartFromStrErr::UnknownDirection(dir) => write!(f, "expected direction R, U, L, or D, got {}", dir)
        }
    }
}

impl Error for WirePartFromStrErr {}

impl FromStr for WirePart {
    type Err = WirePartFromStrErr;

    fn from_str(s: &str) -> Result<WirePart, WirePartFromStrErr> {
        Ok(WirePart {
            direction: match s.chars().next().ok_or(WirePartFromStrErr::MissingDirection)? {
                'R' => Direction::Right,
                'U' => Direction::Up,
                'L' => Direction::Left,
                'D' => Direction::Down,
                dir => { return Err(WirePartFromStrErr::UnknownDirection(dir)); }
            },
            length: s[1..].parse()?
        })
    }
}

impl FromIterator<WirePart> for Wire {
    fn from_iter<T: IntoIterator<Item = WirePart>>(iter: T) -> Self {
        let mut result = HashMap::default();
        let mut pos = (0, 0);
        let mut len = 0;
        for part in iter {
            part.add(&mut result, &mut pos, &mut len);
        }
        result
    }
}

fn manhattan_distance(p1: Point, p2: Point) -> u32 {
    (p1.0 - p2.0).abs() as u32 + (p1.1 - p2.1).abs() as u32
}

fn parse_wire(s: &str) -> Result<Wire, WirePartFromStrErr> {
    s.split(',').map(str::parse::<WirePart>).collect()
}

fn min_distance(w1: &Wire, w2: &Wire) -> Option<u32> {
    w1.keys()
        .filter(|point| w2.contains_key(point))
        .copied()
        .map(|point| manhattan_distance((0, 0), point))
        .min()
}

fn min_steps(w1: &Wire, w2: &Wire) -> Option<u32> {
    w1.iter()
        .filter_map(|(point, steps1)| w2.get(point).map(|steps2| steps1 + steps2))
        .min()
}

#[aoc_generator(day3)]
pub fn gen(input: &str) -> Result<Vec<Wire>, WirePartFromStrErr> {
    input.lines().map(parse_wire).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Wire]) -> u32 {
    let (w1, w2) = input.iter().collect_tuple().expect("expected exactly 2 wires");
    min_distance(w1, w2).expect("no intersections found")
}

#[aoc(day3, part2)]
pub fn part2(input: &[Wire]) -> u32 {
    let (w1, w2) = input.iter().collect_tuple().expect("expected exactly 2 wires");
    min_steps(w1, w2).expect("no intersections found")
}

#[test]
fn ex0() -> Result<(), WirePartFromStrErr> {
    let w1 = parse_wire("R8,U5,L5,D3")?;
    let w2 = parse_wire("U7,R6,D4,L4")?;
    assert_eq!(min_distance(w1, w2), Some(6));
    Ok(())
}

#[test]
fn ex1() -> Result<(), WirePartFromStrErr> {
    let w1 = parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72")?;
    let w2 = parse_wire("U62,R66,U55,R34,D71,R55,D58,R83")?;
    assert_eq!(min_distance(w1, w2), Some(159));
    Ok(())
}

#[test]
fn ex2() -> Result<(), WirePartFromStrErr> {
    let w1 = parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")?;
    let w2 = parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")?;
    assert_eq!(min_distance(w1, w2), Some(135));
    Ok(())
}
