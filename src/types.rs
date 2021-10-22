use crate::binding::*;

#[derive(Debug)]
pub enum Class {
    LD,
    LDX,
    ST,
    STX,
    ALU,
    JMP,
    RET,
    MISC,
}

#[derive(Debug)]
pub enum Size {
    W,
    H,
    B,
    DW,
}

#[derive(Debug)]
pub enum Mode {
    IMM,
    ABS,
    IND,
    MEM,
    LEN,
    MSH,
}

#[derive(Debug)]
pub enum Op {
    ADD,
    SUB,
    MUL,
    DIV,
    OR,
    AND,
    LSH,
    RSH,
    NEG,
    MOD,
    XOR,
    JA,
    JEQ,
    JGT,
    JGE,
    JSET,
}

#[derive(Debug)]
pub enum Src {
    K,
    X,
}

pub fn class(inst: u64) -> Option<Class> {
    match (inst & 0x07) as u32 {
        bpf::BPF_LD => Some(Class::LD),
        bpf::BPF_LDX => Some(Class::LDX),
        bpf::BPF_ST => Some(Class::ST),
        bpf::BPF_STX => Some(Class::STX),
        bpf::BPF_ALU => Some(Class::ALU),
        bpf::BPF_JMP => Some(Class::JMP),
        bpf::BPF_RET => Some(Class::RET),
        bpf::BPF_MISC => Some(Class::MISC),
        _ => None,
    }
}

pub fn size(inst: u64) -> Option<Size> {
    match (inst & 0x18) as u32 {
        bpf::BPF_W => Some(Size::W),
        bpf::BPF_H => Some(Size::H),
        bpf::BPF_B => Some(Size::B),
        bpf::BPF_DW => Some(Size::DW),
        _ => None,
    }
}

pub fn mode(inst: u64) -> Option<Mode> {
    match (inst & 0xe0) as u32 {
        bpf::BPF_IMM => Some(Mode::IMM),
        bpf::BPF_ABS => Some(Mode::ABS),
        bpf::BPF_IND => Some(Mode::IND),
        bpf::BPF_MEM => Some(Mode::MEM),
        bpf::BPF_LEN => Some(Mode::LEN),
        bpf::BPF_MSH => Some(Mode::MSH),
        _ => None,
    }
}

pub fn op(inst: u64) -> Option<Op> {
    match (inst & 0xf0) as u32 {
        bpf::BPF_ADD => Some(Op::ADD),
        bpf::BPF_SUB => Some(Op::SUB),
        bpf::BPF_MUL => Some(Op::MUL),
        bpf::BPF_DIV => Some(Op::DIV),
        bpf::BPF_OR => Some(Op::OR),
        bpf::BPF_AND => Some(Op::AND),
        bpf::BPF_LSH => Some(Op::LSH),
        bpf::BPF_RSH => Some(Op::RSH),
        bpf::BPF_NEG => Some(Op::NEG),
        bpf::BPF_MOD => Some(Op::MOD),
        bpf::BPF_XOR => Some(Op::XOR),
        // TODO: check mode
        bpf::BPF_JA => Some(Op::JA),
        bpf::BPF_JEQ => Some(Op::JEQ),
        bpf::BPF_JGT => Some(Op::JGT),
        bpf::BPF_JGE => Some(Op::JGE),
        bpf::BPF_JSET => Some(Op::JSET),
        _ => None,
    }
}

pub fn src(inst: u64) -> Option<Src> {
    match (inst & 0x08) as u32 {
        bpf::BPF_K => Some(Src::K),
        bpf::BPF_X => Some(Src::X),
        _ => None,
    }
}
