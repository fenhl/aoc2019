use {
    std::{
        mem,
        num::ParseIntError
    },
    crate::intcode::Program
};

#[aoc_generator(day5)]
pub fn gen(input: &str) -> Result<Program, ParseIntError> {
    input.parse()
}

#[aoc(day5, part1)]
pub fn part1(input: &Program) -> isize {
    let mut program = input.clone();
    program.input = Some(1);
    let mut last_output = None;
    while let Some(output) = program.run_until_output() {
        if let Some(last) = mem::replace(&mut last_output, None) {
            if last != 0 {
                panic!("self-test failed");
            }
        }
        last_output = Some(output);
    }
    last_output.expect("no output")
}

#[aoc(day5, part2)]
pub fn part2(input: &Program) -> isize {
    let mut program = input.clone();
    program.run_with_input(5).expect("no output")
}
