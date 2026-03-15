#[cfg(test)]
mod tests;

#[derive(Debug)]
struct RawInstruction(u32);

impl RawInstruction {
    fn opcode(&self) -> u8 {
        (self.0 & 0x7F) as u8
    }
    fn rd(&self) -> u8 {
        ((self.0 >> 7) & 0x1F) as u8
    }
    fn funct3(&self) -> u8 {
        ((self.0 >> 12) & 0x7) as u8
    }
    fn rs1(&self) -> u8 {
        ((self.0 >> 15) & 0x1F) as u8
    }
    fn rs2(&self) -> u8 {
        ((self.0 >> 20) & 0x1F) as u8
    }
    fn funct7(&self) -> u8 {
        ((self.0 >> 25) & 0x7F) as u8
    }
    fn funct12(&self) -> u16 {
        (self.0 >> 20) as u16
    }
    fn imm_store(&self) -> u16 {
        ((((self.0 >> 7) & 0x1F | ((self.0 >> 25) & 0x7F) << 5) << 20) as i32 >> 20)
            .try_into()
            .unwrap()
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

    Sb { rs1: u8, rs2: u8, imm: u16 },
    Sh { rs1: u8, rs2: u8, imm: u16 },
    Sw { rs1: u8, rs2: u8, imm: u16 },

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
}

#[derive(Debug)]
pub enum Extension {
    I,
    M,
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
            | Self::Fence { .. } => Some(Extension::I),
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
        }
    }
}

macro_rules! impl_pretty_print {
    ($enum_name:ident { $($variant:ident),* $(,)? }) => {
        impl $enum_name {
            pub fn pretty_print(&self) -> String {
                match self {
                    $(
                        $enum_name::$variant { .. } => stringify!($variant).to_lowercase(),
                    )*
                }
            }
        }
    };
}

