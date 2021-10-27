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

#[derive(Debug, PartialEq)]
pub enum Src {
    K,
    X,
}
