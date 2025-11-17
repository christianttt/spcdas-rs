use crate::instruction::{DecodedInstruction, DecodedOperand};
use crate::spc::{Config, Spc};

pub fn format_instruction(instr: &DecodedInstruction, spc: &Spc) -> String {
    let prefix = format_prefix(instr, spc);
    let disassembly = format_disassembly(instr, &spc.config);
    format!("{}{}", prefix, disassembly).trim_end().to_string()
}

fn format_prefix(instr: &DecodedInstruction, spc: &Spc) -> String {
    let mut prefix = String::new();
    let config = &spc.config;
    if config.show_addr {
        prefix.push_str(&format!("{:04x}: ", instr.address));
    }
    if config.show_hex {
        let mut hex_part = String::new();
        let len = instr.definition.len as usize;
        for i in 0..len {
            let byte = spc.read_byte(instr.address.wrapping_add(i as u16));
            hex_part.push_str(&format!("{:02x} ", byte));
        }
        prefix.push_str(&format!("{:<10}", hex_part));
    }
    prefix
}

fn format_disassembly(instr: &DecodedInstruction, config: &Config) -> String {
    let mne = instr.definition.mnemonic;
    let opcode = instr.definition.opcode;
    let len = instr.definition.len;
    let pc = instr.address;

    let operand_str = match &instr.operand {
        DecodedOperand::None => match opcode {
            0x0D => "psw".to_string(),
            0x2D => "a".to_string(),
            0x4D => "x".to_string(),
            0x6D => "y".to_string(),
            0x8E => "psw".to_string(),
            0xAE => "a".to_string(),
            0xCE => "x".to_string(),
            0xEE => "y".to_string(),
            0x1C | 0x3C | 0x5C | 0x7C | 0x9C | 0xBC => "a".to_string(),
            0x1D => "x".to_string(),
            0x3D => "x".to_string(),
            0xDC => "y".to_string(),
            0xFC => "y".to_string(),
            0x5D => "x,a".to_string(),
            0x7D => "a,x".to_string(),
            0x9D => "x,sp".to_string(),
            0xBD => "sp,x".to_string(),
            0xDD => "a,y".to_string(),
            0xFD => "y,a".to_string(),
            0xBE => "a".to_string(),
            0xDF => "a".to_string(),
            0x9F => "a".to_string(),
            0xCF => "ya".to_string(),
            0x9E => "ya,x".to_string(),
            0x19 | 0x39 | 0x59 | 0x79 | 0x99 | 0xB9 => "(x),(y)".to_string(),
            _ => "".to_string(),
        },
        DecodedOperand::TCall(n) => format!("{}", n),
        DecodedOperand::PCall(addr) => format!("${:02x}", addr),
        DecodedOperand::Byte(val) => match opcode {
            0xC8 | 0xCD => format!("x,#${:02x}", val),
            0x8D | 0xAD => format!("y,#${:02x}", val),
            _ => format!("a,#${:02x}", val),
        },
        DecodedOperand::Relative(offset) => {
            let target_str = if config.resolve_rel {
                let target = pc
                    .wrapping_add(len as u16)
                    .wrapping_add(*offset as i16 as u16);
                format!("${:04x}", target)
            } else {
                format!("${:02x}", *offset as u8)
            };
            if opcode == 0xFE {
                format!("y,{}", target_str)
            } else {
                target_str
            }
        }
        DecodedOperand::Direct(addr) => match opcode {
            0x1A | 0x3A => format!("${:02x}", addr),
            0x5A | 0x7A | 0x9A | 0xBA => format!("ya,${:02x}", addr),
            0xDA => format!("${:02x},ya", addr),

            0x3E | 0xF8 => format!("x,${:02x}", addr),

            0x7E | 0xEB => format!("y,${:02x}", addr),

            0xC4 => format!("${:02x},a", addr),
            0xCB => format!("${:02x},y", addr),
            0xD8 => format!("${:02x},x", addr),

            0x04 | 0x24 | 0x44 | 0x64 | 0x84 | 0xA4 | 0xE4 => format!("a,${:02x}", addr),

            _ => format!("${:02x}", addr),
        },
        DecodedOperand::Absolute(addr) => match mne {
            "jmp" | "call" => format!("${:04x}", addr),

            "mov" => match opcode {
                0xC5 => format!("${:04x},a", addr),
                0xC9 => format!("${:04x},x", addr),
                0xCC => format!("${:04x},y", addr),
                0xE5 => format!("a,${:04x}", addr),
                0xE9 => format!("x,${:04x}", addr),
                0xEC => format!("y,${:04x}", addr),
                _ => format!("${:04x}", addr),
            },

            "cmp" => match opcode {
                0x1E => format!("x,${:04x}", addr),
                0x5E => format!("y,${:04x}", addr),
                _ => format!("a,${:04x}", addr),
            },

            "asl" | "dec" | "inc" | "lsr" | "rol" | "ror" => format!("${:04x}", addr),

            "tset1" | "tclr1" => format!("${:04x}", addr),

            _ => format!("a,${:04x}", addr),
        },
        DecodedOperand::DirectX(addr) => match mne {
            "mov" => match opcode {
                0xFB => format!("y,${:02x}+x", addr),
                0xDB => format!("${:02x}+x,y", addr),
                0xD4 => format!("${:02x}+x,a", addr),
                _ => format!("a,${:02x}+x", addr),
            },
            "adc" | "and" | "cmp" | "eor" | "or" | "sbc" => format!("a,${:02x}+x", addr),
            _ => format!("${:02x}+x", addr),
        },
        DecodedOperand::DirectY(addr) => match mne {
            "mov" => match opcode {
                0xD9 => format!("${:02x}+y,x", addr),
                _ => format!("x,${:02x}+y", addr),
            },
            _ => format!("a,${:02x}+y", addr),
        },
        DecodedOperand::AbsoluteX(addr) => match mne {
            "jmp" => format!("(${:04x}+x)", addr),
            "mov" => match opcode {
                0xD5 => format!("${:04x}+x,a", addr),
                _ => format!("a,${:04x}+x", addr),
            },
            _ => format!("a,${:04x}+x", addr),
        },
        DecodedOperand::AbsoluteY(addr) => match mne {
            "mov" => match opcode {
                0xD6 => format!("${:04x}+y,a", addr),
                _ => format!("a,${:04x}+y", addr),
            },
            _ => format!("a,${:04x}+y", addr),
        },
        DecodedOperand::IndirectX(addr) => match mne {
            "mov" => match opcode {
                0xC7 => format!("(${:02x}+x),a", addr),
                _ => format!("a,(${:02x}+x)", addr),
            },
            _ => format!("a,(${:02x}+x)", addr),
        },
        DecodedOperand::IndirectY(addr) => match mne {
            "mov" => match opcode {
                0xD7 => format!("(${:02x})+y,a", addr),
                _ => format!("a,(${:02x})+y", addr),
            },
            _ => format!("a,(${:02x})+y", addr),
        },

        DecodedOperand::Indirect => match opcode {
            0xC6 => "(x),a".to_string(),
            _ => "a,(x)".to_string(),
        },
        DecodedOperand::IndirectAutoInc => match opcode {
            0xAF => "(x)+,a".to_string(),
            0xBF => "a,(x)+".to_string(),
            _ => "???".to_string(),
        },

        DecodedOperand::DirectBit { addr } => format!("${:02x}", addr),
        DecodedOperand::DirectBitRelative { addr, offset } => {
            let branch = if config.resolve_rel {
                let target = pc
                    .wrapping_add(len as u16)
                    .wrapping_add(*offset as i16 as u16);
                format!("${:04x}", target)
            } else {
                format!("${:02x}", *offset as u8)
            };
            format!("${:02x},{}", addr, branch)
        }
        DecodedOperand::MemoryBit { addr, bit } => {
            let base = format!("${:04x},{}", addr, bit);
            match mne {
                "or1" | "and1" | "eor1" => {
                    if (opcode & 0x20) != 0 {
                        format!("c,!({})", base)
                    } else {
                        format!("c,{}", base)
                    }
                }
                "mov1" => {
                    if opcode == 0xAA {
                        format!("c,{}", base)
                    } else {
                        format!("{},c", base)
                    }
                }
                "not1" => base,
                _ => "???".to_string(),
            }
        }
        DecodedOperand::DpToDp { dest, src } => format!("(${:02x}),(${:02x})", dest, src),
        DecodedOperand::DpImm { addr, imm } => format!("${:02x},#${:02x}", addr, imm),
        DecodedOperand::DirectRelative { addr, offset } => {
            let branch = if config.resolve_rel {
                let target = pc
                    .wrapping_add(len as u16)
                    .wrapping_add(*offset as i16 as u16);
                format!("${:04x}", target)
            } else {
                format!("${:02x}", *offset as u8)
            };
            format!("${:02x},{}", addr, branch)
        }
        DecodedOperand::DirectXRelative { addr, offset } => {
            let branch = if config.resolve_rel {
                let target = pc
                    .wrapping_add(len as u16)
                    .wrapping_add(*offset as i16 as u16);
                format!("${:04x}", target)
            } else {
                format!("${:02x}", *offset as u8)
            };
            format!("${:02x}+x,{}", addr, branch)
        }
    };
    if operand_str.is_empty() {
        mne.to_string()
    } else {
        format!("{:<5} {}", mne, operand_str)
    }
}
