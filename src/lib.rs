use std::collections::HashSet;

pub mod opcodes {
    include!(concat!(env!("OUT_DIR"), "/opcodes.rs"));
}

#[cfg(test)]
mod tests;

pub mod targets;

#[derive(Debug)]
struct RawInstruction(u32);

impl RawInstruction {
    fn rd(&self) -> u8 {
        ((self.0 >> 7) & 0x1F) as u8
    }
    fn rs1(&self) -> u8 {
        ((self.0 >> 15) & 0x1F) as u8
    }
    fn rs2(&self) -> u8 {
        ((self.0 >> 20) & 0x1F) as u8
    }
    fn rs3(&self) -> u8 {
        ((self.0 >> 27) & 0x1F) as u8
    }
    fn imm_store(&self) -> i16 {
        let imm_4_0 = (self.0 >> 7) & 0x1F;
        let imm_11_5 = (self.0 >> 25) & 0x7F;

        let imm_12 = imm_4_0 | (imm_11_5 << 5);

        ((imm_12 << 4) as i16) >> 4
    }
    fn imm_load(&self) -> u16 {
        ((self.0 >> 20) & 0xFFF) as u16
    }
    fn imm_jalr(&self) -> i32 {
        let imm = (self.0 >> 20) as i32;
        (imm << 20) >> 20
    }
    fn imm_jal(&self) -> i32 {
        let inst = self.0;

        let imm20 = (inst >> 31) & 0x1;
        let imm10_1 = (inst >> 21) & 0x3FF;
        let imm11 = (inst >> 20) & 0x1;
        let imm19_12 = (inst >> 12) & 0xFF;

        let res = (imm20 << 20) | (imm19_12 << 12) | (imm11 << 11) | (imm10_1 << 1);

        ((res as i32) << 11) >> 11
    }
    fn imm_branch(&self) -> i32 {
        let inst = self.0;

        let imm12 = (inst >> 31) & 0x1;
        let imm11 = (inst >> 7) & 0x1;
        let imm10_5 = (inst >> 25) & 0x3F;
        let imm4_1 = (inst >> 8) & 0xF;

        let res = (imm12 << 12) | (imm11 << 11) | (imm10_5 << 5) | (imm4_1 << 1);

        ((res as i32) << 19) >> 19
    }
    fn imm_lui_auipc(&self) -> u32 {
        self.0 & 0xFFFFF000
    }
    fn imm_addi(&self) -> u16 {
        ((self.0 >> 20) & 0xFFF) as u16
    }
    fn fence_predecessor_successor(&self) -> (u8, u8) {
        let pred = ((self.0 >> 24) & 0xF) as u8;

        let succ = ((self.0 >> 20) & 0xF) as u8;

        (pred, succ)
    }
}

// macro_rules_attribute > macro_rules_derive
#[derive(Debug)]
pub enum Instruction {
    Add { rd: u8, rs1: u8, rs2: u8 },
    Sub { rd: u8, rs1: u8, rs2: u8 },
    Sll { rd: u8, rs1: u8, rs2: u8 },
    Slt { rd: u8, rs1: u8, rs2: u8 },
    Sltu { rd: u8, rs1: u8, rs2: u8 },
    Xor { rd: u8, rs1: u8, rs2: u8 },
    Srl { rd: u8, rs1: u8, rs2: u8 },
    Sra { rd: u8, rs1: u8, rs2: u8 },
    Or { rd: u8, rs1: u8, rs2: u8 },
    And { rd: u8, rs1: u8, rs2: u8 },

    Addi { rd: u8, rs1: u8, imm: u16 },
    Slti { rd: u8, rs1: u8, imm: u16 },
    Sltiu { rd: u8, rs1: u8, imm: u16 },
    Andi { rd: u8, rs1: u8, imm: u16 },
    Ori { rd: u8, rs1: u8, imm: u16 },
    Xori { rd: u8, rs1: u8, imm: u16 },
    Slli { rd: u8, rs1: u8, imm: u16 },
    Srli { rd: u8, rs1: u8, imm: u16 },
    Srai { rd: u8, rs1: u8, imm: u16 },

    Sb { rs1: u8, rs2: u8, imm: i16 },
    Sh { rs1: u8, rs2: u8, imm: i16 },
    Sw { rs1: u8, rs2: u8, imm: i16 },

    Lb { rd: u8, rs1: u8, imm: u16 },
    Lh { rd: u8, rs1: u8, imm: u16 },
    Lw { rd: u8, rs1: u8, imm: u16 },
    Lbu { rd: u8, rs1: u8, imm: u16 },
    Lhu { rd: u8, rs1: u8, imm: u16 },

    Beq { rs1: u8, rs2: u8, imm: i32 },
    Bne { rs1: u8, rs2: u8, imm: i32 },
    Blt { rs1: u8, rs2: u8, imm: i32 },
    Bge { rs1: u8, rs2: u8, imm: i32 },
    Bltu { rs1: u8, rs2: u8, imm: i32 },
    Bgeu { rs1: u8, rs2: u8, imm: i32 },

    Jalr { rd: u8, imm: i32 },
    Jal { rd: u8, rs1: u8, imm: i32 },

    Lui { rd: u8, imm: u32 },
    Auipc { rd: u8, imm: u32 },

