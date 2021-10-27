use crate::binding::*;
use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Size {
    W,
    H,
    B,
    DW,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    IMM,
    ABS,
    IND,
    MEM,
    LEN,
    MSH,
    XADD,
}

#[derive(Debug, PartialEq)]
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

#[derive(PartialEq)]
pub enum Src {
    K,
    X,
}

impl fmt::Debug for Src {
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

#[derive(Debug, PartialEq)]
pub struct Inst {
    imm: i32,
    off: i16,
    srcreg: u8,
    dstreg: u8,
    class: Option<Class>,
    size: Option<Size>,
    mode: Option<Mode>,
    op: Option<Op>,
    src: Option<Src>,
}

impl Default for Inst {
    fn default() -> Self {
        Self {
            imm: 0,
            off: 0,
            srcreg: 0,
            dstreg: 0,
            class: None,
            size: None,
            mode: None,
            op: None,
            src: None,
        }
    }
}

impl Inst {
    pub fn decode(inst: u64) -> Self {
        let cls = class(inst).unwrap();
        Self {
            imm: immediate(inst),
            off: offset(inst),
            srcreg: srcreg(inst),
            dstreg: dstreg(inst),
            class: Some(cls),
            size: size(inst, cls),
            mode: mode(inst, cls),
            op: op(inst, cls),
            src: src(inst, cls),
        }
    }
}

pub fn immediate(inst: u64) -> i32 {
    (inst >> 32) as i32
}

pub fn offset(inst: u64) -> i16 {
    (inst >> 16) as i16
}

pub fn srcreg(inst: u64) -> u8 {
    ((inst >> 12) & 0x0f) as u8
}

pub fn dstreg(inst: u64) -> u8 {
    ((inst >> 8) & 0x0f) as u8
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

pub fn size(inst: u64, cls: Class) -> Option<Size> {
    match cls {
        Class::LD | Class::LDX | Class::ST | Class::STX => match (inst & 0x18) as u32 {
            bpf::BPF_W => Some(Size::W),
            bpf::BPF_H => Some(Size::H),
            bpf::BPF_B => Some(Size::B),
            bpf::BPF_DW => Some(Size::DW),
            _ => None,
        },
        _ => None,
    }
}

pub fn mode(inst: u64, cls: Class) -> Option<Mode> {
    match cls {
        Class::LD | Class::LDX | Class::ST | Class::STX => match (inst & 0xe0) as u32 {
            bpf::BPF_IMM => Some(Mode::IMM),
            bpf::BPF_ABS => Some(Mode::ABS),
            bpf::BPF_IND => Some(Mode::IND),
            bpf::BPF_MEM => Some(Mode::MEM),
            bpf::BPF_LEN => Some(Mode::LEN),
            bpf::BPF_MSH => Some(Mode::MSH),
            bpf::BPF_XADD => Some(Mode::XADD),
            _ => None,
        },
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
        Class::ALU | Class::ALU64 | Class::JMP | Class::JMP32 => match (inst & 0x08) as u32 {
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
    let insts: [(u64, Inst); 9] = [
        (
            (bpf::BPF_ADD | bpf::BPF_X | bpf::BPF_ALU).into(),
            Inst {
                class: Some(Class::ALU),
                op: Some(Op::ADD),
                src: Some(Src::X),
                ..Inst::default()
            },
        ),
        (
            (bpf::BPF_XOR | bpf::BPF_K | bpf::BPF_ALU).into(),
            Inst {
                class: Some(Class::ALU),
                op: Some(Op::XOR),
                src: Some(Src::K),
                ..Inst::default()
            },
        ),
        (
            (bpf::BPF_MOV | bpf::BPF_X | bpf::BPF_ALU).into(),
            Inst {
                class: Some(Class::ALU),
                op: Some(Op::MOV),
                src: Some(Src::X),
                ..Inst::default()
            },
        ),
        (
            (bpf::BPF_ADD | bpf::BPF_X | bpf::BPF_ALU64).into(),
            Inst {
                class: Some(Class::ALU64),
                op: Some(Op::ADD),
                src: Some(Src::X),
                ..Inst::default()
            },
        ),
        (
            (bpf::BPF_IND | bpf::BPF_W | bpf::BPF_LD).into(),
            Inst {
                class: Some(Class::LD),
                size: Some(Size::W),
                mode: Some(Mode::IND),
                ..Inst::default()
            },
        ),
        (
            (bpf::BPF_XADD | bpf::BPF_W | bpf::BPF_STX).into(),
            Inst {
                class: Some(Class::STX),
                size: Some(Size::W),
                mode: Some(Mode::XADD),
                ..Inst::default()
            },
        ),
        (
            (bpf::BPF_XADD | bpf::BPF_DW | bpf::BPF_STX).into(),
            Inst {
                class: Some(Class::STX),
                size: Some(Size::DW),
                mode: Some(Mode::XADD),
                ..Inst::default()
            },
        ),
        (
            (bpf::BPF_LD | bpf::BPF_DW | bpf::BPF_IMM).into(),
            Inst {
                class: Some(Class::LD),
                size: Some(Size::DW),
                mode: Some(Mode::IMM),
                ..Inst::default()
            },
        ),
        (
            0x000000ff000001b4,
            Inst {
                imm: 0x000000ff,
                dstreg: 1,
                class: Some(Class::ALU),
                op: Some(Op::MOV),
                src: Some(Src::K),
                ..Inst::default()
            },
        ),
    ];
    for (inst, decoded) in insts {
        assert_eq!(decoded, Inst::decode(inst as u64));
    }
}
