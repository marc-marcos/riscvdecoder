use super::*;

#[test]
fn test_add() {
    let raw : u32 = 0x00c982b3;

    let instr : Result<Instruction, DecodeError> = raw.try_from();


}

#[test]
fn test_sub() {
}

#[test]
fn test_sll() {
}

#[test]
fn test_slt() {
}

#[test]
fn test_sltu() {
}

#[test]
fn test_xor() {
}

#[test]
fn test_srl() {
}

#[test]
fn test_sra() {
}

#[test]
fn test_or() {
}

#[test]
fn test_and() {
}

#[test]
fn test_addi() {
}

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
