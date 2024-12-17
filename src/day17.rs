use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools; // for Iterator::next_tuple, Iterator::tuples

#[aoc_generator(day17)]
fn parse(input: &str) -> (StrangeDevice, Vec<(Instruction, Operand)>) {
    let (register_str, program_str) = input.trim().split("\n\n").next_tuple().unwrap();

    // Parse the registers, trusting that the input is well-formed
    let mut reg_it = register_str
        .lines()
        .map(|line| line.split(": ").last().unwrap().parse().unwrap());
    let initial_state = StrangeDevice::with_registers(
        reg_it.next().unwrap(),
        reg_it.next().unwrap(),
        reg_it.next().unwrap(),
    );

    let program = program_str
        .split(' ')
        .next_tuple::<(&str, &str)>()
        .unwrap()
        .1
        .split(',')
        .map(|tok| tok.parse().unwrap())
        .tuples()
        .map(|(opcode, operand)| {
            let inst = Instruction::from_opcode(opcode);
            let operand = Operand::from_u8(operand, inst.arg_type());
            (inst, operand)
        })
        .collect();

    (initial_state, program)
}

#[aoc(day17, part1)]
fn part1((initial_state, program): &(StrangeDevice, Vec<(Instruction, Operand)>)) -> String {
    todo!()
}

#[aoc(day17, part2)]
fn part2((initial_state, program): &(StrangeDevice, Vec<(Instruction, Operand)>)) -> String {
    todo!()
}

struct StrangeDevice {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instruction_pointer: usize,
}

impl StrangeDevice {
    fn with_registers(register_a: i64, register_b: i64, register_c: i64) -> Self {
        Self {
            register_a,
            register_b,
            register_c,
            instruction_pointer: 0,
        }
    }
}

enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn from_opcode(opcode: u8) -> Self {
        match opcode {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid opcode: {}", opcode),
        }
    }

    fn arg_type(&self) -> OperandType {
        match self {
            Self::Adv | Self::Bst | Self::Out | Self::Bdv | Self::Cdv => OperandType::Combo,
            Self::Bxl | Self::Jnz => OperandType::Literal,
            Self::Bxc => OperandType::Ignored,
        }
    }
}

enum Operand {
    Literal(u8),
    RegisterA,
    RegisterB,
    RegisterC,
    Ignored,
}

impl Operand {
    fn literal_from_u8(value: u8) -> Self {
        Self::Literal(value)
    }

    fn combo_from_u8(value: u8) -> Self {
        match value {
            0..4 => Self::literal_from_u8(value),
            4 => Self::RegisterA,
            5 => Self::RegisterB,
            6 => Self::RegisterC,
            _ => panic!("Invalid combo value: {}", value),
        }
    }

    fn from_u8(value: u8, op_type: OperandType) -> Self {
        match op_type {
            OperandType::Literal => Self::literal_from_u8(value),
            OperandType::Combo => Self::combo_from_u8(value),
            OperandType::Ignored => Self::Ignored,
        }
    }
}

enum OperandType {
    Literal,
    Combo,
    Ignored,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    #[ignore]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "<RESULT>");
    }
}
