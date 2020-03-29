use {
    std::{
        convert::TryFrom,
        num::ParseIntError,
        str::FromStr
    },
    derive_more::{
        Index,
        IndexMut
    },
    num::Integer as _
};

#[derive(Clone, Copy)]
enum Instruction {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt
}

use Instruction::*;

impl Instruction {
    fn num_params(&self) -> usize {
        match self {
            Add => 3,
            Multiply => 3,
            Input => 1,
            Output => 1,
            JumpIfTrue => 2,
            JumpIfFalse => 2,
            LessThan => 3,
            Equals => 3,
            Halt => 0
        }
    }
}

impl TryFrom<isize> for Instruction {
    type Error = isize;

    fn try_from(opcode: isize) -> Result<Instruction, isize> {
        match opcode {
            1 => Ok(Add),
            2 => Ok(Multiply),
            3 => Ok(Input),
            4 => Ok(Output),
            5 => Ok(JumpIfTrue),
            6 => Ok(JumpIfFalse),
            7 => Ok(LessThan),
            8 => Ok(Equals),
            99 => Ok(Halt),
            _ => Err(opcode)
        }
    }
}

enum Arg {
    Pos {
        pos: usize,
        value: isize
    },
    Immediate(isize)
}

impl Arg {
    fn new(mode: isize, raw_arg: isize, program: &Program) -> Arg {
        match mode {
            0 => Arg::Pos {
                pos: raw_arg as usize,
                value: program[raw_arg as usize]
            },
            1 => Arg::Immediate(raw_arg),
            _ => { panic!("unknown argument mode: {}", mode); }
        }
    }

    fn read(&self) -> isize {
        match *self {
            Arg::Pos { value, .. } => value,
            Arg::Immediate(value) => value
        }
    }
}

enum InstrResult {
    MoveTo(usize),
    Output(isize, usize),
    Halt
}

#[derive(Clone, Index, IndexMut)]
pub struct Program {
    #[index]
    #[index_mut]
    memory: Vec<isize>,
    ip: usize,
    pub(crate) input: Option<isize>,
    output: Option<isize>
}

impl Program {
    fn new(memory: Vec<isize>) -> Program {
        Program {
            memory,
            ip: 0,
            input: None,
            output: None
        }
    }

    fn write(&mut self, arg: &Arg, value: isize) {
        match *arg {
            Arg::Pos { pos, .. } => { self[pos] = value; }
            Arg::Immediate(_) => { panic!("can't write to immediate-mode argument"); }
        }
    }

    fn apply_with_args(&mut self, instr: Instruction, args: Vec<Arg>) -> InstrResult {
        match instr {
            Add => { self.write(&args[2], args[0].read() + args[1].read()); }
            Multiply => { self.write(&args[2], args[0].read() * args[1].read()); }
            Input => { self.write(&args[0], self.input.expect("missing input")); }
            Output => { return InstrResult::Output(args[0].read(), self.ip + 1 + instr.num_params()); }
            JumpIfTrue => if args[0].read() != 0 { return InstrResult::MoveTo(args[1].read() as usize); },
            JumpIfFalse => if args[0].read() == 0 { return InstrResult::MoveTo(args[1].read() as usize); },
            LessThan => { self.write(&args[2], if args[0].read() < args[1].read() { 1 } else { 0 }); }
            Equals => { self.write(&args[2], if args[0].read() == args[1].read() { 1 } else { 0 }); }
            Halt => { return InstrResult::Halt; }
        }
        InstrResult::MoveTo(self.ip + 1 + instr.num_params())
    }

    fn step(&mut self) -> InstrResult {
        let (mut modes, instr) = self[self.ip].div_rem(&100);
        let instr = Instruction::try_from(instr).expect(&format!("unknown instruction at position {}", self.ip));
        let raw_args = self[self.ip + 1..self.ip + 1 + instr.num_params()].to_vec();
        let mut args = Vec::default();
        for i in 0..instr.num_params() {
            let (next_modes, mode) = modes.div_rem(&10);
            args.push(Arg::new(mode, raw_args[i], &self));
            modes = next_modes;
        }
        self.apply_with_args(instr, args)
    }

    pub(crate) fn run(&mut self) -> Option<isize> {
        loop {
            match self.step() {
                InstrResult::MoveTo(new_ip) => { self.ip = new_ip; }
                InstrResult::Output(output, new_ip) => {
                    self.output = Some(output);
                    self.ip = new_ip;
                }
                InstrResult::Halt => { break; }
            }
        }
        self.output
    }

    pub(crate) fn run_with_input(&mut self, input: isize) -> Option<isize> {
        self.input = Some(input);
        self.run()
    }

    pub(crate) fn run_until_output(&mut self) -> Option<isize> {
        loop {
            match self.step() {
                InstrResult::MoveTo(new_ip) => { self.ip = new_ip; }
                InstrResult::Output(output, new_ip) => {
                    self.output = Some(output);
                    self.ip = new_ip;
                    return Some(output);
                }
                InstrResult::Halt => { return None; }
            }
        }
    }
}

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Program, ParseIntError> {
        Ok(Program::new(s.split(',').map(str::parse).collect::<Result<_, _>>()?))
    }
}
