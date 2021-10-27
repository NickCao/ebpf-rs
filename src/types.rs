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
    class: Class,
    op: OpCode,
}

#[derive(Debug, PartialEq)]
pub enum OpCode {
    Class1(Op, Src),
    Class2(Mode, Size),
}

impl Default for Inst {
    fn default() -> Self {
        Self {
            imm: 0,
            off: 0,
            srcreg: 0,
            dstreg: 0,
            class: Class::ALU,
            op: OpCode::Class1(Op::ADD, Src::K),
        }
    }
}

impl Inst {
    pub fn decode(inst: u64) -> Option<Self> {
        let cls = match (inst & 0x07) as u32 {
            bpf::BPF_LD => Class::LD,
            bpf::BPF_LDX => Class::LDX,
            bpf::BPF_ST => Class::ST,
            bpf::BPF_STX => Class::STX,
            bpf::BPF_ALU => Class::ALU,
            bpf::BPF_JMP => Class::JMP,
            bpf::BPF_JMP32 => Class::JMP32,
            bpf::BPF_ALU64 => Class::ALU64,
            _ => return None,
        };
        Some(Self {
            imm: (inst >> 32) as i32,
            off: (inst >> 16) as i16,
            srcreg: ((inst >> 12) & 0x0f) as u8,
            dstreg: ((inst >> 8) & 0x0f) as u8,
            class: cls,
            op: match cls {
                Class::LD | Class::LDX | Class::ST | Class::STX => OpCode::Class2(
                    match (inst & 0xe0) as u32 {
                        bpf::BPF_IMM => Mode::IMM,
                        bpf::BPF_ABS => Mode::ABS,
                        bpf::BPF_IND => Mode::IND,
                        bpf::BPF_MEM => Mode::MEM,
                        bpf::BPF_LEN => Mode::LEN,
                        bpf::BPF_MSH => Mode::MSH,
                        bpf::BPF_XADD => Mode::XADD,
                        _ => return None,
                    },
                    match (inst & 0x18) as u32 {
                        bpf::BPF_W => Size::W,
                        bpf::BPF_H => Size::H,
                        bpf::BPF_B => Size::B,
                        bpf::BPF_DW => Size::DW,
                        _ => return None,
                    },
                ),
                Class::ALU | Class::ALU64 => OpCode::Class1(
                    match (inst & 0xf0) as u32 {
                        bpf::BPF_ADD => Op::ADD,
                        bpf::BPF_SUB => Op::SUB,
                        bpf::BPF_MUL => Op::MUL,
                        bpf::BPF_DIV => Op::DIV,
                        bpf::BPF_OR => Op::OR,
                        bpf::BPF_AND => Op::AND,
                        bpf::BPF_LSH => Op::LSH,
                        bpf::BPF_RSH => Op::RSH,
                        bpf::BPF_NEG => Op::NEG,
                        bpf::BPF_MOD => Op::MOD,
                        bpf::BPF_XOR => Op::XOR,
                        bpf::BPF_MOV => Op::MOV,
                        bpf::BPF_ARSH => Op::ARSH,
                        bpf::BPF_END => Op::END,
                        _ => return None,
                    },
                    match (inst & 0x08) as u32 {
                        bpf::BPF_K => Src::K,
                        bpf::BPF_X => Src::X,
                        _ => return None,
                    },
                ),
                Class::JMP | Class::JMP32 => OpCode::Class1(
                    match (inst & 0xf0) as u32 {
                        bpf::BPF_JA => Op::JA,
                        bpf::BPF_JEQ => Op::JEQ,
                        bpf::BPF_JGT => Op::JGT,
                        bpf::BPF_JGE => Op::JGE,
                        bpf::BPF_JSET => Op::JSET,
                        bpf::BPF_JNE => Op::JNE,
                        bpf::BPF_JSGT => Op::JSGT,
                        bpf::BPF_JSGE => Op::JSGE,
                        bpf::BPF_CALL => Op::CALL,
                        bpf::BPF_EXIT => Op::EXIT,
                        bpf::BPF_JLT => Op::JLT,
                        bpf::BPF_JLE => Op::JLE,
                        bpf::BPF_JSLT => Op::JSLT,
                        bpf::BPF_JSLE => Op::JSLE,
                        _ => return None,
                    },
                    match (inst & 0x08) as u32 {
                        bpf::BPF_K => Src::K,
                        bpf::BPF_X => Src::X,
                        _ => return None,
                    },
                ),
            },
        })
    }
}

#[cfg(test)]
#[test]
fn inst() {
    let insts: [(u64, Inst); 9] = [
        (
            (bpf::BPF_ADD | bpf::BPF_X | bpf::BPF_ALU).into(),
            Inst {
                class: Class::ALU,
                op: OpCode::Class1(Op::ADD, Src::X),
                ..Inst::default()
            },
        ),
        (
            (bpf::BPF_XOR | bpf::BPF_K | bpf::BPF_ALU).into(),
            Inst {
                class: Class::ALU,
                op: OpCode::Class1(Op::XOR, Src::K),
                ..Inst::default()
            },
        ),
        (
            (bpf::BPF_MOV | bpf::BPF_X | bpf::BPF_ALU).into(),
            Inst {
                class: Class::ALU,
                op: OpCode::Class1(Op::MOV, Src::X),
                ..Inst::default()
            },
        ),
        (
            (bpf::BPF_ADD | bpf::BPF_X | bpf::BPF_ALU64).into(),
            Inst {
                class: Class::ALU64,
                op: OpCode::Class1(Op::ADD, Src::X),
                ..Inst::default()
            },
        ),
        (
            (bpf::BPF_IND | bpf::BPF_W | bpf::BPF_LD).into(),
            Inst {
                class: Class::LD,
                op: OpCode::Class2(Mode::IND, Size::W),
                ..Inst::default()
            },
        ),
        (
            (bpf::BPF_XADD | bpf::BPF_W | bpf::BPF_STX).into(),
            Inst {
                class: Class::STX,
                op: OpCode::Class2(Mode::XADD, Size::W),
                ..Inst::default()
            },
        ),
        (
            (bpf::BPF_XADD | bpf::BPF_DW | bpf::BPF_STX).into(),
            Inst {
                class: Class::STX,
                op: OpCode::Class2(Mode::XADD, Size::DW),
                ..Inst::default()
            },
        ),
        (
            (bpf::BPF_LD | bpf::BPF_DW | bpf::BPF_IMM).into(),
            Inst {
                class: Class::LD,
                op: OpCode::Class2(Mode::IMM, Size::DW),
                ..Inst::default()
            },
        ),
        (
            0x000000ff000001b4,
            Inst {
                imm: 0x000000ff,
                dstreg: 1,
                class: Class::ALU,
                op: OpCode::Class1(Op::MOV, Src::K),
                ..Inst::default()
            },
        ),
    ];
    for (inst, decoded) in insts {
        assert_eq!(decoded, Inst::decode(inst as u64).unwrap());
    }
}
