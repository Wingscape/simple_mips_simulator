use std::collections::HashMap;
use std::fs;

fn parse_reg(ins: &str) -> usize {
    ins.trim_start_matches('#')
        .trim_end_matches(',')
        .parse()
        .unwrap_or(0)
}

fn parse_imm(ins: &str) -> u32 {
    ins.trim_end_matches(',').parse().unwrap_or(0)
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

    // TODO: add one of the machine cycles of: Fetch the next Instruction
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
        println!("value: {}", dict);
    }

    execute_lines(lines, &jmp_labels);
}

fn execute_lines(lines: Vec<&str>, jmp_labels: &HashMap<String, usize>) {
    let mut pc = 0;
    let mut registers: [u32; 32] = [0; 32];

    while pc < lines.len() {
        let mut jmp_pc = None;
        println!("pc: {}", pc);

        let parts: Vec<&str> = lines[pc].split_whitespace().collect();
        if parts.is_empty() {
            break;
        }

        // it's the opcode
        let opc = parts[0];

        match opc {
            // Syntax: [Instruction] [Destination], [Source], [Source], so on so forth
            "ADD" => {
                let opr_1 = parse_reg(parts[1]);
                let opr_2 = parse_imm(parts[2]);
                registers[opr_1] += opr_2;
            }
            // Syntax: [Instruction] [Source], [Destination]
            "LOAD" => {
                let opr_1 = parse_imm(parts[1]);
                let opr_2 = parse_reg(parts[2]);
                registers[opr_2] = opr_1;
            }
            // Syntax: [Instruction] [Source], [Source], [Destination]
            "BEQ" => {
                let opr_1 = parse_reg(parts[1]);
                let opr_2 = parse_reg(parts[2]);

                if registers[opr_1] == registers[opr_2] {
                    match jmp_labels.get(parts[3]) {
                        Some(value) => jmp_pc = Some(*value),
                        _ => {
                            eprintln!("Label not found: {}", parts[3]);
                            break;
                        }
                    }
                }
            }
            // Syntax: [Instruction] [Source], [Source], [Destination]
            "BNEQ" => {
                let opr_1 = parse_reg(parts[1]);
                let opr_2 = parse_reg(parts[2]);

                if registers[opr_1] != registers[opr_2] {
                    match jmp_labels.get(parts[3]) {
                        Some(value) => jmp_pc = Some(*value),
                        _ => {
                            eprintln!("Label not found: {}", parts[3]);
                            break;
                        }
                    }
                }
            }
            _ => {}
        }

        println!("R1: {}, R2: {}", registers[1], registers[2]);

        if let Some(jmp_pc) = jmp_pc {
            pc = jmp_pc;
        } else {
            pc += 1;
        }
    }
}

fn main() {
    run_file();
}
