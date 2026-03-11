struct RawInstruction(u32);

impl RawInstruction {
    fn opcode(&self) -> u8 { (self.0 & 0x7F) as u8 }
    fn rd(&self) -> u8     { ((self.0 >> 7) & 0x1F) as u8 }
    fn funct3(&self) -> u8 { ((self.0 >> 12) & 0x7) as u8 }
    fn rs1(&self) -> u8    { ((self.0 >> 15) & 0x1F) as u8 }
    fn rs2(&self) -> u8    { ((self.0 >> 20) & 0x1F) as u8 }
    fn funct7(&self) -> u8 { ((self.0 >> 25) & 0x7F) as u8 }
    fn imm_store(&self) -> u16 { ((((self.0 >> 7) & 0x1F | ((self.0 >> 25) & 0x7F) << 5) << 20) as i32 >> 20).try_into().unwrap() }
}

#[derive(Debug)]
pub enum Instruction {
    Add {rd:u8, rs1:u8, rs2:u8},
    Sub {rd:u8, rs1:u8, rs2:u8},
    Sll {rd:u8, rs1:u8, rs2:u8},
    Slt {rd:u8, rs1:u8, rs2:u8},
    Sltu {rd:u8, rs1:u8, rs2:u8},
    Xor {rd:u8, rs1:u8, rs2:u8},
    Srl {rd:u8, rs1:u8, rs2:u8},
    Sra {rd:u8, rs1:u8, rs2:u8},
    Or {rd:u8, rs1:u8, rs2:u8},
    And {rd:u8, rs1:u8, rs2:u8},
    Sb {rs1:u8, rs2:u8, imm:u16},
    Sh {rs1:u8, rs2:u8, imm:u16},
    Sw {rs1:u8, rs2:u8, imm:u16},
}

#[derive(Debug)]
pub enum Extension {
    I
}

impl Instruction {
    pub fn extension(&self) -> Option<Extension> {
        match self {
            Self::Add { .. } | Self::Sub { .. } | Self::Sll { .. } |
            Self::Slt { .. } | Self::Sltu { .. } | Self::Xor { .. } |
            Self::Srl { .. } | Self::Sra { .. } | Self::Or { .. } |
            Self::And { .. } | Self::Sb { .. } | Self::Sw { .. } |
            Self::Sh { .. } => Some(Extension::I),

            _ => None,
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
                        _ => todo!(),
                    )*
                }
            }
        }
    };
}

impl_pretty_print!(Instruction {
    Add
});

#[derive(Debug)]
pub enum DecodeError {
    InvalidOpcode(u8)
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
                _ => Err(DecodeError::InvalidOpcode(instr.opcode())),
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
                _ => Err(DecodeError::InvalidOpcode(instr.opcode())),
            },
            _ => Err(DecodeError::InvalidOpcode(instr.opcode())),
        }
    }
}

// CONSTANTS


// OP_ALU

pub const OP_ALU: u8 = 0b0110011;

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
