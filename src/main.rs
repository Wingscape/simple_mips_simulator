use std::collections::HashMap;
use std::fs;

struct Registers {
    registers: [u32; 31],
    hilo: u64,
}

// abstraction
impl Registers {
    fn new() -> Self {
        Self {
            registers: [0; 31],
            hilo: 0,
        }
    }

    // here we can modify the in
    fn set(&mut self, index: usize, value: u32) {
        if index > 0 {
            self.registers[index - 1] = value;
        }
    }

    // here we can modify the out
    fn get(&self, index: usize) -> u32 {
        if index == 0 {
            0
        } else {
            self.registers[index - 1]
        }
    }

    fn set_hilo(&mut self, value: u64) {
        self.hilo = value;
    }

    fn get_hi(&self) -> u32 {
        (self.hilo >> 32) as u32
    }

    fn get_lo(&self) -> u32 {
        // implicit dereference
        // (*self).hilo as u32
        self.hilo as u32
    }
}

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

fn parse_imm_signed(field: &str) -> i32 {
    field.parse().unwrap_or(0)
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

                // use modular addition to wrap around after reaching a specific value
                // (a + b) (mod 2^N)
                // how so?
                // 1111 1111 + 0000 0001
                // result in 0000 0000 (0 in decimal) with overflow of 1
                // so it will go back to the beginning
                registers.set(dest, registers.get(reg).wrapping_add(registers.get(reg_2)));
            }
            // Syntax: [Instruction] [Destination], [Source], [Source]
            "add" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let reg_2 = parse_reg(fields[2]);

                // if overflow occurs, just panic the assembly program itself
                let (result, is_overflow) =
                    registers.get(reg).overflowing_add(registers.get(reg_2));

                if is_overflow {
                    eprintln!("Overflow just occured!");
                    break;
                }

                registers.set(dest, result);
            }
            // so addiu and addi both always treat the immediate as signed integer
            // Syntax: [Instruction] [Destination], [Source], [Imm]
            "addiu" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                // it copy over the binary from signed integer to unsigned
                let imm = parse_imm_signed(fields[2]) as u32;

                registers.set(dest, registers.get(reg).wrapping_add(imm));
            }
            // Syntax: [Instruction] [Destination], [Source], [Imm]
            "addi" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let imm = parse_imm_signed(fields[2]) as u32;

                // if overflow occurs, just panic the assembly program itself
                let (result, is_overflow) = registers.get(reg).overflowing_add(imm);

                if is_overflow {
                    eprintln!("Overflow just occured!");
                    break;
                }

                registers.set(dest, result);
            }
            // Syntax: [Instruction] [Destination], [Source], [Source]
            "subu" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let reg_2 = parse_reg(fields[2]);

                registers.set(dest, registers.get(reg).wrapping_sub(registers.get(reg_2)));
            }
            // Syntax: [Instruction] [Destination], [Source], [Source]
            "sub" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let reg_2 = parse_reg(fields[2]);

                // if overflow occurs, just panic the assembly program itself
                let (result, is_overflow) =
                    registers.get(reg).overflowing_sub(registers.get(reg_2));

                if is_overflow {
                    eprintln!("Overflow just occured!");
                    break;
                }

                registers.set(dest, result);
            }
            // Signed Multiplication
            // Syntax: [Instruction] [Source], [Source]
            "mult" => {
                let reg = parse_reg(fields[0]);
                let reg_2 = parse_reg(fields[1]);

                let mult_opr = (registers.get(reg) as i32) as i64;
                let mult_opr_2 = (registers.get(reg_2) as i32) as i64;

                registers.set_hilo(mult_opr.wrapping_mul(mult_opr_2) as u64);
            }
            // Unsigned Multiplication
            // Syntax: [Instruction] [Source], [Source]
            "multu" => {
                let reg = parse_reg(fields[0]);
                let reg_2 = parse_reg(fields[1]);

                let mult_opr = registers.get(reg) as u64;
                let mult_opr_2 = registers.get(reg_2) as u64;

                registers.set_hilo(mult_opr.wrapping_mul(mult_opr_2));
            }
            "mfhi" => {
                let reg = parse_reg(fields[0]);
                registers.set(reg, registers.get_hi());
            }
            "mflo" => {
                let reg = parse_reg(fields[0]);
                registers.set(reg, registers.get_lo());
            }
            // Signed Division
            // Syntax: [Instruction] [Source], [Source]
            "div" => {
                let reg = parse_reg(fields[0]);
                let reg_2 = parse_reg(fields[1]);

                let div_opr = (registers.get(reg) as i32) as i64;
                let div_opr_2 = (registers.get(reg_2) as i32) as i64;

                let quotient = div_opr.wrapping_div(div_opr_2);
                let remainder = div_opr.wrapping_rem(div_opr_2);

                registers.set_hilo((remainder << 32 | quotient) as u64);
            }
            // Unsigned Division
            // Syntax: [Instruction] [Source], [Source]
            "divu" => {
                let reg = parse_reg(fields[0]);
                let reg_2 = parse_reg(fields[1]);

                let div_opr = registers.get(reg) as u64;
                let div_opr_2 = registers.get(reg_2) as u64;

                let quotient = div_opr.wrapping_div(div_opr_2);
                let remainder = div_opr.wrapping_rem(div_opr_2);

                registers.set_hilo(remainder << 32 | quotient);
            }
            _ => {
                eprintln!("Opcode not found: {}", opc);
                break;
            }
        }

        println!(
            "R10: {}, R11: {}, R9: {}, R8: {}",
            registers.get(10),
            registers.get(11),
            registers.get(9),
            registers.get(8)
        );
    }
}

fn run_file() {
    let content = fs::read_to_string("src/test.asm").expect("Failed to read file");
    let input_lines: Vec<&str> = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
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
