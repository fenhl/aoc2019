use std::num::ParseIntError;

fn fuel(mass: u32) -> u32 { (mass / 3).saturating_sub(2) }

fn fuel_rec(mass: u32) -> u32 {
    let fuel = fuel(mass);
    if fuel == 0 { fuel } else { fuel + fuel_rec(fuel) }
}

#[aoc_generator(day1)]
pub fn gen(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u64 {
    input.iter().copied().map(fuel).map(u64::from).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u64 {
    input.iter().copied().map(fuel_rec).map(u64::from).sum()
}
