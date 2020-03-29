use {
    std::num::ParseIntError,
    crate::intcode::Program
};

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Result<Program, ParseIntError> {
    input.parse()
}

fn replace_and_run(program: &mut Program, noun: isize, verb: isize) -> isize {
    program[1] = noun;
    program[2] = verb;
    program.run();
    program[0]
}

#[aoc(day2, part1)]
pub fn part1(input: &Program) -> isize {
    replace_and_run(&mut input.clone(), 12, 2)
}

#[aoc(day2, part2)]
pub fn part2(input: &Program) -> isize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if replace_and_run(&mut input.clone(), noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("no solution found");
}

#[test]
fn ex0() {
    let mut program = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    run(&mut program);
    assert_eq!(program, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
}

#[test]
fn ex1() {
    let mut program = vec![1,0,0,0,99];
    run(&mut program);
    assert_eq!(program, vec![2,0,0,0,99]);
}

#[test]
fn ex2() {
    let mut program = vec![2,3,0,3,99];
    run(&mut program);
    assert_eq!(program, vec![2,3,0,6,99]);
}

#[test]
fn ex3() {
    let mut program = vec![2,4,4,5,99,0];
    run(&mut program);
    assert_eq!(program, vec![2,4,4,5,99,9801]);
}

#[test]
fn ex4() {
    let mut program = vec![1,1,1,4,99,5,6,0,99];
    run(&mut program);
    assert_eq!(program, vec![30,1,1,4,2,5,6,0,99]);
}