impl_pretty_print!(Instruction {
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
});

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

        match instr.opcode() {
            OP_ALU => match (instr.funct3(), instr.funct7()) {
                (FUNCT3_ADD, FUNCT7_ADD) => Ok(Instruction::Add {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_SUB, FUNCT7_SUB) => Ok(Instruction::Sub {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_SLL, FUNCT7_SLL) => Ok(Instruction::Sll {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_SLT, FUNCT7_SLT) => Ok(Instruction::Slt {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_SLTU, FUNCT7_SLTU) => Ok(Instruction::Sltu {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_XOR, FUNCT7_XOR) => Ok(Instruction::Xor {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_SRL, FUNCT7_SRL) => Ok(Instruction::Srl {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_SRA, FUNCT7_SRA) => Ok(Instruction::Sra {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_OR, FUNCT7_OR) => Ok(Instruction::Or {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_AND, FUNCT7_AND) => Ok(Instruction::And {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_MUL, FUNCT7_MULDIV) => Ok(Instruction::Mul {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_MULH, FUNCT7_MULDIV) => Ok(Instruction::Mulh {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_MULHU, FUNCT7_MULDIV) => Ok(Instruction::Mulhu {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_MULHSU, FUNCT7_MULDIV) => Ok(Instruction::Mulhsu {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_DIV, FUNCT7_MULDIV) => Ok(Instruction::Div {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_DIVU, FUNCT7_MULDIV) => Ok(Instruction::Divu {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_REM, FUNCT7_MULDIV) => Ok(Instruction::Rem {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_REMU, FUNCT7_MULDIV) => Ok(Instruction::Remu {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                _ => Err(DecodeError::InvalidFunct3(instr.funct3())),
            },
            OP_OP32 => match (instr.funct3(), instr.funct7()) {
                (FUNCT3_MUL, FUNCT7_MULDIV) => Ok(Instruction::Mulw {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_DIV, FUNCT7_MULDIV) => Ok(Instruction::Divw {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_DIVU, FUNCT7_MULDIV) => Ok(Instruction::Divuw {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_REM, FUNCT7_MULDIV) => Ok(Instruction::Remw {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                (FUNCT3_REMU, FUNCT7_MULDIV) => Ok(Instruction::Remuw {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                }),
                _ => Err(DecodeError::InvalidFunct3(instr.funct3())),
            },
            OP_IMM => match instr.funct3() {
                FUNCT3_ADDI => Ok(Instruction::Addi {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    imm: instr.imm_addi(),
                }),
                FUNCT3_SLTIU => Ok(Instruction::Sltiu {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    imm: instr.imm_addi(),
                }),
                FUNCT3_SLTI => Ok(Instruction::Slti {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    imm: instr.imm_addi(),
                }),
                FUNCT3_XORI => Ok(Instruction::Xori {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    imm: instr.imm_addi(),
                }),
                FUNCT3_ORI => Ok(Instruction::Ori {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    imm: instr.imm_addi(),
                }),
                FUNCT3_ANDI => Ok(Instruction::Andi {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    imm: instr.imm_addi(),
                }),
                FUNCT3_SRLI => {
                    if instr.imm_addi() >> 6 == 0 {
                        Ok(Instruction::Srli {
                            rd: instr.rd(),
                            rs1: instr.rs1(),
                            imm: instr.imm_addi(),
                        })
                    } else if instr.imm_addi() >> 6 == 0x10 {
                        Ok(Instruction::Srai {
                            rd: instr.rd(),
                            rs1: instr.rs1(),
                            imm: instr.imm_addi(),
                        })
                    } else {
                        Err(DecodeError::InvalidSomething(
                            (instr.imm_addi() >> 6).into(),
                        ))
                    }
                }
                FUNCT3_SLLI => {
                    if (instr.imm_addi() & 0xFE0) >> 6 == 0 {
                        Ok(Instruction::Slli {
                            rd: instr.rd(),
                            rs1: instr.rs1(),
                            imm: instr.imm_addi(),
                        })
                    } else {
                        Err(DecodeError::InvalidSomething(instr.0))
                    }
                }
                _ => Err(DecodeError::InvalidFunct3(instr.funct3())),
            },
            OP_STORE => match instr.funct3() {
                FUNCT3_SB => Ok(Instruction::Sb {
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                    imm: instr.imm_store(),
                }),
                FUNCT3_SW => Ok(Instruction::Sw {
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                    imm: instr.imm_store(),
                }),
                FUNCT3_SH => Ok(Instruction::Sh {
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                    imm: instr.imm_store(),
                }),
                _ => Err(DecodeError::InvalidFunct3(instr.funct3())),
            },
            OP_LOAD => match instr.funct3() {
                FUNCT3_LB => Ok(Instruction::Lb {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    imm: instr.imm_load(),
                }),
                FUNCT3_LH => Ok(Instruction::Lh {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    imm: instr.imm_load(),
                }),
                FUNCT3_LW => Ok(Instruction::Lw {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    imm: instr.imm_load(),
                }),
                FUNCT3_LBU => Ok(Instruction::Lbu {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    imm: instr.imm_load(),
                }),
                FUNCT3_LHU => Ok(Instruction::Lhu {
                    rd: instr.rd(),
                    rs1: instr.rs1(),
                    imm: instr.imm_load(),
                }),
                _ => Err(DecodeError::InvalidFunct3(instr.funct3())),
            },
            OP_BRANCH => match instr.funct3() {
                FUNCT3_BEQ => Ok(Instruction::Beq {
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                    imm: instr.imm_branch(),
                }),
                FUNCT3_BNE => Ok(Instruction::Bne {
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                    imm: instr.imm_branch(),
                }),
                FUNCT3_BLT => Ok(Instruction::Blt {
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                    imm: instr.imm_branch(),
                }),
                FUNCT3_BGE => Ok(Instruction::Bge {
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                    imm: instr.imm_branch(),
                }),
                FUNCT3_BLTU => Ok(Instruction::Bltu {
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                    imm: instr.imm_branch(),
                }),
                FUNCT3_BGEU => Ok(Instruction::Bgeu {
                    rs1: instr.rs1(),
                    rs2: instr.rs2(),
                    imm: instr.imm_branch(),
                }),
                _ => Err(DecodeError::InvalidFunct3(instr.funct3())),
            },
            OP_JALR => match instr.funct3() {
                FUNCT3_JALR => Ok(Instruction::Jalr {
                    rd: instr.rd(),
                    imm: instr.imm_jalr(),
                }),
                _ => Err(DecodeError::InvalidFunct3(instr.funct3())),
            },
            OP_JAL => Ok(Instruction::Jal {
                rd: instr.rd(),
                rs1: instr.rs1(),
                imm: instr.imm_jal(),
            }),
            OP_AUIPC => Ok(Instruction::Auipc {
                rd: instr.rd(),
                imm: instr.imm_lui_auipc(),
            }),
            OP_LUI => Ok(Instruction::Lui {
                rd: instr.rd(),
                imm: instr.imm_lui_auipc(),
            }),
            OP_SYSTEM => match instr.funct12() {
                FUNCT12_ECALL => Ok(Instruction::Ecall),
                FUNCT12_EBREAK => Ok(Instruction::Ebreak),
                _ => Err(DecodeError::InvalidFunct12(instr.funct12())),
            },
            OP_MISCMEM => match (instr.funct3(), instr.rs1(), instr.rd()) {
                (FUNCT3_ADD, 0, 0) => Ok(Instruction::Fence {
                    pred: instr.fence_predecessor_successor().0,
                    succ: instr.fence_predecessor_successor().1,
                }),
                _ => Err(DecodeError::InvalidSomething(instr.0)),
            },
            _ => Err(DecodeError::InvalidOpcode(instr.opcode())),
        }
    }
}

// CONSTANTS

// OP_ALU

pub const OP_ALU: u8 = 0b0110011;
pub const OP_OP32: u8 = 0b0111011;

pub const FUNCT3_ADD: u8 = 0b000;
pub const FUNCT7_ADD: u8 = 0b0000000;

pub const FUNCT3_SUB: u8 = 0b000;
pub const FUNCT7_SUB: u8 = 0b0100000;

pub const FUNCT3_SLL: u8 = 0b001;
pub const FUNCT7_SLL: u8 = 0b0000000;

pub const FUNCT3_SLT: u8 = 0b010;
pub const FUNCT7_SLT: u8 = 0b0000000;

pub const FUNCT3_SLTU: u8 = 0b011;
pub const FUNCT7_SLTU: u8 = 0b0000000;

pub const FUNCT3_XOR: u8 = 0b100;
pub const FUNCT7_XOR: u8 = 0b0000000;

pub const FUNCT3_SRL: u8 = 0b101;
pub const FUNCT7_SRL: u8 = 0b0000000;

pub const FUNCT3_SRA: u8 = 0b101;
pub const FUNCT7_SRA: u8 = 0b0100000;

pub const FUNCT3_OR: u8 = 0b110;
pub const FUNCT7_OR: u8 = 0b0000000;

pub const FUNCT3_AND: u8 = 0b111;
pub const FUNCT7_AND: u8 = 0b0000000;

// OP_STORE

pub const OP_STORE: u8 = 0b0100011;

pub const FUNCT3_SB: u8 = 0b000;
pub const FUNCT3_SH: u8 = 0b001;
pub const FUNCT3_SW: u8 = 0b010;

// OP_IMM

pub const OP_IMM: u8 = 0b0010011;

pub const FUNCT3_ADDI: u8 = 0b000;

pub const FUNCT3_SLLI: u8 = 0b001;
pub const FUNCT7_SLLI: u8 = 0b0000000;

pub const FUNCT3_SLTI: u8 = 0b010;

pub const FUNCT3_SLTIU: u8 = 0b011;

pub const FUNCT3_XORI: u8 = 0b100;

pub const FUNCT3_SRLI: u8 = 0b101;
pub const FUNCT7_SRLI: u8 = 0b0000000;

pub const FUNCT3_SRAI: u8 = 0b101;
pub const FUNCT7_SRAI: u8 = 0b0100000;

pub const FUNCT3_ORI: u8 = 0b110;

pub const FUNCT3_ANDI: u8 = 0b111;

// OP_LOAD

pub const OP_LOAD: u8 = 0b0000011;

pub const FUNCT3_LB: u8 = 0b000;
pub const FUNCT3_LH: u8 = 0b001;
pub const FUNCT3_LW: u8 = 0b010;

pub const FUNCT3_LBU: u8 = 0b100;
pub const FUNCT3_LHU: u8 = 0b101;

// OP_BRANCH

pub const OP_BRANCH: u8 = 0b1100011;

pub const FUNCT3_BEQ: u8 = 0b000;
pub const FUNCT3_BNE: u8 = 0b001;
pub const FUNCT3_BLT: u8 = 0b100;
pub const FUNCT3_BGE: u8 = 0b101;
pub const FUNCT3_BLTU: u8 = 0b110;
pub const FUNCT3_BGEU: u8 = 0b111;

// OP_JALR

pub const OP_JALR: u8 = 0b1100111;

pub const FUNCT3_JALR: u8 = 0b000;

// OP_JAL

pub const OP_JAL: u8 = 0b1101111;

// OP_AUIPC

pub const OP_AUIPC: u8 = 0b0010111;

// OP_LUI

pub const OP_LUI: u8 = 0b0110111;

// OP_FENCE

pub const OP_MISCMEM: u8 = 0b0001111;

// OP_SYSTEM

pub const OP_SYSTEM: u8 = 0b1110011;

pub const FUNCT12_ECALL: u16 = 0b000000000000;
pub const FUNCT12_EBREAK: u16 = 0b000000000001;

// MULDIV

pub const FUNCT7_MULDIV: u8 = 0b0000001;

pub const FUNCT3_MUL: u8 = 0b000;
pub const FUNCT3_MULH: u8 = 0b001;
pub const FUNCT3_MULHU: u8 = 0b011;
pub const FUNCT3_MULHSU: u8 = 0b010;

pub const FUNCT3_DIV: u8 = 0b100;
pub const FUNCT3_DIVU: u8 = 0b101;
pub const FUNCT3_REM: u8 = 0b110;
pub const FUNCT3_REMU: u8 = 0b111;
