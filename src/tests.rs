use super::*;

macro_rules! test_instruction {
    ($test_name:ident, $raw_hex:expr, $variant:ident) => {
        #[test]
        fn $test_name() {
            let raw: u32 = $raw_hex;
            let instr = Instruction::try_from(raw);

            // We use the "matches!" macro or "if let" to check the variant
            if !matches!(instr, Ok(Instruction::$variant { .. })) {
                panic!(
                    "Expected Ok(Instruction::{}), but got {:?}",
                    stringify!($variant),
                    instr
                );
            }
        }
    };
}

test_instruction!(test_add, 0x00c982b3, Add);
test_instruction!(test_sub, 0x41268333, Sub);
test_instruction!(test_sll, 0x009396b3, Sll);
test_instruction!(test_slt, 0x00b82eb3, Slt);
test_instruction!(test_sltu, 0x005c3433, Sltu);
test_instruction!(test_xor, 0x0156ce33, Xor);
test_instruction!(test_srl, 0x01cb55b3, Srl);
test_instruction!(test_sra, 0x41cb55b3, Sra);
test_instruction!(test_or, 0x01e6e4b3, Or);
test_instruction!(test_and, 0x0069f7b3, And);
test_instruction!(test_addi, 0x00098793, Addi);
test_instruction!(test_slli, 0x00c69493, Slli);
test_instruction!(test_slti, 0x00a9a793, Slti);
test_instruction!(test_sltiu, 0x00fc3e13, Sltiu);
test_instruction!(test_andi, 0x00c69493, Slli);
test_instruction!(test_ori, 0x07b2e493, Ori);
test_instruction!(test_xori, 0x00004393, Xori);
test_instruction!(test_srli, 0x02b95293, Srli);
test_instruction!(test_srai, 0x42b95293, Srai);
test_instruction!(test_sb, 0x00960c23, Sb);
test_instruction!(test_sw, 0x00962c23, Sw);
test_instruction!(test_sh, 0x00961c23, Sh);
test_instruction!(test_lb, 0x01860483, Lb);
test_instruction!(test_lh, 0x01861483, Lh);
test_instruction!(test_lw, 0x01862483, Lw);
test_instruction!(test_lbu, 0x01864483, Lbu);
test_instruction!(test_lhu, 0x01865483, Lhu);
test_instruction!(test_beq, 0x07328d63, Beq);
test_instruction!(test_bne, 0x07329d63, Bne);
test_instruction!(test_blt, 0x0732cd63, Blt);
test_instruction!(test_bge, 0x0732dd63, Bge);
test_instruction!(test_bltu, 0x0732ed63, Bltu);
test_instruction!(test_bgeu, 0x0732fd63, Bgeu);
test_instruction!(test_jal, 0x1d3009ef, Jal);
test_instruction!(test_jalr, 0x0f3289e7, Jalr);
test_instruction!(test_lui, 0x00dc14b7, Lui);
test_instruction!(test_auipc, 0x4c4e2997, Auipc);
test_instruction!(test_ecall, 0x00000073, Ecall);
test_instruction!(test_ebreak, 0x00100073, Ebreak);
test_instruction!(test_fence, 0x0a50000f, Fence);
test_instruction!(test_mul, 0x026684b3, Mul);
test_instruction!(test_mulh, 0x029f97b3, Mulh);
test_instruction!(test_mulhu, 0x02b93eb3, Mulhu);
test_instruction!(test_mulhsu, 0x03e926b3, Mulhsu);
test_instruction!(test_mulw, 0x02d482bb, Mulw);
test_instruction!(test_div, 0x02b4c2b3, Div);
test_instruction!(test_divu, 0x02be52b3, Divu);
test_instruction!(test_rem, 0x036ee4b3, Rem);
test_instruction!(test_remu, 0x027f7333, Remu);
test_instruction!(test_divw, 0x0376c33b, Divw);
test_instruction!(test_divuw, 0x0353d5bb, Divuw);
test_instruction!(test_remw, 0x031962bb, Remw);
test_instruction!(test_remuw, 0x032bf4bb, Remuw);
test_instruction!(test_ld, 0x1c59b583, Ld);
test_instruction!(test_sd, 0x989f3a23, Sd);
test_instruction!(test_lwu, 0x977be683, Lwu);
test_instruction!(test_addw, 0x00bf093b, Addw);
test_instruction!(test_addiw, 0x5479829b, Addiw);
test_instruction!(test_subw, 0x418682bb, Subw);
test_instruction!(test_sllw, 0x00e992bb, Sllw);
test_instruction!(test_slliw, 0x00d3199b, Slliw);
test_instruction!(test_srlw, 0x00f4dfbb, Srlw);
test_instruction!(test_srliw, 0x0089531b, Srliw);
test_instruction!(test_sraw, 0x4179d33b, Sraw);
test_instruction!(test_sraiw, 0x4059d31b, Sraiw);
