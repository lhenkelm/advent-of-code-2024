use std::fmt::{self, Debug, Display, Formatter};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools; // for Iterator::next_tuple

#[aoc_generator(day17)]
fn parse(input: &str) -> (StrangeDevice, Vec<u8>) {
    let input = input.replace("\r\n", "\n");
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
    eval_program(program, initial_state)
        .output_buffer
        .iter()
        .join(",")
}

const BE_LOUD: bool = false;

#[aoc(day17, part2)]
fn part2((initial_state, program): &(StrangeDevice, Vec<u8>)) -> u64 {
    if BE_LOUD {
        println!(
            "Finding initial state of register A that makes a quine of: {}\n",
            num_cat(program)
        );
        println!("The program decompiles to the following instructions,");
        println!("(in the format <long description>; (<mnemonic>, <opcode>)  [with operand: <operand>]):\n");
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        for i in 0..program.len() {
            if i % 2 == 1 {
                continue;
            }
            let instruction = Instruction::from_opcode(program[i]);
            let operand = Operand::from_u8(program[i + 1], instruction.arg_type());
            let inst_str =
                format!("{}", instruction).replace("<operand>", &format!("{:?}", operand));
            println!("{inst_str}  [with operand: {operand:?}]");
        }
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
    }

    // Recurse greedily through increasing reverse slices of the program to gradually build up a set of good
    // candidate initial values. This works because the _only_ way the described instruction
    // set can produce a number greater than 7 in the output buffer is by shifting register A right
    // by a number greater than 3, outputting the result % 8, and then jumping back ahead of some similar
    // shift instruction to continue the loop until A is consumed.
    //
    // As a result we can approach the solution in steps of powers of 8,
    // and at each power, there are only 8 different possible inputs.
    // We can also confirm the lowest-power part of a partial solution at each step,
    // by comparing the output of the program with with the concatenation of the end of the program.
    // Because both opcodes and operands are 3 bits, this end-first check can match the last-power-of-8
    // splitting of the solution by simply reverse-iterating through the program.
    //
    // Because each part of the program output is "obscured" by the modulo 8 operation, but the internal
    // state of the registers is not constrained, we need to build up a set of all possible quine(-making value)s
    // at each step to continue the search at the next step from.
    //
    // In principle it could be necessary to check much longer registers than the program length,
    // (since a general program could *skip* part of the register depending on the value of some register)
    // but in practice neither the test not my input required this. For the reason outlined above,
    // this is the minimum number of "powers of 8" we need to check to find this "shortest possible" solution.
    // The minimum value of the quines at the current (minimal) power of 8
    // is the minimal quine-completing value, i.e. the solution
    let quines = quine_recursion(program.len() - 1, 0, program, initial_state);
    let result = *quines.iter().min().unwrap();
    if BE_LOUD {
        println!("\nFound solution: {}\n\n", result);
    }
    result
}

fn quine_recursion(
    up_to: usize,
    register_a: u64,
    program: &[u8],
    initial_state: &StrangeDevice,
) -> Vec<u64> {
    let mut solutions = vec![];
    for i in 0..8 {
        let new = (register_a << 3) + i;
        let result = get_output(program, new, initial_state);
        if result != num_cat(&program[up_to..]) {
            continue;
        }
        if BE_LOUD {
            println!("{} -> {}", new, result);
        }
        if up_to == 0 {
            solutions.push(new);
            break;
        }
        let mut deep = quine_recursion(up_to - 1, new, program, initial_state);
        solutions.append(&mut deep);
    }
    solutions
}

fn get_output(program: &[u8], register_a: u64, initial_state: &StrangeDevice) -> u64 {
    let state = StrangeDevice {
        register_a,
        register_b: initial_state.register_b,
        register_c: initial_state.register_c,
        instruction_pointer: initial_state.instruction_pointer,
        output_buffer: initial_state.output_buffer.clone(),
    };
    num_cat(&eval_program(program, &state).output_buffer)
}

