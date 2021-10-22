use crate::binding::*;
use core::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Class {
    LD,
    LDX,
    ST,
    STX,
    ALU,
    JMP,
    JMP32,
    ALU64,
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
    XADD,
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
    MOV,
    ARSH,
    END,
    JA,
    JEQ,
    JGT,
    JGE,
    JSET,
    JNE,
    JSGT,
    JSGE,
    CALL,
    EXIT,
    JLT,
    JLE,
    JSLT,
    JSLE,
}

#[derive(Debug)]
pub enum Src {
    K,
    X,
}

impl fmt::Display for Src {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::K => "imm",
                Self::X => "reg",
            }
        )
    }
}

#[derive(Debug)]
pub struct Inst {
    class: Class,
    size: Size,
    mode: Mode,
    op: Option<Op>,
    src: Option<Src>,
}

impl Inst {
    pub fn decode(inst: u64) -> Self {
        let cls = class(inst).unwrap();
        Self {
            class: cls,
            size: size(inst).unwrap(),
            mode: mode(inst).unwrap(),
            op: op(inst, cls),
            src: src(inst, cls),
        }
    }
}

pub fn class(inst: u64) -> Option<Class> {
    match (inst & 0x07) as u32 {
        bpf::BPF_LD => Some(Class::LD),
        bpf::BPF_LDX => Some(Class::LDX),
        bpf::BPF_ST => Some(Class::ST),
        bpf::BPF_STX => Some(Class::STX),
        bpf::BPF_ALU => Some(Class::ALU),
        bpf::BPF_JMP => Some(Class::JMP),
        bpf::BPF_JMP32 => Some(Class::JMP32),
        bpf::BPF_ALU64 => Some(Class::ALU64),
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
        bpf::BPF_XADD => Some(Mode::XADD),
        _ => None,
    }
}

pub fn op(inst: u64, cls: Class) -> Option<Op> {
    match cls {
        Class::ALU | Class::ALU64 => match (inst & 0xf0) as u32 {
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
            bpf::BPF_MOV => Some(Op::MOV),
            bpf::BPF_ARSH => Some(Op::ARSH),
            bpf::BPF_END => Some(Op::END),
            _ => None,
        },
        Class::JMP | Class::JMP32 => match (inst & 0xf0) as u32 {
            bpf::BPF_JA => Some(Op::JA),
            bpf::BPF_JEQ => Some(Op::JEQ),
            bpf::BPF_JGT => Some(Op::JGT),
            bpf::BPF_JGE => Some(Op::JGE),
            bpf::BPF_JSET => Some(Op::JSET),
            bpf::BPF_JNE => Some(Op::JNE),
            bpf::BPF_JSGT => Some(Op::JSGT),
            bpf::BPF_JSGE => Some(Op::JSGE),
            bpf::BPF_CALL => Some(Op::CALL),
            bpf::BPF_EXIT => Some(Op::EXIT),
            bpf::BPF_JLT => Some(Op::JLT),
            bpf::BPF_JLE => Some(Op::JLE),
            bpf::BPF_JSLT => Some(Op::JSLT),
            bpf::BPF_JSLE => Some(Op::JSLE),
            _ => None,
        },
        _ => None,
    }
}

pub fn src(inst: u64, cls: Class) -> Option<Src> {
    match cls {
        Class::ALU | Class::JMP => match (inst & 0x08) as u32 {
            bpf::BPF_K => Some(Src::K),
            bpf::BPF_X => Some(Src::X),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
#[test]
fn inst() {
    println!(
        "{:?}",
        Inst::decode((bpf::BPF_ADD | bpf::BPF_X | bpf::BPF_ALU) as u64)
    );
    println!(
        "{:?}",
        Inst::decode((bpf::BPF_XOR | bpf::BPF_K | bpf::BPF_ALU) as u64)
    );
    println!("{:?}", Inst::decode(0x00000000000001bf));
    println!("{:?}", Inst::decode(0x00000010000000dc));
    println!("{:?}", Inst::decode(0x0000000000000287));
    println!("{:?}", Inst::decode(0x0000000000000095));
}
