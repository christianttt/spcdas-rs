#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingModeKind {
    Implied,
    ImmediateByte,
    DirectPage,
    Absolute,
    DirectPageX,
    DirectPageY,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Indirect,
    IndirectAutoInc,
    Relative,
    DirectPageBit,
    DirectPageBitRelative,
    MemoryBit,
    MemoryBitNegated,
    DirectPageToDirectPage,
    DirectPageImmediate,
    DirectPageRelative,
    DirectPageXRelative,
    ImpliedRelative,
    TCall(u8),
    PCall,
    Brk,
    Invalid,
}

#[derive(Debug, Clone, Copy)]
pub struct OpcodeDef {
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub mode: AddressingModeKind,
    pub len: u8,
    #[allow(dead_code)]
    pub cycles: u8,
}

const INVALID_OP: OpcodeDef = OpcodeDef {
    opcode: 0,
    mnemonic: "invalid",
    mode: AddressingModeKind::Invalid,
    len: 1,
    cycles: 1,
};

pub static OPCODES: [OpcodeDef; 256] = {
    macro_rules! op {
        ($opcode:expr, $mne:expr, $mode:expr, $len:expr, $cyc:expr) => {
            OpcodeDef {
                opcode: $opcode,
                mnemonic: $mne,
                mode: $mode,
                len: $len,
                cycles: $cyc,
            }
        };
    }
    use AddressingModeKind::*;

    let mut table = [INVALID_OP; 256];

    // 0x00 - 0x0F
    table[0x00] = op!(0x00, "nop", Implied, 1, 2);
    table[0x01] = op!(0x01, "tcall", TCall(0), 1, 8);
    table[0x02] = op!(0x02, "set0", DirectPageBit, 2, 4);
    table[0x03] = op!(0x03, "bbs0", DirectPageBitRelative, 3, 5);
    table[0x04] = op!(0x04, "or", DirectPage, 2, 3);
    table[0x05] = op!(0x05, "or", Absolute, 3, 4);
    table[0x06] = op!(0x06, "or", Indirect, 1, 3);
    table[0x07] = op!(0x07, "or", IndirectX, 2, 6);
    table[0x08] = op!(0x08, "or", ImmediateByte, 2, 2);
    table[0x09] = op!(0x09, "or", DirectPageToDirectPage, 3, 6);
    table[0x0A] = op!(0x0A, "or1", MemoryBit, 3, 5);
    table[0x0B] = op!(0x0B, "asl", DirectPage, 2, 4);
    table[0x0C] = op!(0x0C, "asl", Absolute, 3, 5);
    table[0x0D] = op!(0x0D, "push", Implied, 1, 4);
    table[0x0E] = op!(0x0E, "tset1", Absolute, 3, 6);
    table[0x0F] = op!(0x0F, "brk", Brk, 1, 8);
    // 0x10 - 0x1F
    table[0x10] = op!(0x10, "bpl", Relative, 2, 2);
    table[0x11] = op!(0x11, "tcall", TCall(1), 1, 8);
    table[0x12] = op!(0x12, "clr0", DirectPageBit, 2, 4);
    table[0x13] = op!(0x13, "bbc0", DirectPageBitRelative, 3, 5);
    table[0x14] = op!(0x14, "or", DirectPageX, 2, 4);
    table[0x15] = op!(0x15, "or", AbsoluteX, 3, 5);
    table[0x16] = op!(0x16, "or", AbsoluteY, 3, 5);
    table[0x17] = op!(0x17, "or", IndirectY, 2, 6);
    table[0x18] = op!(0x18, "or", DirectPageImmediate, 3, 5);
    table[0x19] = op!(0x19, "or", Implied, 1, 5);
    table[0x1A] = op!(0x1A, "decw", DirectPage, 2, 6);
    table[0x1B] = op!(0x1B, "asl", DirectPageX, 2, 5);
    table[0x1C] = op!(0x1C, "asl", Implied, 1, 2);
    table[0x1D] = op!(0x1D, "dec", Implied, 1, 2);
    table[0x1E] = op!(0x1E, "cmp", Absolute, 3, 4);
    table[0x1F] = op!(0x1F, "jmp", AbsoluteX, 3, 6);
    // 0x20 - 0x2F
    table[0x20] = op!(0x20, "clrp", Implied, 1, 2);
    table[0x21] = op!(0x21, "tcall", TCall(2), 1, 8);
    table[0x22] = op!(0x22, "set1", DirectPageBit, 2, 4);
    table[0x23] = op!(0x23, "bbs1", DirectPageBitRelative, 3, 5);
    table[0x24] = op!(0x24, "and", DirectPage, 2, 3);
    table[0x25] = op!(0x25, "and", Absolute, 3, 4);
    table[0x26] = op!(0x26, "and", Indirect, 1, 3);
    table[0x27] = op!(0x27, "and", IndirectX, 2, 6);
    table[0x28] = op!(0x28, "and", ImmediateByte, 2, 2);
    table[0x29] = op!(0x29, "and", DirectPageToDirectPage, 3, 6);
    table[0x2A] = op!(0x2A, "or1", MemoryBitNegated, 3, 5);
    table[0x2B] = op!(0x2B, "rol", DirectPage, 2, 4);
    table[0x2C] = op!(0x2C, "rol", Absolute, 3, 5);
    table[0x2D] = op!(0x2D, "push", Implied, 1, 4);
    table[0x2E] = op!(0x2E, "cbne", DirectPageRelative, 3, 5);
    table[0x2F] = op!(0x2F, "bra", Relative, 2, 4);
    // 0x30 - 0x3F
    table[0x30] = op!(0x30, "bmi", Relative, 2, 2);
    table[0x31] = op!(0x31, "tcall", TCall(3), 1, 8);
    table[0x32] = op!(0x32, "clr1", DirectPageBit, 2, 4);
    table[0x33] = op!(0x33, "bbc1", DirectPageBitRelative, 3, 5);
    table[0x34] = op!(0x34, "and", DirectPageX, 2, 4);
    table[0x35] = op!(0x35, "and", AbsoluteX, 3, 5);
    table[0x36] = op!(0x36, "and", AbsoluteY, 3, 5);
    table[0x37] = op!(0x37, "and", IndirectY, 2, 6);
    table[0x38] = op!(0x38, "and", DirectPageImmediate, 3, 5);
    table[0x39] = op!(0x39, "and", Implied, 1, 5);
    table[0x3A] = op!(0x3A, "incw", DirectPage, 2, 6);
    table[0x3B] = op!(0x3B, "rol", DirectPageX, 2, 5);
    table[0x3C] = op!(0x3C, "rol", Implied, 1, 2);
    table[0x3D] = op!(0x3D, "inc", Implied, 1, 2);
    table[0x3E] = op!(0x3E, "cmp", DirectPage, 2, 3);
    table[0x3F] = op!(0x3F, "call", Absolute, 3, 8);
    // 0x40 - 0x4F
    table[0x40] = op!(0x40, "setp", Implied, 1, 2);
    table[0x41] = op!(0x41, "tcall", TCall(4), 1, 8);
    table[0x42] = op!(0x42, "set2", DirectPageBit, 2, 4);
    table[0x43] = op!(0x43, "bbs2", DirectPageBitRelative, 3, 5);
    table[0x44] = op!(0x44, "eor", DirectPage, 2, 3);
    table[0x45] = op!(0x45, "eor", Absolute, 3, 4);
    table[0x46] = op!(0x46, "eor", Indirect, 1, 3);
    table[0x47] = op!(0x47, "eor", IndirectX, 2, 6);
    table[0x48] = op!(0x48, "eor", ImmediateByte, 2, 2);
    table[0x49] = op!(0x49, "eor", DirectPageToDirectPage, 3, 6);
    table[0x4A] = op!(0x4A, "and1", MemoryBit, 3, 4);
    table[0x4B] = op!(0x4B, "lsr", DirectPage, 2, 4);
    table[0x4C] = op!(0x4C, "lsr", Absolute, 3, 5);
    table[0x4D] = op!(0x4D, "push", Implied, 1, 4);
    table[0x4E] = op!(0x4E, "tclr1", Absolute, 3, 6);
    table[0x4F] = op!(0x4F, "pcall", PCall, 2, 6);
    // 0x50 - 0x5F
    table[0x50] = op!(0x50, "bvc", Relative, 2, 2);
    table[0x51] = op!(0x51, "tcall", TCall(5), 1, 8);
    table[0x52] = op!(0x52, "clr2", DirectPageBit, 2, 4);
    table[0x53] = op!(0x53, "bbc2", DirectPageBitRelative, 3, 5);
    table[0x54] = op!(0x54, "eor", DirectPageX, 2, 4);
    table[0x55] = op!(0x55, "eor", AbsoluteX, 3, 5);
    table[0x56] = op!(0x56, "eor", AbsoluteY, 3, 5);
    table[0x57] = op!(0x57, "eor", IndirectY, 2, 6);
    table[0x58] = op!(0x58, "eor", DirectPageImmediate, 3, 5);
    table[0x59] = op!(0x59, "eor", Implied, 1, 5);
    table[0x5A] = op!(0x5A, "cmpw", DirectPage, 2, 4);
    table[0x5B] = op!(0x5B, "lsr", DirectPageX, 2, 5);
    table[0x5C] = op!(0x5C, "lsr", Implied, 1, 2);
    table[0x5D] = op!(0x5D, "mov", Implied, 1, 2);
    table[0x5E] = op!(0x5E, "cmp", Absolute, 3, 4);
    table[0x5F] = op!(0x5F, "jmp", Absolute, 3, 3);
    // 0x60 - 0x6F
    table[0x60] = op!(0x60, "clrc", Implied, 1, 2);
    table[0x61] = op!(0x61, "tcall", TCall(6), 1, 8);
    table[0x62] = op!(0x62, "set3", DirectPageBit, 2, 4);
    table[0x63] = op!(0x63, "bbs3", DirectPageBitRelative, 3, 5);
    table[0x64] = op!(0x64, "cmp", DirectPage, 2, 3);
    table[0x65] = op!(0x65, "cmp", Absolute, 3, 4);
    table[0x66] = op!(0x66, "cmp", Indirect, 1, 3);
    table[0x67] = op!(0x67, "cmp", IndirectX, 2, 6);
    table[0x68] = op!(0x68, "cmp", ImmediateByte, 2, 2);
    table[0x69] = op!(0x69, "cmp", DirectPageToDirectPage, 3, 6);
    table[0x6A] = op!(0x6A, "and1", MemoryBitNegated, 3, 4);
    table[0x6B] = op!(0x6B, "ror", DirectPage, 2, 4);
    table[0x6C] = op!(0x6C, "ror", Absolute, 3, 5);
    table[0x6D] = op!(0x6D, "push", Implied, 1, 4);
    table[0x6E] = op!(0x6E, "dbnz", DirectPageRelative, 3, 5);
    table[0x6F] = op!(0x6F, "ret", Implied, 1, 5);
    // 0x70 - 0x7F
    table[0x70] = op!(0x70, "bvs", Relative, 2, 2);
    table[0x71] = op!(0x71, "tcall", TCall(7), 1, 8);
    table[0x72] = op!(0x72, "clr3", DirectPageBit, 2, 4);
    table[0x73] = op!(0x73, "bbc3", DirectPageBitRelative, 3, 5);
    table[0x74] = op!(0x74, "cmp", DirectPageX, 2, 4);
    table[0x75] = op!(0x75, "cmp", AbsoluteX, 3, 5);
    table[0x76] = op!(0x76, "cmp", AbsoluteY, 3, 5);
    table[0x77] = op!(0x77, "cmp", IndirectY, 2, 6);
    table[0x78] = op!(0x78, "cmp", DirectPageImmediate, 3, 5);
    table[0x79] = op!(0x79, "cmp", Implied, 1, 5);
    table[0x7A] = op!(0x7A, "addw", DirectPage, 2, 5);
    table[0x7B] = op!(0x7B, "ror", DirectPageX, 2, 5);
    table[0x7C] = op!(0x7C, "ror", Implied, 1, 2);
    table[0x7D] = op!(0x7D, "mov", Implied, 1, 2);
    table[0x7E] = op!(0x7E, "cmp", DirectPage, 2, 3);
    table[0x7F] = op!(0x7F, "reti", Implied, 1, 6);
    // 0x80 - 0x8F
    table[0x80] = op!(0x80, "setc", Implied, 1, 2);
    table[0x81] = op!(0x81, "tcall", TCall(8), 1, 8);
    table[0x82] = op!(0x82, "set4", DirectPageBit, 2, 4);
    table[0x83] = op!(0x83, "bbs4", DirectPageBitRelative, 3, 5);
    table[0x84] = op!(0x84, "adc", DirectPage, 2, 3);
    table[0x85] = op!(0x85, "adc", Absolute, 3, 4);
    table[0x86] = op!(0x86, "adc", Indirect, 1, 3);
    table[0x87] = op!(0x87, "adc", IndirectX, 2, 6);
    table[0x88] = op!(0x88, "adc", ImmediateByte, 2, 2);
    table[0x89] = op!(0x89, "adc", DirectPageToDirectPage, 3, 6);
    table[0x8A] = op!(0x8A, "eor1", MemoryBit, 3, 5);
    table[0x8B] = op!(0x8B, "dec", DirectPage, 2, 4);
    table[0x8C] = op!(0x8C, "dec", Absolute, 3, 5);
    table[0x8D] = op!(0x8D, "mov", ImmediateByte, 2, 2);
    table[0x8E] = op!(0x8E, "pop", Implied, 1, 4);
    table[0x8F] = op!(0x8F, "mov", DirectPageImmediate, 3, 5);
    // 0x90 - 0x9F
    table[0x90] = op!(0x90, "bcc", Relative, 2, 2);
    table[0x91] = op!(0x91, "tcall", TCall(9), 1, 8);
    table[0x92] = op!(0x92, "clr4", DirectPageBit, 2, 4);
    table[0x93] = op!(0x93, "bbc4", DirectPageBitRelative, 3, 5);
    table[0x94] = op!(0x94, "adc", DirectPageX, 2, 4);
    table[0x95] = op!(0x95, "adc", AbsoluteX, 3, 5);
    table[0x96] = op!(0x96, "adc", AbsoluteY, 3, 5);
    table[0x97] = op!(0x97, "adc", IndirectY, 2, 6);
    table[0x98] = op!(0x98, "adc", DirectPageImmediate, 3, 5);
    table[0x99] = op!(0x99, "adc", Implied, 1, 5);
    table[0x9A] = op!(0x9A, "subw", DirectPage, 2, 5);
    table[0x9B] = op!(0x9B, "dec", DirectPageX, 2, 5);
    table[0x9C] = op!(0x9C, "dec", Implied, 1, 2);
    table[0x9D] = op!(0x9D, "mov", Implied, 1, 2);
    table[0x9E] = op!(0x9E, "div", Implied, 1, 12);
    table[0x9F] = op!(0x9F, "xcn", Implied, 1, 5);
    // 0xA0 - 0xAF
    table[0xA0] = op!(0xA0, "ei", Implied, 1, 3);
    table[0xA1] = op!(0xA1, "tcall", TCall(10), 1, 8);
    table[0xA2] = op!(0xA2, "set5", DirectPageBit, 2, 4);
    table[0xA3] = op!(0xA3, "bbs5", DirectPageBitRelative, 3, 5);
    table[0xA4] = op!(0xA4, "sbc", DirectPage, 2, 3);
    table[0xA5] = op!(0xA5, "sbc", Absolute, 3, 4);
    table[0xA6] = op!(0xA6, "sbc", Indirect, 1, 3);
    table[0xA7] = op!(0xA7, "sbc", IndirectX, 2, 6);
    table[0xA8] = op!(0xA8, "sbc", ImmediateByte, 2, 2);
    table[0xA9] = op!(0xA9, "sbc", DirectPageToDirectPage, 3, 6);
    table[0xAA] = op!(0xAA, "mov1", MemoryBit, 3, 4);
    table[0xAB] = op!(0xAB, "inc", DirectPage, 2, 4);
    table[0xAC] = op!(0xAC, "inc", Absolute, 3, 5);
    table[0xAD] = op!(0xAD, "cmp", ImmediateByte, 2, 2);
    table[0xAE] = op!(0xAE, "pop", Implied, 1, 4);
    table[0xAF] = op!(0xAF, "mov", IndirectAutoInc, 1, 4);
    // 0xB0 - 0xBF
    table[0xB0] = op!(0xB0, "bcs", Relative, 2, 2);
    table[0xB1] = op!(0xB1, "tcall", TCall(11), 1, 8);
    table[0xB2] = op!(0xB2, "clr5", DirectPageBit, 2, 4);
    table[0xB3] = op!(0xB3, "bbc5", DirectPageBitRelative, 3, 5);
    table[0xB4] = op!(0xB4, "sbc", DirectPageX, 2, 4);
    table[0xB5] = op!(0xB5, "sbc", AbsoluteX, 3, 5);
    table[0xB6] = op!(0xB6, "sbc", AbsoluteY, 3, 5);
    table[0xB7] = op!(0xB7, "sbc", IndirectY, 2, 6);
    table[0xB8] = op!(0xB8, "sbc", DirectPageImmediate, 3, 5);
    table[0xB9] = op!(0xB9, "sbc", Implied, 1, 5);
    table[0xBA] = op!(0xBA, "movw", DirectPage, 2, 5);
    table[0xBB] = op!(0xBB, "inc", DirectPageX, 2, 5);
    table[0xBC] = op!(0xBC, "inc", Implied, 1, 2);
    table[0xBD] = op!(0xBD, "mov", Implied, 1, 2);
    table[0xBE] = op!(0xBE, "das", Implied, 1, 3);
    table[0xBF] = op!(0xBF, "mov", IndirectAutoInc, 1, 4);
    // 0xC0 - 0xCF
    table[0xC0] = op!(0xC0, "di", Implied, 1, 3);
    table[0xC1] = op!(0xC1, "tcall", TCall(12), 1, 8);
    table[0xC2] = op!(0xC2, "set6", DirectPageBit, 2, 4);
    table[0xC3] = op!(0xC3, "bbs6", DirectPageBitRelative, 3, 5);
    table[0xC4] = op!(0xC4, "mov", DirectPage, 2, 4);
    table[0xC5] = op!(0xC5, "mov", Absolute, 3, 5);
    table[0xC6] = op!(0xC6, "mov", Indirect, 1, 4);
    table[0xC7] = op!(0xC7, "mov", IndirectX, 2, 7);
    table[0xC8] = op!(0xC8, "cmp", ImmediateByte, 2, 2);
    table[0xC9] = op!(0xC9, "mov", Absolute, 3, 5);
    table[0xCA] = op!(0xCA, "mov1", MemoryBit, 3, 6);
    table[0xCB] = op!(0xCB, "mov", DirectPage, 2, 4);
    table[0xCC] = op!(0xCC, "mov", Absolute, 3, 5);
    table[0xCD] = op!(0xCD, "mov", ImmediateByte, 2, 2);
    table[0xCE] = op!(0xCE, "pop", Implied, 1, 4);
    table[0xCF] = op!(0xCF, "mul", Implied, 1, 9);
    // 0xD0 - 0xDF
    table[0xD0] = op!(0xD0, "bne", Relative, 2, 2);
    table[0xD1] = op!(0xD1, "tcall", TCall(13), 1, 8);
    table[0xD2] = op!(0xD2, "clr6", DirectPageBit, 2, 4);
    table[0xD3] = op!(0xD3, "bbc6", DirectPageBitRelative, 3, 5);
    table[0xD4] = op!(0xD4, "mov", DirectPageX, 2, 5);
    table[0xD5] = op!(0xD5, "mov", AbsoluteX, 3, 6);
    table[0xD6] = op!(0xD6, "mov", AbsoluteY, 3, 6);
    table[0xD7] = op!(0xD7, "mov", IndirectY, 2, 7);
    table[0xD8] = op!(0xD8, "mov", DirectPage, 2, 4);
    table[0xD9] = op!(0xD9, "mov", DirectPageY, 2, 5);
    table[0xDA] = op!(0xDA, "movw", DirectPage, 2, 5);
    table[0xDB] = op!(0xDB, "mov", DirectPageX, 2, 5);
    table[0xDC] = op!(0xDC, "dec", Implied, 1, 2);
    table[0xDD] = op!(0xDD, "mov", Implied, 1, 2);
    table[0xDE] = op!(0xDE, "cbne", DirectPageXRelative, 3, 6);
    table[0xDF] = op!(0xDF, "daa", Implied, 1, 3);
    // 0xE0 - 0xEF
    table[0xE0] = op!(0xE0, "clrv", Implied, 1, 2);
    table[0xE1] = op!(0xE1, "tcall", TCall(14), 1, 8);
    table[0xE2] = op!(0xE2, "set7", DirectPageBit, 2, 4);
    table[0xE3] = op!(0xE3, "bbs7", DirectPageBitRelative, 3, 5);
    table[0xE4] = op!(0xE4, "mov", DirectPage, 2, 3);
    table[0xE5] = op!(0xE5, "mov", Absolute, 3, 4);
    table[0xE6] = op!(0xE6, "mov", Indirect, 1, 3);
    table[0xE7] = op!(0xE7, "mov", IndirectX, 2, 6);
    table[0xE8] = op!(0xE8, "mov", ImmediateByte, 2, 2);
    table[0xE9] = op!(0xE9, "mov", Absolute, 3, 4);
    table[0xEA] = op!(0xEA, "not1", MemoryBit, 3, 5);
    table[0xEB] = op!(0xEB, "mov", DirectPage, 2, 3);
    table[0xEC] = op!(0xEC, "mov", Absolute, 3, 4);
    table[0xED] = op!(0xED, "notc", Implied, 1, 3);
    table[0xEE] = op!(0xEE, "pop", Implied, 1, 4);
    table[0xEF] = op!(0xEF, "sleep", Implied, 1, 3);
    // 0xF0 - 0xFF
    table[0xF0] = op!(0xF0, "beq", Relative, 2, 2);
    table[0xF1] = op!(0xF1, "tcall", TCall(15), 1, 8);
    table[0xF2] = op!(0xF2, "clr7", DirectPageBit, 2, 4);
    table[0xF3] = op!(0xF3, "bbc7", DirectPageBitRelative, 3, 5);
    table[0xF4] = op!(0xF4, "mov", DirectPageX, 2, 4);
    table[0xF5] = op!(0xF5, "mov", AbsoluteX, 3, 5);
    table[0xF6] = op!(0xF6, "mov", AbsoluteY, 3, 5);
    table[0xF7] = op!(0xF7, "mov", IndirectY, 2, 6);
    table[0xF8] = op!(0xF8, "mov", DirectPage, 2, 3);
    table[0xF9] = op!(0xF9, "mov", DirectPageY, 2, 4);
    table[0xFA] = op!(0xFA, "mov", DirectPageToDirectPage, 3, 5);
    table[0xFB] = op!(0xFB, "mov", DirectPageX, 2, 4);
    table[0xFC] = op!(0xFC, "inc", Implied, 1, 2);
    table[0xFD] = op!(0xFD, "mov", Implied, 1, 2);
    table[0xFE] = op!(0xFE, "dbnz", ImpliedRelative, 2, 4);
    table[0xFF] = op!(0xFF, "stop", Implied, 1, 3);

    table
};

#[derive(Debug)]
pub enum DecodedOperand {
    None,
    Byte(u8),
    Direct(u8),
    Absolute(u16),
    DirectX(u8),
    DirectY(u8),
    AbsoluteX(u16),
    AbsoluteY(u16),
    IndirectX(u8),
    IndirectY(u8),
    Indirect,
    IndirectAutoInc,
    Relative(i8),
    DirectBit { addr: u8 },
    DirectBitRelative { addr: u8, offset: i8 },
    MemoryBit { addr: u16, bit: u8 },
    DpToDp { dest: u8, src: u8 },
    DpImm { addr: u8, imm: u8 },
    TCall(u8),
    PCall(u8),
    DirectRelative { addr: u8, offset: i8 },
    DirectXRelative { addr: u8, offset: i8 },
}

#[derive(Debug)]
pub struct DecodedInstruction {
    pub address: u16,
    pub definition: &'static OpcodeDef,
    pub operand: DecodedOperand,
}
