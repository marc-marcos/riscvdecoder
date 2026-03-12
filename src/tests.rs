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
        panic!("Expected Ok(Instruction::Sltu), got {:?}", instr);
    }
}

/*
#[test]
fn test_slli() {
}

#[test]
fn test_slti() {
}

#[test]
fn test_sltiu() {
}

#[test]
fn test_andi() {
}

#[test]
fn test_ori() {
}

#[test]
fn test_xori() {
}

#[test]
fn test_srli() {
}

#[test]
fn test_srai() {
}

#[test]
fn test_sb() {
}

#[test]
fn test_sw() {
}

#[test]
fn test_sh() {
}

#[test]
fn test_lb() {
}

#[test]
fn test_lh() {
}

#[test]
fn test_lw() {
}

#[test]
fn test_lbu() {
}

#[test]
fn test_lhu() {
}

#[test]
fn test_beq() {
}

#[test]
fn test_bne() {
}

#[test]
fn test_blt() {
}

#[test]
fn test_bge() {
}

#[test]
fn test_bltu() {
}

#[test]
fn test_bgeu() {
}

#[test]
fn test_jal() {
}

#[test]
fn test_jalr() {
}

#[test]
fn test_lui() {
}

#[test]
fn test_auipc() {
}
*/