    Ecall,
    Ebreak,

    Fence { pred: u8, succ: u8 },

    Ld { rd: u8, rs1: u8, imm: u16 },
    Sd { rs1: u8, rs2: u8, imm: i16 },
    Lwu { rd: u8, rs1: u8, imm: u16 },

    Addw { rd: u8, rs1: u8, rs2: u8 },
    Addiw { rd: u8, rs1: u8, imm: u16 },
    Subw { rd: u8, rs1: u8, rs2: u8 },

    Sllw { rd: u8, rs1: u8, rs2: u8 },
    Slliw { rd: u8, rs1: u8, imm: u16 },
    Srlw { rd: u8, rs1: u8, rs2: u8 },
    Srliw { rd: u8, rs1: u8, imm: u16 },
    Sraw { rd: u8, rs1: u8, rs2: u8 },
    Sraiw { rd: u8, rs1: u8, imm: u16 },

    Mul { rd: u8, rs1: u8, rs2: u8 },
    Mulh { rd: u8, rs1: u8, rs2: u8 },
    Mulhu { rd: u8, rs1: u8, rs2: u8 },
    Mulhsu { rd: u8, rs1: u8, rs2: u8 },
    Mulw { rd: u8, rs1: u8, rs2: u8 },

    Div { rd: u8, rs1: u8, rs2: u8 },
    Divu { rd: u8, rs1: u8, rs2: u8 },
    Rem { rd: u8, rs1: u8, rs2: u8 },
    Remu { rd: u8, rs1: u8, rs2: u8 },
    Divw { rd: u8, rs1: u8, rs2: u8 },
    Divuw { rd: u8, rs1: u8, rs2: u8 },
    Remw { rd: u8, rs1: u8, rs2: u8 },
    Remuw { rd: u8, rs1: u8, rs2: u8 },

    Lrw { rd: u8, rs1: u8, imm: u16 },
    Lrd { rd: u8, rs1: u8, imm: u16 },
    Scw { rs1: u8, rs2: u8, imm: i16 },
    Scd { rs1: u8, rs2: u8, imm: i16 },

    Amoswapw { rd: u8, rs1: u8, rs2: u8 },
    Amoswapd { rd: u8, rs1: u8, rs2: u8 },
    Amoaddw { rd: u8, rs1: u8, rs2: u8 },
    Amoaddd { rd: u8, rs1: u8, rs2: u8 },
    Amoandw { rd: u8, rs1: u8, rs2: u8 },
    Amoandd { rd: u8, rs1: u8, rs2: u8 },
    Amoorw { rd: u8, rs1: u8, rs2: u8 },
    Amoord { rd: u8, rs1: u8, rs2: u8 },
    Amoxorw { rd: u8, rs1: u8, rs2: u8 },
    Amoxord { rd: u8, rs1: u8, rs2: u8 },
    Amomaxw { rd: u8, rs1: u8, rs2: u8 },
    Amomaxd { rd: u8, rs1: u8, rs2: u8 },
    Amomaxuw { rd: u8, rs1: u8, rs2: u8 },
    Amomaxud { rd: u8, rs1: u8, rs2: u8 },
    Amominw { rd: u8, rs1: u8, rs2: u8 },
    Amomind { rd: u8, rs1: u8, rs2: u8 },
    Amominuw { rd: u8, rs1: u8, rs2: u8 },
    Amominud { rd: u8, rs1: u8, rs2: u8 },

    // RV32F
    Flw { rd: u8, rs1: u8, imm: u16 },
    Fsw { rs1: u8, rs2: u8, imm: i16 },
    FmaddS { rs1: u8, rs2: u8, rs3: u8, rd: u8 },
    FmsubS { rs1: u8, rs2: u8, rs3: u8, rd: u8 },
    FnmsubS { rs1: u8, rs2: u8, rs3: u8, rd: u8 },
    FnmaddS { rs1: u8, rs2: u8, rs3: u8, rd: u8 },
    FaddS { rd: u8, rs1: u8, rs2: u8 },
    FsubS { rd: u8, rs1: u8, rs2: u8 },
    FmulS { rd: u8, rs1: u8, rs2: u8 },
    FdivS { rd: u8, rs1: u8, rs2: u8 },
    FsqrtS { rd: u8, rs1: u8 },
    FsgnjS { rd: u8, rs1: u8, rs2: u8 },
    FsgnjnS { rd: u8, rs1: u8, rs2: u8 },
    FsgnjxS { rd: u8, rs1: u8, rs2: u8 },
    FminS { rd: u8, rs1: u8, rs2: u8 },
    FmaxS { rd: u8, rs1: u8, rs2: u8 },
    FcvtWS { rd: u8, rs1: u8 },
    FcvtWuS { rd: u8, rs1: u8 },
    FmvXW { rd: u8, rs1: u8 },
    FeqS { rd: u8, rs1: u8, rs2: u8 },
    FltS { rd: u8, rs1: u8, rs2: u8 },
    FleS { rd: u8, rs1: u8, rs2: u8 },
    FclassS { rd: u8, rs1: u8 },
    FcvtSW { rd: u8, rs1: u8 },
    FcvtSWu { rd: u8, rs1: u8 },
    FmvWX { rd: u8, rs1: u8 },

