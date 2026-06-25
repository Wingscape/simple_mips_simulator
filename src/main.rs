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

// TODO: if register 0 then the value always 0
// NOTE: this code also simulates how the machine cycle works under the hood
fn execute_lines(lines: Vec<&str>, jmp_labels: &HashMap<String, usize>) {
    let mut pc = 0;
    let mut registers: [u32; 32] = [0; 32];

    // TODO: temp solution
    registers[0] = 0;

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
                registers[opr_1] += opr_2;
            }
            // Syntax: [Instruction] [Source], [Destination]
            "LOAD" => {
                let opr_1 = parse_imm(fields[0]);
                let opr_2 = parse_reg(fields[1]);
                registers[opr_2] = opr_1;
            }
            // Syntax: [Instruction] [Source], [Source], [Destination]
            "BEQ" => {
                let opr_1 = parse_reg(fields[0]);
                let opr_2 = parse_reg(fields[1]);

                if registers[opr_1] == registers[opr_2] {
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

                if registers[opr_1] != registers[opr_2] {
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

                registers[dest] = registers[reg] | imm;
            }
            // Syntax: [Instruction] [Destination], [Source], [Imm]
            "andi" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let imm = parse_imm(fields[2]);

                registers[dest] = registers[reg] & imm;
            }
            // Syntax: [Instruction] [Destination], [Source], [Imm]
            "xori" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let imm = parse_imm(fields[2]);

                registers[dest] = registers[reg] ^ imm;
            }
            _ => {
                eprintln!("Opcode not found: {}", opc);
                break;
            }
        }

        // TODO: temp solution
        registers[0] = 0;

        println!("R1: {}, R2: {}", registers[1], registers[2]);
    }
}

fn main() {
    run_file();
}
