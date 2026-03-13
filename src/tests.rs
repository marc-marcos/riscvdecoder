use super::*;

#[test]
fn test_add() {
    let raw: u32 = 0x00c982b3;

    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Add { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Add), got {:?}", instr);
    }
}

#[test]
fn test_sub() {
    let raw: u32 = 0x41268333;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Sub { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Sub), got {:?}", instr);
    }
}

#[test]
fn test_sll() {
    let raw: u32 = 0x009396b3;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Sll { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Sll), got {:?}", instr);
    }
}

#[test]
fn test_slt() {
    let raw: u32 = 0x00b82eb3;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Slt { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Slt), got {:?}", instr);
    }
}

#[test]
fn test_sltu() {
    let raw: u32 = 0x005c3433;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Sltu { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Sltu), got {:?}", instr);
    }
}

#[test]
fn test_xor() {
    let raw: u32 = 0x0156ce33;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Xor { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Xor), got {:?}", instr);
    }
}

#[test]
fn test_srl() {
    let raw: u32 = 0x01cb55b3;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Srl { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Srl), got {:?}", instr);
    }
}

#[test]
fn test_sra() {
    let raw: u32 = 0x41cb55b3;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Sra { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Sra), got {:?}", instr);
    }
}

#[test]
fn test_or() {
    let raw: u32 = 0x01e6e4b3;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Or { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Or), got {:?}", instr);
    }
}

#[test]
fn test_and() {
    let raw: u32 = 0x0069f7b3;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::And { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Sltu), got {:?}", instr);
    }
}

#[test]
fn test_addi() {
    let raw: u32 = 0x00098793;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Addi { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Addi), got {:?}", instr);
    }
}

#[test]
fn test_slli() {
    let raw: u32 = 0x00c69493;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Slli { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Slli), got {:?}", instr);
    }
}

#[test]
fn test_slti() {
    let raw: u32 = 0x00a9a793;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Slti { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Slti), got {:?}", instr);
    }
}

#[test]
fn test_sltiu() {
    let raw: u32 = 0x00fc3e13;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Sltiu { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Sltiu), got {:?}", instr);
    }
}

#[test]
fn test_andi() {
    let raw: u32 = 0x00c69493;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Slli { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Slli), got {:?}", instr);
    }
}

#[test]
fn test_ori() {
    let raw: u32 = 0x07b2e493;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Ori { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Ori), got {:?}", instr);
    }
}

#[test]
fn test_xori() {
    let raw: u32 = 0x00004393;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Xori { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Xori), got {:?}", instr);
    }
}

#[test]
fn test_srli() {
    let raw: u32 = 0x02b95293;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Srli { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Srli), got {:?}", instr);
    }
}

#[test]
fn test_srai() {
    let raw: u32 = 0x42b95293;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Srai { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Srai), got {:?}", instr);
    }
}

#[test]
fn test_sb() {
    let raw: u32 = 0x00960c23;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Sb { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Sb), got {:?}", instr);
    }
}

#[test]
fn test_sw() {
    let raw: u32 = 0x00962c23;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Sw { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Sw), got {:?}", instr);
    }
}

#[test]
fn test_sh() {
    let raw: u32 = 0x00961c23;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Sh { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Sh), got {:?}", instr);
    }
}

#[test]
fn test_lb() {
    let raw: u32 = 0x01860483;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Lb { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Lb), got {:?}", instr);
    }
}

#[test]
fn test_lh() {
    let raw: u32 = 0x01861483;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Lh { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Lh), got {:?}", instr);
    }
}

#[test]
fn test_lw() {
    let raw: u32 = 0x01862483;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Lw { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Lw), got {:?}", instr);
    }
}

#[test]
fn test_lbu() {
    let raw: u32 = 0x01864483;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Lbu { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Lbu), got {:?}", instr);
    }
}

#[test]
fn test_lhu() {
    let raw: u32 = 0x01865483;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Lhu { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Lhu), got {:?}", instr);
    }
}

#[test]
fn test_beq() {
    let raw: u32 = 0x07328d63;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Beq { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Beq), got {:?}", instr);
    }
}

#[test]
fn test_bne() {
    let raw: u32 = 0x07329d63;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Bne { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Bne), got {:?}", instr);
    }
}

#[test]
fn test_blt() {
    let raw: u32 = 0x0732cd63;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Blt { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Blt), got {:?}", instr);
    }
}

#[test]
fn test_bge() {
    let raw: u32 = 0x0732dd63;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Bge { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Bge), got {:?}", instr);
    }
}

#[test]
fn test_bltu() {
    let raw: u32 = 0x0732ed63;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Bltu { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Bltu), got {:?}", instr);
    }
}

#[test]
fn test_bgeu() {
    let raw: u32 = 0x0732fd63;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Bgeu { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Bgeu), got {:?}", instr);
    }
}

#[test]
fn test_jal() {
    let raw: u32 = 0x1d3009ef;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Jal { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Jal), got {:?}", instr);
    }
}

#[test]
fn test_jalr() {
    let raw: u32 = 0x0f3289e7;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Jalr { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Jalr), got {:?}", instr);
    }
}

#[test]
fn test_lui() {
    let raw: u32 = 0x00dc14b7;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Lui { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Lui), got {:?}", instr);
    }
}

#[test]
fn test_auipc() {
    let raw: u32 = 0x4c4e2997;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Auipc { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Auipc), got {:?}", instr);
    }
}

#[test]
fn test_ecall() {
    let raw: u32 = 0x00000073;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Ecall { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Ecall), got {:?}", instr);
    }
}

#[test]
fn test_ebreak() {
    let raw: u32 = 0x00100073;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Ebreak { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Ebreak), got {:?}", instr);
    }
}

#[test]
fn test_fence() {
    let raw: u32 = 0x0a50000f;
    let instr: Result<Instruction, DecodeError> = Instruction::try_from(raw);

    if let Ok(Instruction::Fence { .. }) = instr {
        assert_eq!(1, 1);
    } else {
        panic!("Expected Ok(Instruction::Fence), got {:?}", instr);
    }
}