    // RV64F
    FcvtLS { rd: u8, rs1: u8 },
    FcvtLuS { rd: u8, rs1: u8 },
    FcvtSL { rd: u8, rs1: u8 },
    FcvtSLu { rd: u8, rs1: u8 },

    // RV32D
    Fld { rd: u8, rs1: u8, imm: u16 },
    Fsd { rs1: u8, rs2: u8, imm: i16 },
    FmaddD { rs1: u8, rs2: u8, rs3: u8, rd: u8 },
    FmsubD { rs1: u8, rs2: u8, rs3: u8, rd: u8 },
    FnmsubD { rs1: u8, rs2: u8, rs3: u8, rd: u8 },
    FnmaddD { rs1: u8, rs2: u8, rs3: u8, rd: u8 },
    FaddD { rd: u8, rs1: u8, rs2: u8 },
    FsubD { rd: u8, rs1: u8, rs2: u8 },
    FmulD { rd: u8, rs1: u8, rs2: u8 },
    FdivD { rd: u8, rs1: u8, rs2: u8 },
    FsqrtD { rd: u8, rs1: u8 },
    FsgnjD { rd: u8, rs1: u8, rs2: u8 },
    FsgnjnD { rd: u8, rs1: u8, rs2: u8 },
    FsgnjxD { rd: u8, rs1: u8, rs2: u8 },
    FminD { rd: u8, rs1: u8, rs2: u8 },
    FmaxD { rd: u8, rs1: u8, rs2: u8 },
    FcvtSD { rd: u8, rs1: u8 },
    FcvtDS { rd: u8, rs1: u8 },
    FeqD { rd: u8, rs1: u8, rs2: u8 },
    FltD { rd: u8, rs1: u8, rs2: u8 },
    FleD { rd: u8, rs1: u8, rs2: u8 },
    FclassD { rd: u8, rs1: u8 },
    FcvtWD { rd: u8, rs1: u8 },
    FcvtWuD { rd: u8, rs1: u8 },
    FcvtDW { rd: u8, rs1: u8 },
    FcvtDWu { rd: u8, rs1: u8 },

    // RV64D
    FcvtLD { rd: u8, rs1: u8 },
    FcvtLuD { rd: u8, rs1: u8 },
    FmvXD { rd: u8, rs1: u8 },
    FcvtDL { rd: u8, rs1: u8 },
    FcvtDLu { rd: u8, rs1: u8 },
    FmvDX { rd: u8, rs1: u8 },
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Extension {
    I,
    M,
    A,
    F,
    D,
}

impl Instruction {
    pub fn extension(&self) -> Option<Extension> {
        match self {
            Self::Add { .. }
            | Self::Sub { .. }
            | Self::Sll { .. }
            | Self::Slt { .. }
            | Self::Sltu { .. }
            | Self::Xor { .. }
            | Self::Srl { .. }
            | Self::Sra { .. }
            | Self::Or { .. }
            | Self::And { .. }
            | Self::Sb { .. }
            | Self::Sw { .. }
            | Self::Sh { .. }
            | Self::Lb { .. }
            | Self::Lh { .. }
            | Self::Lw { .. }
            | Self::Lbu { .. }
            | Self::Lhu { .. }
            | Self::Beq { .. }
            | Self::Bne { .. }
            | Self::Blt { .. }
            | Self::Bge { .. }
            | Self::Bltu { .. }
            | Self::Bgeu { .. }
            | Self::Jal { .. }
            | Self::Jalr { .. }
            | Self::Lui { .. }
            | Self::Auipc { .. }
            | Self::Addi { .. }
            | Self::Slti { .. }
            | Self::Sltiu { .. }
            | Self::Andi { .. }
            | Self::Ori { .. }
            | Self::Xori { .. }
            | Self::Slli { .. }
            | Self::Srli { .. }
            | Self::Srai { .. }
            | Self::Ebreak
            | Self::Ecall
            | Self::Fence { .. }
            | Self::Ld { .. }
            | Self::Sd { .. }
            | Self::Lwu { .. }
            | Self::Addw { .. }
            | Self::Addiw { .. }
            | Self::Subw { .. }
            | Self::Sllw { .. }
            | Self::Slliw { .. }
            | Self::Srlw { .. }
            | Self::Srliw { .. }
            | Self::Sraw { .. }
            | Self::Sraiw { .. } => Some(Extension::I),
            Self::Mul { .. }
            | Self::Mulw { .. }
            | Self::Mulh { .. }
            | Self::Mulhu { .. }
            | Self::Mulhsu { .. }
            | Self::Div { .. }
            | Self::Divu { .. }
            | Self::Rem { .. }
            | Self::Remu { .. }
            | Self::Divw { .. }
            | Self::Divuw { .. }
            | Self::Remw { .. }
            | Self::Remuw { .. } => Some(Extension::M),
            Self::Lrw { .. }
            | Self::Lrd { .. }
            | Self::Scw { .. }
            | Self::Scd { .. }
            | Self::Amoswapw { .. }
            | Self::Amoswapd { .. }
            | Self::Amoaddw { .. }
            | Self::Amoaddd { .. }
            | Self::Amoandw { .. }
            | Self::Amoandd { .. }
            | Self::Amoorw { .. }
            | Self::Amoord { .. }
            | Self::Amoxorw { .. }
            | Self::Amoxord { .. }
            | Self::Amomaxw { .. }
            | Self::Amomaxd { .. }
            | Self::Amomaxuw { .. }
            | Self::Amomaxud { .. }
            | Self::Amominw { .. }
            | Self::Amomind { .. }
            | Self::Amominuw { .. }
            | Self::Amominud { .. } => Some(Extension::A),
            Self::Flw { .. }
            | Self::Fsw { .. }
            | Self::FmaddS { .. }
            | Self::FmsubS { .. }
            | Self::FnmsubS { .. }
            | Self::FnmaddS { .. }
            | Self::FaddS { .. }
            | Self::FsubS { .. }
            | Self::FmulS { .. }
            | Self::FdivS { .. }
            | Self::FsqrtS { .. }
            | Self::FsgnjS { .. }
            | Self::FsgnjnS { .. }
            | Self::FsgnjxS { .. }
            | Self::FminS { .. }
            | Self::FmaxS { .. }
            | Self::FcvtWS { .. }
            | Self::FcvtWuS { .. }
            | Self::FmvXW { .. }
            | Self::FeqS { .. }
            | Self::FltS { .. }
            | Self::FleS { .. }
            | Self::FclassS { .. }
            | Self::FcvtSW { .. }
            | Self::FcvtSWu { .. }
            | Self::FmvWX { .. }
            | Self::FcvtLS { .. }
            | Self::FcvtLuS { .. }
            | Self::FcvtSL { .. }
            | Self::FcvtSLu { .. } => Some(Extension::F),
            Self::Fld { .. }
            | Self::Fsd { .. }
            | Self::FmaddD { .. }
            | Self::FmsubD { .. }
            | Self::FnmsubD { .. }
            | Self::FnmaddD { .. }
            | Self::FaddD { .. }
            | Self::FsubD { .. }
            | Self::FmulD { .. }
            | Self::FdivD { .. }
            | Self::FsqrtD { .. }
            | Self::FsgnjD { .. }
            | Self::FsgnjnD { .. }
            | Self::FsgnjxD { .. }
            | Self::FminD { .. }
            | Self::FmaxD { .. }
            | Self::FcvtSD { .. }
            | Self::FcvtDS { .. }
            | Self::FeqD { .. }
            | Self::FltD { .. }
            | Self::FleD { .. }
            | Self::FclassD { .. }
            | Self::FcvtWD { .. }
            | Self::FcvtWuD { .. }
            | Self::FcvtDW { .. }
            | Self::FcvtDWu { .. }
            | Self::FcvtLD { .. }
            | Self::FcvtLuD { .. }
            | Self::FmvXD { .. }
            | Self::FcvtDL { .. }
            | Self::FcvtDLu { .. }
            | Self::FmvDX { .. } => Some(Extension::D),
        }
    }
}

macro_rules! impl_display {
    ($enum_name:ident { $($variant:ident),* $(,)? }) => {
        impl ::std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(
                        $enum_name::$variant { .. } => f.write_str(&stringify!($variant).to_lowercase()),
                    )*
                }
            }
        }
    };
}

