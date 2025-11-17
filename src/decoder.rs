use crate::instruction::{self, AddressingModeKind, DecodedInstruction, DecodedOperand};
use crate::spc::Spc;

#[derive(Debug)]
pub struct DecodeError {
    pub pc: u16,
    pub opcode: u8,
}

pub fn decode_one(spc: &Spc, pc: u16) -> Result<DecodedInstruction, DecodeError> {
    let opcode = spc.read_byte(pc);
    let definition = &instruction::OPCODES[opcode as usize];

    if let AddressingModeKind::Invalid = definition.mode {
        return Err(DecodeError { pc, opcode });
    }

    let operand_b = || spc.read_byte(pc.wrapping_add(1));
    let operand_w = || spc.read_word(pc.wrapping_add(1));
    let operand2_b = || spc.read_byte(pc.wrapping_add(2));

    let operand = match definition.mode {
        AddressingModeKind::Implied | AddressingModeKind::Brk => DecodedOperand::None,

        AddressingModeKind::Indirect => DecodedOperand::Indirect,
        AddressingModeKind::IndirectAutoInc => DecodedOperand::IndirectAutoInc,

        AddressingModeKind::TCall(n) => DecodedOperand::TCall(n),

        AddressingModeKind::ImmediateByte => DecodedOperand::Byte(operand_b()),
        AddressingModeKind::DirectPage => DecodedOperand::Direct(operand_b()),
        AddressingModeKind::DirectPageX => DecodedOperand::DirectX(operand_b()),
        AddressingModeKind::DirectPageY => DecodedOperand::DirectY(operand_b()),
        AddressingModeKind::IndirectX => DecodedOperand::IndirectX(operand_b()),
        AddressingModeKind::IndirectY => DecodedOperand::IndirectY(operand_b()),
        AddressingModeKind::Relative => DecodedOperand::Relative(operand_b() as i8),
        AddressingModeKind::ImpliedRelative => DecodedOperand::Relative(operand_b() as i8),
        AddressingModeKind::PCall => DecodedOperand::PCall(operand_b()),

        AddressingModeKind::Absolute => DecodedOperand::Absolute(operand_w()),
        AddressingModeKind::AbsoluteX => DecodedOperand::AbsoluteX(operand_w()),
        AddressingModeKind::AbsoluteY => DecodedOperand::AbsoluteY(operand_w()),

        AddressingModeKind::DirectPageBit => DecodedOperand::DirectBit { addr: operand_b() },
        AddressingModeKind::DirectPageBitRelative => DecodedOperand::DirectBitRelative {
            addr: operand_b(),
            offset: operand2_b() as i8,
        },
        AddressingModeKind::MemoryBit | AddressingModeKind::MemoryBitNegated => {
            let addr_word = operand_w();
            let addr = addr_word >> 3;
            let bit = (addr_word & 0x07) as u8;
            DecodedOperand::MemoryBit { addr, bit }
        }

        AddressingModeKind::DirectPageToDirectPage => DecodedOperand::DpToDp {
            src: operand_b(),
            dest: operand2_b(),
        },
        AddressingModeKind::DirectPageImmediate => DecodedOperand::DpImm {
            imm: operand_b(),
            addr: operand2_b(),
        },
        AddressingModeKind::DirectPageRelative => DecodedOperand::DirectRelative {
            addr: operand_b(),
            offset: operand2_b() as i8,
        },
        AddressingModeKind::DirectPageXRelative => DecodedOperand::DirectXRelative {
            addr: operand_b(),
            offset: operand2_b() as i8,
        },

        AddressingModeKind::Invalid => unreachable!(),
    };

    Ok(DecodedInstruction {
        address: pc,
        definition,
        operand,
    })
}
