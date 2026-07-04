use std::collections::HashMap;
use std::fs;

fn parse_reg(field: &str) -> usize {
    field.trim_start_matches('$').parse().unwrap_or(0)
}

fn parse_imm(field: &str) -> u32 {
    if field.starts_with("0x") {
        // that means it's hex
        u32::from_str_radix(field.trim_start_matches("0x"), 16).unwrap_or(0)
    } else {
        field.parse().unwrap_or(0)
    }
}

struct Registers {
    registers: [u32; 31],
}

// abstraction
impl Registers {
    fn new() -> Self {
        Self { registers: [0; 31] }
    }

    // here we can modify the in
    fn set(&mut self, index: usize, value: u32) {
        // TODO: display error if user is trying to set value for 0 index at input
        self.registers[index - 1] = value;
    }

    // here we can modify the out
    fn get(&self, index: usize) -> u32 {
        if index == 0 {
            0
        } else {
            self.registers[index - 1]
        }
    }
}

// this code also simulates how the machine cycle works under the hood
fn execute_lines(lines: Vec<&str>, jmp_labels: &HashMap<String, usize>) {
    let mut pc = 0;

    // we create a wrapper around registers
    // why wrapper? well we had a vector to intialize...
    // but then we cannot do anything with accessing array or storing it...
    // cause it's a built in data type...
    // so we turn this into a custom data type so we can control in and out...
    // for registers by using the power of setter and getter abstraction
    let mut registers = Registers::new();

    while pc < lines.len() {
        println!("pc: {}", pc);

        // #1: fetch the next instruction
        let (opc, layout_field) = lines[pc]
            .split_once(char::is_whitespace)
            .unwrap_or(("", ""));

        let layout_field = layout_field.trim();
        let fields: Vec<&str> = layout_field.split(",").map(|field| field.trim()).collect();

        // #2: increment the pc
        pc += 1;

        // #3: execute the instruction
        match opc {
            // Syntax: [Instruction] [Destination], [Source], [Source], so on so forth
            "ADD" => {
                let opr_1 = parse_reg(fields[0]);
                let opr_2 = parse_imm(fields[1]);
                // registers[opr_1] += opr_2;
                // registers[opr_1] = registers[opr_1] + opr_2;
                registers.set(opr_1, registers.get(opr_1) + opr_2);
            }
            // Syntax: [Instruction] [Source], [Destination]
            "LOAD" => {
                let opr_1 = parse_imm(fields[0]);
                let opr_2 = parse_reg(fields[1]);
                registers.set(opr_2, registers.get(opr_2) + opr_1);
            }
            // Syntax: [Instruction] [Source], [Source], [Destination]
            "BEQ" => {
                let opr_1 = parse_reg(fields[0]);
                let opr_2 = parse_reg(fields[1]);

                if registers.get(opr_1) == registers.get(opr_2) {
                    match jmp_labels.get(fields[2]) {
                        Some(value) => pc = *value,
                        _ => {
                            eprintln!("Label not found: {}", fields[2]);
                            break;
                        }
                    }
                }
            }
            // Syntax: [Instruction] [Source], [Source], [Destination]
            "BNEQ" => {
                let opr_1 = parse_reg(fields[0]);
                let opr_2 = parse_reg(fields[1]);

                if registers.get(opr_1) != registers.get(opr_2) {
                    match jmp_labels.get(fields[2]) {
                        Some(value) => pc = *value,
                        _ => {
                            eprintln!("Label not found: {}", fields[2]);
                            break;
                        }
                    }
                }
            }
            // Syntax: [Instruction] [Destination], [Source], [Imm]
            "ori" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let imm = parse_imm(fields[2]);

                registers.set(dest, registers.get(reg) | imm);
            }
            // Syntax: [Instruction] [Destination], [Source], [Imm]
            "andi" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let imm = parse_imm(fields[2]);

                registers.set(dest, registers.get(reg) & imm);
            }
            // Syntax: [Instruction] [Destination], [Source], [Imm]
            "xori" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let imm = parse_imm(fields[2]);

                registers.set(dest, registers.get(reg) ^ imm);
            }
            // Syntax: [Instruction] [Destination], [Source], [Imm]
            "sll" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let imm = parse_imm(fields[2]);

                registers.set(dest, registers.get(reg) << imm);
            }
            // Syntax: [Instruction] [Destination], [Source], [Imm]
            "srl" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let imm = parse_imm(fields[2]);

                registers.set(dest, registers.get(reg) >> imm);
            }
            // Syntax: [Instruction] [Destination], [Source], [Source]
            "or" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let reg_2 = parse_reg(fields[2]);

                registers.set(dest, registers.get(reg) | registers.get(reg_2));
            }
            // Syntax: [Instruction] [Destination], [Source], [Source]
            "and" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let reg_2 = parse_reg(fields[2]);

                registers.set(dest, registers.get(reg) & registers.get(reg_2));
            }
            // Syntax: [Instruction] [Destination], [Source], [Source]
            "xor" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let reg_2 = parse_reg(fields[2]);

                registers.set(dest, registers.get(reg) ^ registers.get(reg_2));
            }
            // Syntax: [Instruction] [Destination], [Source], [Source]
            "nor" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let reg_2 = parse_reg(fields[2]);

                registers.set(dest, !(registers.get(reg) | registers.get(reg_2)));
            }
            // Syntax: [Instruction] [Destination], [Source], [Source]
            "addu" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let reg_2 = parse_reg(fields[2]);

                registers.set(dest, registers.get(reg) + registers.get(reg_2));
            }
            // Syntax: [Instruction] [Destination], [Source], [Source]
            "add" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let reg_2 = parse_reg(fields[2]);

                registers.set(dest, registers.get(reg) + registers.get(reg_2));
            }
            _ => {
                eprintln!("Opcode not found: {}", opc);
                break;
            }
        }

        println!(
            "R25: {}, R11: {}, R12: {}",
            registers.get(25),
            registers.get(11),
            registers.get(12)
        );
    }
}

fn run_file() {
    let content = fs::read_to_string("src/test.asm").expect("Failed to read file");
    let input_lines: Vec<&str> = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();

    let mut lines = vec![];
    let mut queued_labels = vec![];
    let mut jmp_labels = HashMap::new();

    for line in input_lines {
        if line.ends_with(":") {
            queued_labels.push(line);
        } else {
            while let Some(queued_label) = queued_labels.pop() {
                jmp_labels.insert(queued_label.trim_end_matches(":").to_string(), lines.len());
            }

            lines.push(line);
        }
    }

    // debug
    for dict in jmp_labels.values() {
        println!("jmp line: {}", dict);
    }

    execute_lines(lines, &jmp_labels);
}

fn main() {
    run_file();
}