impl_display!(Instruction {
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Xor,
    Srl,
    Sra,
    Or,
    And,
    Addi,
    Slli,
    Slti,
    Sltiu,
    Andi,
    Ori,
    Xori,
    Srli,
    Srai,
    Sb,
    Sw,
    Sh,
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
    Jal,
    Jalr,
    Lui,
    Auipc,
    Ecall,
    Ebreak,
    Fence,
    Mul,
    Mulw,
    Mulh,
    Mulhu,
    Mulhsu,
    Div,
    Divu,
    Rem,
    Remu,
    Divw,
    Divuw,
    Remw,
    Remuw,
    Ld,
    Sd,
    Lwu,
    Addw,
    Addiw,
    Subw,
    Sllw,
    Slliw,
    Srlw,
    Srliw,
    Sraw,
    Sraiw,
    Lrw,
    Lrd,
    Scw,
    Scd,
    Amoswapw,
    Amoswapd,
    Amoaddw,
    Amoaddd,
    Amoandw,
    Amoandd,
    Amoorw,
    Amoord,
    Amoxorw,
    Amoxord,
    Amomaxw,
    Amomaxd,
    Amomaxuw,
    Amomaxud,
    Amominw,
    Amomind,
    Amominuw,
    Amominud,
    Flw,
    Fsw,
    FmaddS,
    FmsubS,
    FnmsubS,
    FnmaddS,
    FaddS,
    FsubS,
    FmulS,
    FdivS,
    FsqrtS,
    FsgnjS,
    FsgnjnS,
    FsgnjxS,
    FminS,
    FmaxS,
    FcvtWS,
    FcvtWuS,
    FmvXW,
    FeqS,
    FltS,
    FleS,
    FclassS,
    FcvtSW,
    FcvtSWu,
    FmvWX,
    FcvtLS,
    FcvtLuS,
    FcvtSL,
    FcvtSLu,
    Fld,
    Fsd,
    FmaddD,
    FmsubD,
    FnmsubD,
    FnmaddD,
    FaddD,
    FsubD,
    FmulD,
    FdivD,
    FsqrtD,
    FsgnjD,
    FsgnjnD,
    FsgnjxD,
    FminD,
    FmaxD,
    FcvtSD,
    FcvtDS,
    FeqD,
    FltD,
    FleD,
    FclassD,
    FcvtWD,
    FcvtWuD,
    FcvtDW,
    FcvtDWu,
    FcvtLD,
    FcvtLuD,
    FmvXD,
    FcvtDL,
    FcvtDLu,
    FmvDX
});

