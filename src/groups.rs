use create::Instruction;

pub struct Group(
    pub &'static str, // Name
    pub &'static [Instruction] // Included instruction in the group
)


pub const ALL_GROUPS: &[Group] = &[MEMORY];

pub const MEMORY: Group = Group(
    "Memory operations",
    &[Instruction::Lb, Instruction::Lh, Instruction::Lbu, Instruction::Lhu, Instruction::Ld, Instruction::Sd, Instruction::Lwu, Instruction::Lrw, Instruction::Lrd, Instruction::Scw, Instruction::Scd, Instruction::Flw, Instruction::Fsw, Instruction::Fld, Instruction::Fsd],
)
