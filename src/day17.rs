use std::{
    fmt::{self, Display, Formatter},
    iter,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashSet; // for Iterator::next_tuple, Iterator::tuples

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
    eval_program(program, initial_state)
        .output_buffer
        .iter()
        .join(",")
}

#[aoc(day17, part2)]
fn part2((initial_state, program): &(StrangeDevice, Vec<u8>)) -> u64 {
    const BE_LOUD: bool = false;

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

    // find a good starting value to seed the search, by just trying all possible values
    let min_offset = (0..8)
        .find(|&ra| (get_output(program, ra, initial_state) % 8) as u8 == *program.last().unwrap())
        .unwrap();
    if BE_LOUD {
        println!(
            "seed value for final code: {} -> {}\n",
            min_offset,
            get_output(program, min_offset, initial_state)
        );
    }
    let mut quines = FxHashSet::default();
    quines.insert(min_offset);
    // Iter through increasing reverse slices of the program to gradually build up a set of good
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
    for num in iter::repeat(program)
        .take(program.len())
        .enumerate()
        .map(|(i, p)| num_cat(&p[i..]))
        .rev()
    {
        if BE_LOUD {
            println!("\nLooking for {}", num);
        }
        // For each possible quine, we must try all possible next values, since we don't know yet
        // which one will end up being the smallest.
        // We produce a new set of extended quines which is filtered-at-insertion to just the
        // quines that are not ruled out at the current power-of-8 step.
        // (NB: this is still exponential if the the partial-check filter does not constrain the search space,
        // but in practice it is much much faster than the naive search over all possible register values)
        let mut new_quines = FxHashSet::default();
        for curr in quines {
            for i in 0..8 {
                let new = (curr << 3) + i;
                let result = get_output(program, new, initial_state);
                if result == num {
                    if BE_LOUD {
                        println!("{} -> {}", new, result);
                    }
                    new_quines.insert(new);
                }
            }
        }
        if new_quines.is_empty() {
            panic!("No quines found for {}", num);
        }
        quines = new_quines;
    }
    // The minimum value of the quines at the current (minimal) power of 8
    // is the minimal quine-completing value, i.e. the solution
    let result = *quines.iter().min().unwrap();
    if BE_LOUD {
        println!("\nFound solution: {}\n\n", result);
    }
    result
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
    let operand = match operand {
        Operand::Literal(value) => value as u64,
        Operand::RegisterA => state.register_a,
        Operand::RegisterB => state.register_b,
        Operand::RegisterC => state.register_c,
        Operand::Ignored => panic!("Invalid numerator: {:?}", operand),
    };

    let result = state.register_a >> operand;

    match result_target {
        Operand::RegisterA => {
            state.register_a = result;
            state.instruction_pointer += 2;
        }
        Operand::RegisterB => {
            state.register_b = result;
            state.instruction_pointer += 2;
        }
        Operand::RegisterC => {
            state.register_c = result;
            state.instruction_pointer += 2;
        }
        Operand::Ignored | Operand::Literal(_) => {
            panic!("Invalid result_target: {:?}", result_target)
        }
    }
}

fn bitwise_xor(operand: Operand, state: &mut StrangeDevice) {
    let operand = match operand {
        Operand::Literal(value) => value as u64,
        Operand::RegisterA => state.register_a,
        Operand::RegisterB => state.register_b,
        Operand::RegisterC => state.register_c,
        Operand::Ignored => panic!("Invalid operand: {:?}", operand),
    };
    let result = state.register_b ^ operand;
    state.register_b = result;
    state.instruction_pointer += 2;
}

fn modulo_8(operand: Operand, state: &mut StrangeDevice) {
    let operand = match operand {
        Operand::Literal(value) => value as u64,
        Operand::RegisterA => state.register_a,
        Operand::RegisterB => state.register_b,
        Operand::RegisterC => state.register_c,
        Operand::Ignored => panic!("Invalid operand: {:?}", operand),
    };
    let result = operand % 8;

    state.register_b = result;
    state.instruction_pointer += 2;
}

fn jump_if_nonzero(operand: Operand, state: &mut StrangeDevice) {
    let operand = match operand {
        Operand::Literal(value) => value as usize,
        Operand::RegisterA => state.register_a as usize,
        Operand::RegisterB => state.register_b as usize,
        Operand::RegisterC => state.register_c as usize,
        Operand::Ignored => panic!("Invalid operand: {:?}", operand),
    };

    if state.register_a != 0 {
        state.instruction_pointer = operand;
    } else {
        state.instruction_pointer += 2;
    }
}

fn output_to_buffer(operand: Operand, state: &mut StrangeDevice) {
    let operand = match operand {
        Operand::Literal(value) => value,
        Operand::RegisterA => state.register_a as u8,
        Operand::RegisterB => state.register_b as u8,
        Operand::RegisterC => state.register_c as u8,
        Operand::Ignored => panic!("Invalid operand: {:?}", operand),
    };

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