impl_display!(Extension { I, M, A, F, D });

#[derive(Debug)]
pub enum DecodeError {
    InvalidOpcode(u8),
    InvalidFunct3(u8),
    InvalidFunct7(u8),
    InvalidFunct12(u16),
    InvalidSomething(u32),
}

impl TryFrom<u32> for Instruction {
    type Error = DecodeError;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        let instr = RawInstruction(raw);

        if (raw & opcodes::MASK_ADD) == opcodes::MATCH_ADD {
            return Ok(Instruction::Add {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_SUB) == opcodes::MATCH_SUB {
            return Ok(Instruction::Sub {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_SLL) == opcodes::MATCH_SLL {
            return Ok(Instruction::Sll {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_SLT) == opcodes::MATCH_SLT {
            return Ok(Instruction::Slt {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_SLTU) == opcodes::MATCH_SLTU {
            return Ok(Instruction::Sltu {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_XOR) == opcodes::MATCH_XOR {
            return Ok(Instruction::Xor {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_SRL) == opcodes::MATCH_SRL {
            return Ok(Instruction::Srl {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_SRA) == opcodes::MATCH_SRA {
            return Ok(Instruction::Sra {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_OR) == opcodes::MATCH_OR {
            return Ok(Instruction::Or {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AND) == opcodes::MATCH_AND {
            return Ok(Instruction::And {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_ADDI) == opcodes::MATCH_ADDI {
            return Ok(Instruction::Addi {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_addi(),
            });
        } else if (raw & opcodes::MASK_SLLI) == opcodes::MATCH_SLLI {
            return Ok(Instruction::Slli {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_addi(),
            });
        } else if (raw & opcodes::MASK_SLTI) == opcodes::MATCH_SLTI {
            return Ok(Instruction::Slti {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_addi(),
            });
        } else if (raw & opcodes::MASK_SLTIU) == opcodes::MATCH_SLTIU {
            return Ok(Instruction::Sltiu {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_addi(),
            });
        } else if (raw & opcodes::MASK_ORI) == opcodes::MATCH_ORI {
            return Ok(Instruction::Ori {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_addi(),
            });
        } else if (raw & opcodes::MASK_XORI) == opcodes::MATCH_XORI {
            return Ok(Instruction::Xori {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_addi(),
            });
        } else if (raw & opcodes::MASK_SRLI) == opcodes::MATCH_SRLI {
            return Ok(Instruction::Srli {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_addi(),
            });
        } else if (raw & opcodes::MASK_SRAI) == opcodes::MATCH_SRAI {
            return Ok(Instruction::Srai {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_addi(),
            });
        } else if (raw & opcodes::MASK_SB) == opcodes::MATCH_SB {
            return Ok(Instruction::Sb {
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                imm: instr.imm_store(),
            });
        } else if (raw & opcodes::MASK_SW) == opcodes::MATCH_SW {
            return Ok(Instruction::Sw {
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                imm: instr.imm_store(),
            });
        } else if (raw & opcodes::MASK_SH) == opcodes::MATCH_SH {
            return Ok(Instruction::Sh {
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                imm: instr.imm_store(),
            });
        } else if (raw & opcodes::MASK_LB) == opcodes::MATCH_LB {
            return Ok(Instruction::Lb {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_load(),
            });
        } else if (raw & opcodes::MASK_LW) == opcodes::MATCH_LW {
            return Ok(Instruction::Lw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_load(),
            });
        } else if (raw & opcodes::MASK_LH) == opcodes::MATCH_LH {
            return Ok(Instruction::Lh {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_load(),
            });
        } else if (raw & opcodes::MASK_LBU) == opcodes::MATCH_LBU {
            return Ok(Instruction::Lbu {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_load(),
            });
        } else if (raw & opcodes::MASK_LHU) == opcodes::MATCH_LHU {
            return Ok(Instruction::Lhu {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_load(),
            });
        } else if (raw & opcodes::MASK_BEQ) == opcodes::MATCH_BEQ {
            return Ok(Instruction::Beq {
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                imm: instr.imm_branch(),
            });
        } else if (raw & opcodes::MASK_BNE) == opcodes::MATCH_BNE {
            return Ok(Instruction::Bne {
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                imm: instr.imm_branch(),
            });
        } else if (raw & opcodes::MASK_BLT) == opcodes::MATCH_BLT {
            return Ok(Instruction::Blt {
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                imm: instr.imm_branch(),
            });
        } else if (raw & opcodes::MASK_BGE) == opcodes::MATCH_BGE {
            return Ok(Instruction::Bge {
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                imm: instr.imm_branch(),
            });
        } else if (raw & opcodes::MASK_BLTU) == opcodes::MATCH_BLTU {
            return Ok(Instruction::Bltu {
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                imm: instr.imm_branch(),
            });
        } else if (raw & opcodes::MASK_BGEU) == opcodes::MATCH_BGEU {
            return Ok(Instruction::Bgeu {
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                imm: instr.imm_branch(),
            });
        } else if (raw & opcodes::MASK_JAL) == opcodes::MATCH_JAL {
            return Ok(Instruction::Jal {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_jal(),
            });
        } else if (raw & opcodes::MASK_JALR) == opcodes::MATCH_JALR {
            return Ok(Instruction::Jalr {
                rd: instr.rd(),
                imm: instr.imm_jalr(),
            });
        } else if (raw & opcodes::MASK_LUI) == opcodes::MATCH_LUI {
            return Ok(Instruction::Lui {
                rd: instr.rd(),
                imm: instr.imm_lui_auipc(),
            });
        } else if (raw & opcodes::MASK_AUIPC) == opcodes::MATCH_AUIPC {
            return Ok(Instruction::Auipc {
                rd: instr.rd(),
                imm: instr.imm_lui_auipc(),
            });
        } else if (raw & opcodes::MASK_ECALL) == opcodes::MATCH_ECALL {
            return Ok(Instruction::Ecall);
        } else if (raw & opcodes::MASK_EBREAK) == opcodes::MATCH_EBREAK {
            return Ok(Instruction::Ebreak);
        } else if (raw & opcodes::MASK_FENCE) == opcodes::MATCH_FENCE {
            return Ok(Instruction::Fence {
                pred: instr.fence_predecessor_successor().0,
                succ: instr.fence_predecessor_successor().1,
            });
        } else if (raw & opcodes::MASK_LD) == opcodes::MATCH_LD {
            return Ok(Instruction::Ld {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_load(),
            });
        } else if (raw & opcodes::MASK_SD) == opcodes::MATCH_SD {
            return Ok(Instruction::Sd {
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                imm: instr.imm_store(),
            });
        } else if (raw & opcodes::MASK_LWU) == opcodes::MATCH_LWU {
            return Ok(Instruction::Lwu {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_load(),
            });
        } else if (raw & opcodes::MASK_ADDW) == opcodes::MATCH_ADDW {
            return Ok(Instruction::Addw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_ADDIW) == opcodes::MATCH_ADDIW {
            return Ok(Instruction::Addiw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_addi(),
            });
        } else if (raw & opcodes::MASK_SUBW) == opcodes::MATCH_SUBW {
            return Ok(Instruction::Subw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_SLLW) == opcodes::MATCH_SLLW {
            return Ok(Instruction::Sllw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_SLLIW) == opcodes::MATCH_SLLIW {
            return Ok(Instruction::Slliw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_addi(),
            });
        } else if (raw & opcodes::MASK_SRLW) == opcodes::MATCH_SRLW {
            return Ok(Instruction::Srlw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_SRLIW) == opcodes::MATCH_SRLIW {
            return Ok(Instruction::Srliw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_addi(),
            });
        } else if (raw & opcodes::MASK_SRAW) == opcodes::MATCH_SRAW {
            return Ok(Instruction::Sraw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_SRAIW) == opcodes::MATCH_SRAIW {
            return Ok(Instruction::Sraiw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_addi(),
            });
        } else if (raw & opcodes::MASK_MUL) == opcodes::MATCH_MUL {
            return Ok(Instruction::Mul {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_MULH) == opcodes::MATCH_MULH {
            return Ok(Instruction::Mulh {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_MULHU) == opcodes::MATCH_MULHU {
            return Ok(Instruction::Mulhu {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_MULHSU) == opcodes::MATCH_MULHSU {
            return Ok(Instruction::Mulhsu {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_MULW) == opcodes::MATCH_MULW {
            return Ok(Instruction::Mulw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_DIV) == opcodes::MATCH_DIV {
            return Ok(Instruction::Div {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_DIVU) == opcodes::MATCH_DIVU {
            return Ok(Instruction::Divu {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_REM) == opcodes::MATCH_REM {
            return Ok(Instruction::Rem {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_REMU) == opcodes::MATCH_REMU {
            return Ok(Instruction::Remu {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_DIVW) == opcodes::MATCH_DIVW {
            return Ok(Instruction::Divw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_DIVUW) == opcodes::MATCH_DIVUW {
            return Ok(Instruction::Divuw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_REMW) == opcodes::MATCH_REMW {
            return Ok(Instruction::Remw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_REMUW) == opcodes::MATCH_REMUW {
            return Ok(Instruction::Remuw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_LR_W) == opcodes::MATCH_LR_W {
            return Ok(Instruction::Lrw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_load(),
            });
        } else if (raw & opcodes::MASK_LR_D) == opcodes::MATCH_LR_D {
            return Ok(Instruction::Lrd {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_load(),
            });
        } else if (raw & opcodes::MASK_SC_W) == opcodes::MATCH_SC_W {
            return Ok(Instruction::Scw {
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                imm: instr.imm_store(),
            });
        } else if (raw & opcodes::MASK_SC_D) == opcodes::MATCH_SC_D {
            return Ok(Instruction::Scd {
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                imm: instr.imm_store(),
            });
        } else if (raw & opcodes::MASK_AMOSWAP_W) == opcodes::MATCH_AMOSWAP_W {
            return Ok(Instruction::Amoswapw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOSWAP_D) == opcodes::MATCH_AMOSWAP_D {
            return Ok(Instruction::Amoswapd {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOADD_W) == opcodes::MATCH_AMOADD_W {
            return Ok(Instruction::Amoaddw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOADD_D) == opcodes::MATCH_AMOADD_D {
            return Ok(Instruction::Amoaddd {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOAND_W) == opcodes::MATCH_AMOAND_W {
            return Ok(Instruction::Amoandw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOAND_D) == opcodes::MATCH_AMOAND_D {
            return Ok(Instruction::Amoandd {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOOR_W) == opcodes::MATCH_AMOOR_W {
            return Ok(Instruction::Amoorw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOOR_D) == opcodes::MATCH_AMOOR_D {
            return Ok(Instruction::Amoord {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOXOR_W) == opcodes::MATCH_AMOXOR_W {
            return Ok(Instruction::Amoxorw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOXOR_D) == opcodes::MATCH_AMOXOR_D {
            return Ok(Instruction::Amoxord {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOMAX_W) == opcodes::MATCH_AMOMAX_W {
            return Ok(Instruction::Amomaxw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOMAX_D) == opcodes::MATCH_AMOMAX_D {
            return Ok(Instruction::Amomaxd {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOMAXU_W) == opcodes::MATCH_AMOMAXU_W {
            return Ok(Instruction::Amomaxuw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOMAXU_D) == opcodes::MATCH_AMOMAXU_D {
            return Ok(Instruction::Amomaxud {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOMIN_W) == opcodes::MATCH_AMOMIN_W {
            return Ok(Instruction::Amominw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOMIN_D) == opcodes::MATCH_AMOMIN_D {
            return Ok(Instruction::Amomind {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOMINU_W) == opcodes::MATCH_AMOMINU_W {
            return Ok(Instruction::Amominuw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_AMOMINU_D) == opcodes::MATCH_AMOMINU_D {
            return Ok(Instruction::Amominud {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FLW) == opcodes::MATCH_FLW {
            return Ok(Instruction::Flw {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_load(),
            });
        } else if (raw & opcodes::MASK_FSW) == opcodes::MATCH_FSW {
            return Ok(Instruction::Fsw {
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                imm: instr.imm_store(),
            });
        } else if (raw & opcodes::MASK_FMADD_S) == opcodes::MATCH_FMADD_S {
            return Ok(Instruction::FmaddS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                rs3: instr.rs3(),
            });
        } else if (raw & opcodes::MASK_FMSUB_S) == opcodes::MATCH_FMSUB_S {
            return Ok(Instruction::FmsubS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                rs3: instr.rs3(),
            });
        } else if (raw & opcodes::MASK_FNMSUB_S) == opcodes::MATCH_FNMSUB_S {
            return Ok(Instruction::FnmsubS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                rs3: instr.rs3(),
            });
        } else if (raw & opcodes::MASK_FNMADD_S) == opcodes::MATCH_FNMADD_S {
            return Ok(Instruction::FnmaddS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                rs3: instr.rs3(),
            });
        } else if (raw & opcodes::MASK_FADD_S) == opcodes::MATCH_FADD_S {
            return Ok(Instruction::FaddS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FSUB_S) == opcodes::MATCH_FSUB_S {
            return Ok(Instruction::FsubS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FMUL_S) == opcodes::MATCH_FMUL_S {
            return Ok(Instruction::FmulS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FDIV_S) == opcodes::MATCH_FDIV_S {
            return Ok(Instruction::FdivS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FSQRT_S) == opcodes::MATCH_FSQRT_S {
            return Ok(Instruction::FsqrtS {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FSGNJ_S) == opcodes::MATCH_FSGNJ_S {
            return Ok(Instruction::FsgnjS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FSGNJN_S) == opcodes::MATCH_FSGNJN_S {
            return Ok(Instruction::FsgnjnS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FSGNJX_S) == opcodes::MATCH_FSGNJX_S {
            return Ok(Instruction::FsgnjxS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FMIN_S) == opcodes::MATCH_FMIN_S {
            return Ok(Instruction::FminS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FMAX_S) == opcodes::MATCH_FMAX_S {
            return Ok(Instruction::FmaxS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FCVT_W_S) == opcodes::MATCH_FCVT_W_S {
            return Ok(Instruction::FcvtWS {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_WU_S) == opcodes::MATCH_FCVT_WU_S {
            return Ok(Instruction::FcvtWuS {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FMV_X_W) == opcodes::MATCH_FMV_X_W {
            return Ok(Instruction::FmvXW {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FEQ_S) == opcodes::MATCH_FEQ_S {
            return Ok(Instruction::FeqS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FLT_S) == opcodes::MATCH_FLT_S {
            return Ok(Instruction::FltS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FLE_S) == opcodes::MATCH_FLE_S {
            return Ok(Instruction::FleS {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FCLASS_S) == opcodes::MATCH_FCLASS_S {
            return Ok(Instruction::FclassS {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_S_W) == opcodes::MATCH_FCVT_S_W {
            return Ok(Instruction::FcvtSW {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_S_WU) == opcodes::MATCH_FCVT_S_WU {
            return Ok(Instruction::FcvtSWu {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FMV_W_X) == opcodes::MATCH_FMV_W_X {
            return Ok(Instruction::FmvWX {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_L_S) == opcodes::MATCH_FCVT_L_S {
            return Ok(Instruction::FcvtLS {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_LU_S) == opcodes::MATCH_FCVT_LU_S {
            return Ok(Instruction::FcvtLuS {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_S_L) == opcodes::MATCH_FCVT_S_L {
            return Ok(Instruction::FcvtSL {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_S_LU) == opcodes::MATCH_FCVT_S_LU {
            return Ok(Instruction::FcvtSLu {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FLD) == opcodes::MATCH_FLD {
            return Ok(Instruction::Fld {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_load(),
            });
        } else if (raw & opcodes::MASK_FSD) == opcodes::MATCH_FSD {
            return Ok(Instruction::Fsd {
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                imm: instr.imm_store(),
            });
        } else if (raw & opcodes::MASK_FMADD_D) == opcodes::MATCH_FMADD_D {
            return Ok(Instruction::FmaddD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                rs3: instr.rs3(),
            });
        } else if (raw & opcodes::MASK_FMSUB_D) == opcodes::MATCH_FMSUB_D {
            return Ok(Instruction::FmsubD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                rs3: instr.rs3(),
            });
        } else if (raw & opcodes::MASK_FNMSUB_D) == opcodes::MATCH_FNMSUB_D {
            return Ok(Instruction::FnmsubD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                rs3: instr.rs3(),
            });
        } else if (raw & opcodes::MASK_FNMADD_D) == opcodes::MATCH_FNMADD_D {
            return Ok(Instruction::FnmaddD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
                rs3: instr.rs3(),
            });
        } else if (raw & opcodes::MASK_FADD_D) == opcodes::MATCH_FADD_D {
            return Ok(Instruction::FaddD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FSUB_D) == opcodes::MATCH_FSUB_D {
            return Ok(Instruction::FsubD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FMUL_D) == opcodes::MATCH_FMUL_D {
            return Ok(Instruction::FmulD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FDIV_D) == opcodes::MATCH_FDIV_D {
            return Ok(Instruction::FdivD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FSQRT_D) == opcodes::MATCH_FSQRT_D {
            return Ok(Instruction::FsqrtD {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FSGNJ_D) == opcodes::MATCH_FSGNJ_D {
            return Ok(Instruction::FsgnjD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FSGNJN_D) == opcodes::MATCH_FSGNJN_D {
            return Ok(Instruction::FsgnjnD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FSGNJX_D) == opcodes::MATCH_FSGNJX_D {
            return Ok(Instruction::FsgnjxD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FMIN_D) == opcodes::MATCH_FMIN_D {
            return Ok(Instruction::FminD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FMAX_D) == opcodes::MATCH_FMAX_D {
            return Ok(Instruction::FmaxD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FCVT_S_D) == opcodes::MATCH_FCVT_S_D {
            return Ok(Instruction::FcvtSD {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_D_S) == opcodes::MATCH_FCVT_D_S {
            return Ok(Instruction::FcvtDS {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FEQ_D) == opcodes::MATCH_FEQ_D {
            return Ok(Instruction::FeqD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FLT_D) == opcodes::MATCH_FLT_D {
            return Ok(Instruction::FltD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FLE_D) == opcodes::MATCH_FLE_D {
            return Ok(Instruction::FleD {
                rd: instr.rd(),
                rs1: instr.rs1(),
                rs2: instr.rs2(),
            });
        } else if (raw & opcodes::MASK_FCLASS_D) == opcodes::MATCH_FCLASS_D {
            return Ok(Instruction::FclassD {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_W_D) == opcodes::MATCH_FCVT_W_D {
            return Ok(Instruction::FcvtWD {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_WU_D) == opcodes::MATCH_FCVT_WU_D {
            return Ok(Instruction::FcvtWuD {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_D_W) == opcodes::MATCH_FCVT_D_W {
            return Ok(Instruction::FcvtDW {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_D_WU) == opcodes::MATCH_FCVT_D_WU {
            return Ok(Instruction::FcvtDWu {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_L_D) == opcodes::MATCH_FCVT_L_D {
            return Ok(Instruction::FcvtLD {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_LU_D) == opcodes::MATCH_FCVT_LU_D {
            return Ok(Instruction::FcvtLuD {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FMV_X_D) == opcodes::MATCH_FMV_X_D {
            return Ok(Instruction::FmvXD {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_D_L) == opcodes::MATCH_FCVT_D_L {
            return Ok(Instruction::FcvtDL {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FCVT_D_LU) == opcodes::MATCH_FCVT_D_LU {
            return Ok(Instruction::FcvtDLu {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        } else if (raw & opcodes::MASK_FMV_D_X) == opcodes::MATCH_FMV_D_X {
            return Ok(Instruction::FmvDX {
                rd: instr.rd(),
                rs1: instr.rs1(),
            });
        }

        Err(DecodeError::InvalidSomething(raw))
    }
}

pub fn get_vec_instructions(raw: Vec<u32>) -> Vec<Instruction> {
    raw.into_iter()
        .filter_map(|elem| elem.try_into().ok())
        .collect()
}

pub fn get_extensions_from_instructions(raw: Vec<Instruction>) -> HashSet<Extension> {
    let mut out = HashSet::new();

    for instr in raw {
        out.insert(instr.extension().expect("Extensions invalid."));
    }

    out
}
