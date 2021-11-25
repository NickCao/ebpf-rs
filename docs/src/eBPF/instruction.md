# instruction

## encoding
64 bit fixed size instructions encoded in host byte order

    63                       31               15   11   7        0
    +------------------------+----------------+----+----+--------+
    |immediate               |offset          |src |dst |opcode  |
    +------------------------+----------------+----+----+--------+

from most significant bit to least significant bit  
 - 32 bit signed immediate
 - 16 bit signed offset
 - 4 bit source register
 - 4 bit destination register
 - 8 bit opcode

when field are not used, they should be zeroed

## opcode
opcode has two different encodings  

    type 1                type 2
    7    4    2     0     7    3   2     0
    +----+----+-----+     +----+---+-----+
    |mode|size|class|     |op  |src|class|
    +----+----+-----+     +----+---+-----+

which can be distinguished by the class field

    type 1
    LD    0x0 // load
    LDX   0x1
    ST    0x2 // store
    STX   0x3

    type 2
    ALU   0x4
    JMP   0x5
    JMP32 0x6
    ALU64 0x7

## type 1
type 1 instructions are generally related to memory access, with few specialized exceptions for accessing packet data, which is out of the scope of this document  

the `size` field specifies the size of memory to access  

    W   0x0  // 4 byte
    H   0x1  // 2 byte
    B   0x2  // 1 byte
    DW  0x3  // 8 byte

the `mode` field specifies the addressing mode

    IMM   0x0  // immediate
    ABS   0x1  // packet
    IND   0x2  // packet
    MEM   0x3  // memory
    LEN   0x4  // reserved
    MSH   0x5  // reserved
    XADD  0x6  // exclusive (atomic) add

## type 2
type 2 instructions are for alu and control flow operations

the `src` field specifies the source of operand

    K  0x0  // immediate
    X  0x1  // register

the `op` field specifies the operation  

for `class` in `ALU` or `ALU64`

    BPF_ADD   0x00
    BPF_SUB   0x10
    BPF_MUL   0x20
    BPF_DIV   0x30
    BPF_OR    0x40
    BPF_AND   0x50
    BPF_LSH   0x60
    BPF_RSH   0x70
    BPF_NEG   0x80
    BPF_MOD   0x90
    BPF_XOR   0xa0
    BPF_MOV   0xb0
    BPF_ARSH  0xc0
    BPF_END   0xd0  // endianness conversion

for `class` in `JMP` or `JMP32`

    BPF_JA    0x00  // JMP only
    BPF_JEQ   0x10
    BPF_JGT   0x20
    BPF_JGE   0x30
    BPF_JSET  0x40
    BPF_JNE   0x50
    BPF_JSGT  0x60
    BPF_JSGE  0x70
    BPF_CALL  0x80  // JMP only, function call
    BPF_EXIT  0x90  // JMP only, function return
    BPF_JLT   0xa0
    BPF_JLE   0xb0
    BPF_JSLT  0xc0
    BPF_JSLE  0xd0

`JMP32` and `ALU` instructions operate on the lower 32 bits of the registers, zeroing the upper half of destination register