fn eval_program(program: &[u8], state: &StrangeDevice) -> StrangeDevice {
    let mut state = state.clone();
    loop {
        let instruction = Instruction::from_opcode(program[state.instruction_pointer]);
        let operand = Operand::from_u8(
            program[state.instruction_pointer + 1],
            instruction.arg_type(),
        );
        // we are set up to deal with instructions being interpreted as operands and vice versa,
        // but I don't think that will happen for our inputs, so I assert to check if it does
        debug_assert!(state.instruction_pointer % 2 == 0);
        instruction.apply(operand, &mut state);
        // + 1 because we take the operand from the pointer's increment
        if state.instruction_pointer + 1 > program.len() {
            break state;
        }
    }
}

fn num_cat(parts: &[u8]) -> u64 {
    parts.iter().fold(0, |acc, &part| acc * 10 + part as u64)
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

    fn apply(&self, operand: Operand, state: &mut StrangeDevice) {
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

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let long_name = match self {
            Self::Adv => "Bit-shift register A right <operand> times -> register A; (adv, 0)",
            Self::Bxl => "Bitwise XOR register B with <operand> -> register B; (bxl, 1)",
            Self::Bst => "Modulo 8 of <operand> -> register B; (bst, 2)",
            Self::Jnz => "Jump to <operand> if register A is non-zero; (jnz, 3)",
            Self::Bxc => "Bitwise XOR register B with register C -> register B; (bxc, 4)",
            Self::Out => "Output <operand> modulo 8 to buffer; (out, 5)",
            Self::Bdv => "Bit-shift register A  right <operand> times -> register B; (bdv, 6)",
            Self::Cdv => "Bit-shift register A right <operand> times -> register C; (cdv, 7)",
        };
        write!(f, "{}", long_name)
    }
}

fn shift_right(operand: Operand, result_target: Operand, state: &mut StrangeDevice) {
    let operand = operand.get_as_u64(state);
    let result = state.register_a >> operand;

    match result_target {
        Operand::RegisterA => {
            state.register_a = result;
        }
        Operand::RegisterB => {
            state.register_b = result;
        }
        Operand::RegisterC => {
            state.register_c = result;
        }
        Operand::Ignored | Operand::Literal(_) => {
            panic!("Invalid result_target: {:?}", result_target)
        }
    }
    state.instruction_pointer += 2;
}

fn bitwise_xor(operand: Operand, state: &mut StrangeDevice) {
    let operand = operand.get_as_u64(state);
    let result = state.register_b ^ operand;
    state.register_b = result;
    state.instruction_pointer += 2;
}

fn modulo_8(operand: Operand, state: &mut StrangeDevice) {
    let operand = operand.get_as_u64(state);
    let result = operand % 8;

    state.register_b = result;
    state.instruction_pointer += 2;
}

fn jump_if_nonzero(operand: Operand, state: &mut StrangeDevice) {
    let operand = operand.get_as_usize(state);

    if state.register_a != 0 {
        state.instruction_pointer = operand;
    } else {
        state.instruction_pointer += 2;
    }
}

fn output_to_buffer(operand: Operand, state: &mut StrangeDevice) {
    let operand = operand.get_as_u8(state);

    state.output_buffer.push(operand % 8);
    state.instruction_pointer += 2;
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

    fn get_as_u64(&self, state: &StrangeDevice) -> u64 {
        match self {
            Self::Literal(value) => *value as u64,
            Self::RegisterA => state.register_a,
            Self::RegisterB => state.register_b,
            Self::RegisterC => state.register_c,
            Self::Ignored => panic!("Operand::Ignored has no value"),
        }
    }

    fn get_as_u8(&self, state: &StrangeDevice) -> u8 {
        match self {
            Self::Literal(value) => *value,
            Self::RegisterA => state.register_a as u8,
            Self::RegisterB => state.register_b as u8,
            Self::RegisterC => state.register_c as u8,
            Self::Ignored => panic!("Operand::Ignored has no value"),
        }
    }

    fn get_as_usize(&self, state: &StrangeDevice) -> usize {
        match self {
            Self::Literal(value) => *value as usize,
            Self::RegisterA => state.register_a as usize,
            Self::RegisterB => state.register_b as usize,
            Self::RegisterC => state.register_c as usize,
            Self::Ignored => panic!("Operand::Ignored has no value"),
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_PT2)), 117440);
    }
}
