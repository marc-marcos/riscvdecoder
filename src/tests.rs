use super::*;

use assert_matches::assert_matches;

macro_rules! test_instruction {
    ($test_name:ident, $raw_hex:expr, $variant:ident, $display:literal) => {
        #[test]
        fn $test_name() {
            let raw: u32 = $raw_hex;
            let instr = Instruction::try_from(raw);
            
            assert_matches!(instr, Ok(Instruction::$variant { .. }));
            assert_eq!(instr.unwrap().to_string(), $display);
        }
    };
}

test_instruction!(test_add, 0x00c982b3, Add, "add");
test_instruction!(test_sub, 0x41268333, Sub, "sub");
test_instruction!(test_sll, 0x009396b3, Sll, "sll");
test_instruction!(test_slt, 0x00b82eb3, Slt, "slt");
test_instruction!(test_sltu, 0x005c3433, Sltu, "sltu");
test_instruction!(test_xor, 0x0156ce33, Xor, "xor");
test_instruction!(test_srl, 0x01cb55b3, Srl, "srl");
test_instruction!(test_sra, 0x41cb55b3, Sra, "sra");
test_instruction!(test_or, 0x01e6e4b3, Or, "or");
test_instruction!(test_and, 0x0069f7b3, And, "and");
test_instruction!(test_addi, 0x00098793, Addi, "addi");
test_instruction!(test_slli, 0x00c69493, Slli, "slli");
test_instruction!(test_slti, 0x00a9a793, Slti, "slti");
test_instruction!(test_sltiu, 0x00fc3e13, Sltiu, "sltiu");
test_instruction!(test_andi, 0x00c69493, Slli, "slli");
test_instruction!(test_ori, 0x07b2e493, Ori, "ori");
test_instruction!(test_xori, 0x00004393, Xori, "xori");
test_instruction!(test_srli, 0x02b95293, Srli, "srli");
test_instruction!(test_srai, 0x42b95293, Srai, "srai");
test_instruction!(test_sb, 0x00960c23, Sb, "sb");
test_instruction!(test_sw, 0x00962c23, Sw, "sw");
test_instruction!(test_sh, 0x00961c23, Sh, "sh");
test_instruction!(test_lb, 0x01860483, Lb, "lb");
test_instruction!(test_lh, 0x01861483, Lh, "lh");
test_instruction!(test_lw, 0x01862483, Lw, "lw");
test_instruction!(test_lbu, 0x01864483, Lbu, "lbu");
test_instruction!(test_lhu, 0x01865483, Lhu, "lhu");
test_instruction!(test_beq, 0x07328d63, Beq, "beq");
test_instruction!(test_bne, 0x07329d63, Bne, "bne");
test_instruction!(test_blt, 0x0732cd63, Blt, "blt");
test_instruction!(test_bge, 0x0732dd63, Bge, "bge");
test_instruction!(test_bltu, 0x0732ed63, Bltu, "bltu");
test_instruction!(test_bgeu, 0x0732fd63, Bgeu, "bgeu");
test_instruction!(test_jal, 0x1d3009ef, Jal, "jal");
test_instruction!(test_jalr, 0x0f3289e7, Jalr, "jalr");
test_instruction!(test_lui, 0x00dc14b7, Lui, "lui");
test_instruction!(test_auipc, 0x4c4e2997, Auipc, "auipc");
test_instruction!(test_ecall, 0x00000073, Ecall, "ecall");
test_instruction!(test_ebreak, 0x00100073, Ebreak, "ebreak");
test_instruction!(test_fence, 0x0a50000f, Fence, "fence");

test_instruction!(test_mul, 0x026684b3, Mul, "mul");
test_instruction!(test_mulh, 0x029f97b3, Mulh, "mulh");
test_instruction!(test_mulhu, 0x02b93eb3, Mulhu, "mulhu");
test_instruction!(test_mulhsu, 0x03e926b3, Mulhsu, "mulhsu");
test_instruction!(test_mulw, 0x02d482bb, Mulw, "mulw");
test_instruction!(test_div, 0x02b4c2b3, Div, "div");
test_instruction!(test_divu, 0x02be52b3, Divu, "divu");
test_instruction!(test_rem, 0x036ee4b3, Rem, "rem");
test_instruction!(test_remu, 0x027f7333, Remu, "remu");
test_instruction!(test_divw, 0x0376c33b, Divw, "divw");
test_instruction!(test_divuw, 0x0353d5bb, Divuw, "divuw");
test_instruction!(test_remw, 0x031962bb, Remw, "remw");
test_instruction!(test_remuw, 0x032bf4bb, Remuw, "remuw");

test_instruction!(test_ld, 0x1c59b583, Ld, "ld");
test_instruction!(test_sd, 0x989f3a23, Sd, "sd");
test_instruction!(test_lwu, 0x977be683, Lwu, "lwu");
test_instruction!(test_addw, 0x00bf093b, Addw, "addw");
test_instruction!(test_addiw, 0x5479829b, Addiw, "addiw");
test_instruction!(test_subw, 0x418682bb, Subw, "subw");
test_instruction!(test_sllw, 0x00e992bb, Sllw, "sllw");
test_instruction!(test_slliw, 0x00d3199b, Slliw, "slliw");
test_instruction!(test_srlw, 0x00f4dfbb, Srlw, "srlw");
test_instruction!(test_srliw, 0x0089531b, Srliw, "srliw");
test_instruction!(test_sraw, 0x4179d33b, Sraw, "sraw");
test_instruction!(test_sraiw, 0x4059d31b, Sraiw, "sraiw");

test_instruction!(test_lrw, 0x1005a52f, Lrw, "lrw");
test_instruction!(test_lrd, 0x1005b52f, Lrd, "lrd");
test_instruction!(test_scw, 0x18c5a52f, Scw, "scw");
test_instruction!(test_scd, 0x18c5b52f, Scd, "scd");

test_instruction!(test_amoswapw, 0x0861a9af, Amoswapw, "amoswapw");
test_instruction!(test_amoswapd, 0x0927b32f, Amoswapd, "amoswapd");
test_instruction!(test_amoaddw, 0x013622af, Amoaddw, "amoaddw");
test_instruction!(test_amoaddd, 0x0095b32f, Amoaddd, "amoaddd");
test_instruction!(test_amoandw, 0x6126a2af, Amoandw, "amoandw");
test_instruction!(test_amoandd, 0x6095b32f, Amoandd, "amoandd");
test_instruction!(test_amoorw, 0x4125a32f, Amoorw, "amoorw");
test_instruction!(test_amoord, 0x4097332f, Amoord, "amoord");
test_instruction!(test_amoxorw, 0x2064a62f, Amoxorw, "amoxorw");
test_instruction!(test_amoxord, 0x20d4b5af, Amoxord, "amoxord");
test_instruction!(test_amomaxw, 0xa13fa6af, Amomaxw, "amomaxw");
test_instruction!(test_amomaxd, 0xa127332f, Amomaxd, "amomaxd");
test_instruction!(test_amomaxuw, 0xe00324af, Amomaxuw, "amomaxuw");
test_instruction!(test_amomaxud, 0xe127332f, Amomaxud, "amomaxud");
test_instruction!(test_amominw, 0x8096a2af, Amominw, "amominw");
test_instruction!(test_amomind, 0x8136332f, Amomind, "amomind");
test_instruction!(test_amominuw, 0xc097232f, Amominuw, "amominuw");
test_instruction!(test_amominud, 0xc136b32f, Amominud, "amominud");
