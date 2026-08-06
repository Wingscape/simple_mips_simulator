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

fn parse_offset(field: &str) -> Option<(i32, usize)> {
    let len = match field.chars().position(|c| c == '(') {
        Some(len) => len,
        None => {
            return None;
        }
    };

    let end_len = match field.chars().position(|c| c == ')') {
        Some(end_len) => end_len,
        None => {
            return None;
        }
    };

    let reg = &field[len + 1..end_len];
    let offset_raw = &field[0..len];

    let offset = if offset_raw.starts_with("0x") {
        // that means it's hex
        i32::from_str_radix(offset_raw.trim_start_matches("0x"), 16).unwrap_or(0)
    } else {
        offset_raw.parse().unwrap_or(0)
    };

    Some((offset, parse_reg(reg)))
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
    let mut memory: [u8; 1024] = [0; 1024];

    let mut jmp = false;
    let mut jmp_pc: usize = 0;

    while pc < lines.len() {
        println!("pc: {}", pc);

        // #1: fetch the next instruction
        let (opc, layout_field) = lines[pc]
            .split_once(char::is_whitespace)
            .unwrap_or(("", ""));

        let layout_field = layout_field.trim();
        let fields: Vec<&str> = layout_field.split(",").map(|field| field.trim()).collect();

        // #2: increment the pc
        if jmp {
            jmp = false;
            pc = jmp_pc;
        } else {
            pc += 1;
        }

        // #3: execute the instruction
        match opc {
            // TODO: shorthand ori
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
                let imm = parse_imm(fields[2]) & 31;

                registers.set(dest, registers.get(reg) << imm);
            }
            // Syntax: [Instruction] [Destination], [Source], [Imm]
            "srl" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let imm = parse_imm(fields[2]) & 31;

                registers.set(dest, registers.get(reg) >> imm);
            }
            // Syntax: [Instruction] [Destination], [Source], [Imm]
            // actually 0..31 is the range that rust can give for
            // because how the CPU works for only contraint to 5 circuits
            // why? cause moving 32 bits, while leaving it all 0, is such a waste
            "sra" => {
                let dest = parse_reg(fields[0]);
                let reg = parse_reg(fields[1]);
                let imm = parse_imm(fields[2]) & 31;

                registers.set(dest, (registers.get(reg) as i32 >> imm) as u32);
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
            // As with the lw instruction, the memory address must be word aligned (a multiple of four).
            // Syntax: [Instruction] [Destination], [Offset([Source])]
            "lw" => {
                let dest = parse_reg(fields[0]);
                let (offset, base_reg) = match parse_offset(fields[1]) {
                    Some((offset, base_reg)) => (offset, base_reg),
                    None => {
                        eprintln!("Offset field weird");
                        break;
                    }
                };

                let addr = (registers.get(base_reg).wrapping_add(offset as u32)) as usize;

                if addr % 4 != 0 {
                    eprintln!("unaligned address");
                    break;
                }

                if addr + 3 >= memory.len() {
                    eprintln!(
                        "address is out of bounds, which what we currently have is: {}",
                        memory.len()
                    );
                    break;
                }

                let value: u32 = 0;
                let step_1 = (value | memory[addr + 3] as u32) << 8;
                let step_2 = (step_1 | memory[addr + 2] as u32) << 8;
                let step_3 = (step_2 | memory[addr + 1] as u32) << 8;
                let step_4 = step_3 | memory[addr] as u32;

                registers.set(dest, step_4);
                // TODO: load delay implementation
            }
            // Syntax: [Instruction] [Source], [Offset([Source])]
            "sw" => {
                let reg = parse_reg(fields[0]);
                let (offset, base_reg) = match parse_offset(fields[1]) {
                    Some((offset, base_reg)) => (offset, base_reg),
                    None => {
                        eprintln!("Offset field weird");
                        break;
                    }
                };

                let addr = (registers.get(base_reg).wrapping_add(offset as u32)) as usize;

                if addr % 4 != 0 {
                    eprintln!("unaligned address");
                    break;
                }

                if addr + 3 >= memory.len() {
                    eprintln!(
                        "address is out of bounds, which what we currently have is: {}",
                        memory.len()
                    );
                    break;
                }

                let value = registers.get(reg);
                let byte_1 = value as u8;
                let byte_2 = (value >> 8) as u8;
                let byte_3 = (value >> 16) as u8;
                let byte_4 = (value >> 24) as u8;

                memory[addr] = byte_1;
                memory[addr + 1] = byte_2;
                memory[addr + 2] = byte_3;
                memory[addr + 3] = byte_4;
            }
            // load upper immediate 2 bytes of 32 bits
            // Syntax: [Instruction] [Destination], [Immediate]
            "lui" => {
                let dest = parse_reg(fields[0]);
                let imm = parse_imm(fields[1]) << 16;

                registers.set(dest, imm);
            }
            // Syntax: [Instruction] [Destination], [Offset([Source])]
            "lb" => {
                let dest = parse_reg(fields[0]);
                let (offset, base_reg) = match parse_offset(fields[1]) {
                    Some((offset, base_reg)) => (offset, base_reg),
                    None => {
                        eprintln!("Offset field weird");
                        break;
                    }
                };

                let addr = (registers.get(base_reg).wrapping_add(offset as u32)) as usize;

                if addr >= memory.len() {
                    eprintln!(
                        "address is out of bounds, which what we currently have is: {}",
                        memory.len()
                    );
                    break;
                }

                let value = (memory[addr] as i8) as i32;
                registers.set(dest, value as u32);
            }
            // Syntax: [Instruction] [Destination], [Offset([Source])]
            "lbu" => {
                let dest = parse_reg(fields[0]);
                let (offset, base_reg) = match parse_offset(fields[1]) {
                    Some((offset, base_reg)) => (offset, base_reg),
                    None => {
                        eprintln!("Offset field weird");
                        break;
                    }
                };

                let addr = (registers.get(base_reg).wrapping_add(offset as u32)) as usize;
                let value = memory[addr] as u32;
                registers.set(dest, value);
            }
            // Syntax: [Instruction] [Source], [Offset([Source])]
            "sb" => {
                let reg = parse_reg(fields[0]);
                let (offset, base_reg) = match parse_offset(fields[1]) {
                    Some((offset, base_reg)) => (offset, base_reg),
                    None => {
                        eprintln!("Offset field weird");
                        break;
                    }
                };

                let addr = (registers.get(base_reg).wrapping_add(offset as u32)) as usize;
                let value = registers.get(reg) as u8;
                memory[addr] = value;
            }
            // As with the lh instruction, the memory address must be halfword aligned (a multiple of two).
            // Syntax: [Instruction] [Destination], [Offset([Source])]
            "lh" => {
                let dest = parse_reg(fields[0]);
                let (offset, base_reg) = match parse_offset(fields[1]) {
                    Some((offset, base_reg)) => (offset, base_reg),
                    None => {
                        eprintln!("Offset field weird");
                        break;
                    }
                };

                let addr = (registers.get(base_reg).wrapping_add(offset as u32)) as usize;

                if addr % 2 != 0 {
                    eprintln!("unaligned address");
                    break;
                }

                if addr + 1 >= memory.len() {
                    eprintln!(
                        "address is out of bounds, which what we currently have is: {}",
                        memory.len()
                    );
                    break;
                }

                let value: u16 = 0;
                let step_1 = (value | memory[addr + 1] as u16) << 8;
                let step_2 = step_1 | memory[addr] as u16;

                registers.set(dest, ((step_2 as i16) as i32) as u32);
            }
            // As with the lhu instruction, the memory address must be halfword aligned (a multiple of two).
            // Syntax: [Instruction] [Destination], [Offset([Source])]
            "lhu" => {
                let dest = parse_reg(fields[0]);
                let (offset, base_reg) = match parse_offset(fields[1]) {
                    Some((offset, base_reg)) => (offset, base_reg),
                    None => {
                        eprintln!("Offset field weird");
                        break;
                    }
                };

                let addr = (registers.get(base_reg).wrapping_add(offset as u32)) as usize;

                if addr % 2 != 0 {
                    eprintln!("unaligned address");
                    break;
                }

                if addr + 1 >= memory.len() {
                    eprintln!(
                        "address is out of bounds, which what we currently have is: {}",
                        memory.len()
                    );
                    break;
                }

                let value: u16 = 0;
                let step_1 = (value | memory[addr + 1] as u16) << 8;
                let step_2 = step_1 | memory[addr] as u16;

                registers.set(dest, step_2 as u32);
            }
            // Syntax: [Instruction] [Source], [Offset([Source])]
            "sh" => {
                let reg = parse_reg(fields[0]);
                let (offset, base_reg) = match parse_offset(fields[1]) {
                    Some((offset, base_reg)) => (offset, base_reg),
                    None => {
                        eprintln!("Offset field weird");
                        break;
                    }
                };

                let addr = (registers.get(base_reg).wrapping_add(offset as u32)) as usize;

                if addr % 2 != 0 {
                    eprintln!("unaligned address");
                    break;
                }

                if addr + 1 >= memory.len() {
                    eprintln!(
                        "address is out of bounds, which what we currently have is: {}",
                        memory.len()
                    );
                    break;
                }

                let value = registers.get(reg);
                let byte_1 = value as u8;
                let byte_2 = (value >> 8) as u8;

                memory[addr] = byte_1;
                memory[addr + 1] = byte_2;
            }
            // Syntax: [Instruction] [Target]
            "j" => match jmp_labels.get(fields[0]) {
                Some(value) => {
                    jmp = true;
                    jmp_pc = *value
                }
                _ => {
                    eprintln!("Label not found: {}", fields[2]);
                    break;
                }
            },
            // Syntax: [Instruction] [Source], [Source], [Target]
            "beq" => {
                let opr_1 = parse_reg(fields[0]);
                let opr_2 = parse_reg(fields[1]);

                if registers.get(opr_1) == registers.get(opr_2) {
                    match jmp_labels.get(fields[2]) {
                        Some(value) => {
                            jmp = true;
                            jmp_pc = *value
                        }
                        _ => {
                            eprintln!("Label not found: {}", fields[2]);
                            break;
                        }
                    }
                }
            }
            _ => {
                eprintln!("Opcode not found: {}", opc);
                break;
            }
        }

        println!("R9: {}, R8: {}", registers.get(9), registers.get(8));
    }

    for curr in 16..16 + 4 {
        println!("the value address of {}: {}", curr, memory[curr]);
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
