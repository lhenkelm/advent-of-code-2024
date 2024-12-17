use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools; // for Iterator::next_tuple, Iterator::tuples

#[aoc_generator(day17)]
fn parse(input: &str) -> (StrangeDevice, Vec<u8>) {
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
        .collect();

    (initial_state, program)
}

#[aoc(day17, part1)]
fn part1((initial_state, program): &(StrangeDevice, Vec<u8>)) -> String {
    let mut history = vec![initial_state.clone()];
    dbg!(program);
    let final_state = loop {
        let state = history.last().unwrap();
        let instruction = Instruction::from_opcode(program[state.instruction_pointer]);
        let operand = Operand::from_u8(
            program[state.instruction_pointer + 1],
            instruction.arg_type(),
        );
        // we are set up to deal with instructions being interpreted as operands and vice versa,
        // but I don't think that will happen for our inputs, so I assert to check if it does
        debug_assert!(state.instruction_pointer % 2 == 0);
        let state = instruction.apply(operand, state);
        // + 1 because we take the operand from the pointer's increment
        if state.instruction_pointer + 1 > program.len() {
            break state;
        }
        history.push(state);
    };
    println!("Halted after {} steps", history.len());
    final_state.output_buffer.iter().join(",")
}

#[aoc(day17, part2)]
fn part2((initial_state, program): &(StrangeDevice, Vec<u8>)) -> u64 {
    todo!()
}

#[derive(Debug, Clone)]
struct StrangeDevice {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_pointer: usize,
    output_buffer: Vec<u8>,
}

impl StrangeDevice {
    fn with_registers(register_a: u64, register_b: u64, register_c: u64) -> Self {
        Self {
            register_a,
            register_b,
            register_c,
            instruction_pointer: 0,
            output_buffer: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn apply(&self, operand: Operand, state: &StrangeDevice) -> StrangeDevice {
        match self {
            Self::Adv => shift_right(operand, Operand::RegisterA, state),
            Self::Bxl => bitwise_xor(operand, state),
            Self::Bst => modulo_8(operand, state),
            Self::Jnz => jump_if_nonzero(operand, state),
            Self::Bxc => bitwise_xor(Operand::RegisterC, state),
            Self::Out => output_to_buffer(operand, state),
            Self::Bdv => shift_right(operand, Operand::RegisterB, state),
            Self::Cdv => shift_right(operand, Operand::RegisterC, state),
        }
    }
}

fn shift_right(operand: Operand, result_target: Operand, state: &StrangeDevice) -> StrangeDevice {
    let operand = match operand {
        Operand::Literal(value) => value as u64,
        Operand::RegisterA => state.register_a,
        Operand::RegisterB => state.register_b,
        Operand::RegisterC => state.register_c,
        Operand::Ignored => panic!("Invalid numerator: {:?}", operand),
    };

    let result = state.register_a >> operand;

    match result_target {
        Operand::RegisterA => StrangeDevice {
            register_a: result,
            register_b: state.register_b,
            register_c: state.register_c,
            instruction_pointer: state.instruction_pointer + 2,
            output_buffer: state.output_buffer.clone(),
        },
        Operand::RegisterB => StrangeDevice {
            register_a: state.register_a,
            register_b: result,
            register_c: state.register_c,
            instruction_pointer: state.instruction_pointer + 2,
            output_buffer: state.output_buffer.clone(),
        },
        Operand::RegisterC => StrangeDevice {
            register_a: state.register_a,
            register_b: state.register_b,
            register_c: result,
            instruction_pointer: state.instruction_pointer + 2,
            output_buffer: state.output_buffer.clone(),
        },
        Operand::Ignored | Operand::Literal(_) => {
            panic!("Invalid result_target: {:?}", result_target)
        }
    }
}

fn bitwise_xor(operand: Operand, state: &StrangeDevice) -> StrangeDevice {
    let operand = match operand {
        Operand::Literal(value) => value as u64,
        Operand::RegisterA => state.register_a,
        Operand::RegisterB => state.register_b,
        Operand::RegisterC => state.register_c,
        Operand::Ignored => panic!("Invalid operand: {:?}", operand),
    };
    let result = state.register_b ^ operand;

    StrangeDevice {
        register_a: state.register_a,
        register_b: result,
        register_c: state.register_c,
        instruction_pointer: state.instruction_pointer + 2,
        output_buffer: state.output_buffer.clone(),
    }
}

fn modulo_8(operand: Operand, state: &StrangeDevice) -> StrangeDevice {
    let operand = match operand {
        Operand::Literal(value) => value as u64,
        Operand::RegisterA => state.register_a,
        Operand::RegisterB => state.register_b,
        Operand::RegisterC => state.register_c,
        Operand::Ignored => panic!("Invalid operand: {:?}", operand),
    };
    let result = operand % 8;

    StrangeDevice {
        register_a: state.register_a,
        register_b: result,
        register_c: state.register_c,
        instruction_pointer: state.instruction_pointer + 2,
        output_buffer: state.output_buffer.clone(),
    }
}

fn jump_if_nonzero(operand: Operand, state: &StrangeDevice) -> StrangeDevice {
    let operand = match operand {
        Operand::Literal(value) => value as usize,
        Operand::RegisterA => state.register_a as usize,
        Operand::RegisterB => state.register_b as usize,
        Operand::RegisterC => state.register_c as usize,
        Operand::Ignored => panic!("Invalid operand: {:?}", operand),
    };

    if state.register_a != 0 {
        StrangeDevice {
            register_a: state.register_a,
            register_b: state.register_b,
            register_c: state.register_c,
            instruction_pointer: operand,
            output_buffer: state.output_buffer.clone(),
        }
    } else {
        StrangeDevice {
            register_a: state.register_a,
            register_b: state.register_b,
            register_c: state.register_c,
            instruction_pointer: state.instruction_pointer + 2,
            output_buffer: state.output_buffer.clone(),
        }
    }
}

fn output_to_buffer(operand: Operand, state: &StrangeDevice) -> StrangeDevice {
    let operand = match operand {
        Operand::Literal(value) => value,
        Operand::RegisterA => state.register_a as u8,
        Operand::RegisterB => state.register_b as u8,
        Operand::RegisterC => state.register_c as u8,
        Operand::Ignored => panic!("Invalid operand: {:?}", operand),
    };

    let mut output_buffer = state.output_buffer.clone();
    output_buffer.push(operand % 8);

    StrangeDevice {
        register_a: state.register_a,
        register_b: state.register_b,
        register_c: state.register_c,
        instruction_pointer: state.instruction_pointer + 2,
        output_buffer,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    const EXAMPLE_PT1: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    const EXAMPLE_PT2: &str = indoc! {"
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_PT1)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part1_example_parse() {
        let (initial_state, program) = parse(EXAMPLE_PT1);
        assert_eq!(initial_state.register_a, 729);
        assert_eq!(initial_state.register_b, 0);
        assert_eq!(initial_state.register_c, 0);
        assert_eq!(initial_state.output_buffer, vec![]);
        assert_eq!(program.len(), 6);
        assert_eq!(program, vec![0, 1, 5, 4, 3, 0]);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_PT2)), 117440);
    }
}
