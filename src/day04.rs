use {
    std::{
        error::Error,
        fmt,
        num::ParseIntError,
        ops::RangeInclusive
    },
    derive_more::From,
    itertools::Itertools as _
};

#[derive(Debug, From)]
pub enum GenError {
    NumSeps,
    ParseInt(ParseIntError)
}

impl fmt::Display for GenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GenError::NumSeps => write!(f, "expected one hyphen, found none or more than one"),
            GenError::ParseInt(e) => e.fmt(f)
        }
    }
}

impl Error for GenError {}

fn valid_a(password: u32) -> bool {
    let digits = password.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
    digits.len() == 6
    && digits.iter().tuple_windows().any(|(d1, d2)| d1 == d2)
    && digits.iter().tuple_windows().all(|(d1, d2)| d1 <= d2)
}

fn valid_b(password: u32) -> bool {
    valid_a(password) && {
        let pwd_string = password.to_string();
        pwd_string.chars()
            .group_by(char::clone).into_iter()
            .any(|(_, group)| group.count() == 2)
    }
}

#[aoc_generator(day4)]
pub fn gen(input: &str) -> Result<RangeInclusive<u32>, GenError> {
    let (start, end) = input.splitn(2, '-').map(str::parse).collect_tuple().ok_or(GenError::NumSeps)?;
    Ok(start?..=end?)
}

#[aoc(day4, part1)]
pub fn part1(input: &RangeInclusive<u32>) -> usize {
    input.clone().filter(|&password| valid_a(password)).count()
}

#[aoc(day4, part2)]
pub fn part2(input: &RangeInclusive<u32>) -> usize {
    input.clone().filter(|&password| valid_b(password)).count()
}
