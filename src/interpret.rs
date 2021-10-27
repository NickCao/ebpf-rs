use crate::types::*;

pub fn interpret(inst: u64) {
    let imm: u64 = ((inst >> 32) & u32::MAX as u64) as u64;
    let off: u64 = ((inst >> 16) & u16::MAX as u64) as u64;
    let src: usize = ((inst >> 12) & 0x0f) as usize;
    let dst: usize = ((inst >> 8) & 0x0f) as usize;
    let op: u8 = (inst & u8::MAX as u64) as u8;
    let mut reg: [u64; 16] = [0; 16];
    match op {
        ALU_K_ADD => {
            reg[dst] += imm;
            reg[dst] &= u32::MAX as u64;
        }
        ALU_X_ADD => {
            reg[dst] += reg[src];
            reg[dst] &= u32::MAX as u64;
        }
        ALU_K_SUB => {
            reg[dst] -= imm;
            reg[dst] &= u32::MAX as u64;
        }
        ALU_X_SUB => {
            reg[dst] -= reg[src];
            reg[dst] &= u32::MAX as u64;
        }
        ALU_K_MUL => {
            reg[dst] *= imm;
            reg[dst] &= u32::MAX as u64;
        }
        ALU_X_MUL => {
            reg[dst] *= reg[src];
            reg[dst] &= u32::MAX as u64;
        }
        // TODO: check div by zero
        ALU_K_DIV => {
            reg[dst] /= imm;
            reg[dst] &= u32::MAX as u64;
        }
        ALU_X_DIV => {
            reg[dst] /= reg[src];
            reg[dst] &= u32::MAX as u64;
        }
        ALU_K_OR => {
            reg[dst] |= imm;
            reg[dst] &= u32::MAX as u64;
        }
        ALU_X_OR => {
            reg[dst] |= reg[src];
            reg[dst] &= u32::MAX as u64;
        }
        ALU_K_AND => {
            reg[dst] &= imm;
            reg[dst] &= u32::MAX as u64;
        }
        ALU_X_AND => {
            reg[dst] &= reg[src];
            reg[dst] &= u32::MAX as u64;
        }
        ALU_K_LSH => {
            reg[dst] <<= imm;
            reg[dst] &= u32::MAX as u64;
        }
        ALU_X_LSH => {
            reg[dst] <<= reg[src];
            reg[dst] &= u32::MAX as u64;
        }
        ALU_K_RSH => {
            reg[dst] = (reg[dst] & u32::MAX as u64) >> imm;
            reg[dst] &= u32::MAX as u64;
        }
        ALU_X_RSH => {
            reg[dst] = (reg[dst] & u32::MAX as u64) >> reg[src];
            reg[dst] &= u32::MAX as u64;
        }
        ALU_K_NEG => {
            reg[dst] = (-(reg[dst] as i64)) as u64;
            reg[dst] &= u32::MAX as u64;
        }
        ALU_X_NEG => {
            reg[dst] = (-(reg[dst] as i64)) as u64;
            reg[dst] &= u32::MAX as u64;
        }
        // TODO: check div by zero
        ALU_K_MOD => {
            reg[dst] = (reg[dst] as u32 % imm as u32) as u64;
            reg[dst] &= u32::MAX as u64;
        }
        ALU_X_MOD => {
            reg[dst] = (reg[dst] as u32 % reg[src] as u32) as u64;
            reg[dst] &= u32::MAX as u64;
        }
        ALU_K_XOR => {
            reg[dst] ^= imm;
            reg[dst] &= u32::MAX as u64;
        }
        ALU_X_XOR => {
            reg[dst] ^= reg[src];
            reg[dst] &= u32::MAX as u64;
        }
        ALU_K_MOV => {
            reg[dst] = imm;
            reg[dst] &= u32::MAX as u64;
        }
        ALU_X_MOV => {
            reg[dst] = reg[src];
            reg[dst] &= u32::MAX as u64;
        }
        ALU_K_ARSH => {
            reg[dst] = (reg[dst] as i32 >> imm) as u64;
            reg[dst] &= u32::MAX as u64;
        }
        ALU_X_ARSH => {
            reg[dst] = (reg[dst] as i32 >> reg[src] as u32) as u64;
            reg[dst] &= u32::MAX as u64;
        }
        // TODO: check invalid param
        ALU_K_END => match imm {
            16 => reg[dst] = (reg[dst] as u16).to_le() as u64,
            32 => reg[dst] = (reg[dst] as u32).to_le() as u64,
            64 => reg[dst] = (reg[dst] as u64).to_le() as u64,
            _ => {}
        },
        ALU_X_END => match imm {
            16 => reg[dst] = (reg[dst] as u16).to_be() as u64,
            32 => reg[dst] = (reg[dst] as u32).to_be() as u64,
            64 => reg[dst] = (reg[dst] as u64).to_be() as u64,
            _ => {}
        },

        ALU64_K_ADD => {
            reg[dst] += imm;
        }
        ALU64_X_ADD => {
            reg[dst] += reg[src];
        }
        ALU64_K_SUB => {
            reg[dst] -= imm;
        }
        ALU64_X_SUB => {
            reg[dst] -= reg[src];
        }
        ALU64_K_MUL => {
            reg[dst] *= imm;
        }
        ALU64_X_MUL => {
            reg[dst] *= reg[src];
        }
        // TODO: check div by zero
        ALU64_K_DIV => {
            reg[dst] /= imm;
        }
        ALU64_X_DIV => {
            reg[dst] /= reg[src];
        }
        ALU64_K_OR => {
            reg[dst] |= imm;
        }
        ALU64_X_OR => {
            reg[dst] |= reg[src];
        }
        ALU64_K_AND => {
            reg[dst] &= imm;
        }
        ALU64_X_AND => {
            reg[dst] &= reg[src];
        }
        ALU64_K_LSH => {
            reg[dst] <<= imm;
        }
        ALU64_X_LSH => {
            reg[dst] <<= reg[src];
        }
        ALU64_K_RSH => {
            reg[dst] >>= imm;
        }
        ALU64_X_RSH => {
            reg[dst] >>= reg[src];
        }
        ALU64_K_NEG => {
            reg[dst] = (-(reg[dst] as i64)) as u64;
        }
        ALU64_X_NEG => {
            reg[dst] = (-(reg[dst] as i64)) as u64;
        }
        // TODO: check div by zero
        ALU64_K_MOD => {
            reg[dst] %= imm;
        }
        ALU64_X_MOD => {
            reg[dst] %= reg[src];
        }
        ALU64_K_XOR => {
            reg[dst] ^= imm;
        }
        ALU64_X_XOR => {
            reg[dst] ^= reg[src];
        }
        ALU64_K_MOV => {
            reg[dst] = imm;
        }
        ALU64_X_MOV => {
            reg[dst] = reg[src];
        }
        ALU64_K_ARSH => {
            reg[dst] = (reg[dst] as i64 >> imm) as u64;
        }
        ALU64_X_ARSH => {
            reg[dst] = (reg[dst] as i64 >> reg[src]) as u64;
        }
        ALU64_K_END => {}
        ALU64_X_END => {}

        JMP_K_JA => {}
        JMP_X_JA => {}
        JMP_K_JEQ => {}
        JMP_X_JEQ => {}
        JMP_K_JGT => {}
        JMP_X_JGT => {}
        JMP_K_JGE => {}
        JMP_X_JGE => {}
        JMP_K_JSET => {}
        JMP_X_JSET => {}
        JMP_K_JNE => {}
        JMP_X_JNE => {}
        JMP_K_JSGT => {}
        JMP_X_JSGT => {}
        JMP_K_JSGE => {}
        JMP_X_JSGE => {}
        JMP_K_CALL => {}
        JMP_X_CALL => {}
        JMP_K_EXIT => {}
        JMP_X_EXIT => {}
        JMP_K_JLT => {}
        JMP_X_JLT => {}
        JMP_K_JLE => {}
        JMP_X_JLE => {}
        JMP_K_JSLT => {}
        JMP_X_JSLT => {}
        JMP_K_JSLE => {}
        JMP_X_JSLE => {}

        JMP32_K_JA => {}
        JMP32_X_JA => {}
        JMP32_K_JEQ => {}
        JMP32_X_JEQ => {}
        JMP32_K_JGT => {}
        JMP32_X_JGT => {}
        JMP32_K_JGE => {}
        JMP32_X_JGE => {}
        JMP32_K_JSET => {}
        JMP32_X_JSET => {}
        JMP32_K_JNE => {}
        JMP32_X_JNE => {}
        JMP32_K_JSGT => {}
        JMP32_X_JSGT => {}
        JMP32_K_JSGE => {}
        JMP32_X_JSGE => {}
        JMP32_K_CALL => {}
        JMP32_X_CALL => {}
        JMP32_K_EXIT => {}
        JMP32_X_EXIT => {}
        JMP32_K_JLT => {}
        JMP32_X_JLT => {}
        JMP32_K_JLE => {}
        JMP32_X_JLE => {}
        JMP32_K_JSLT => {}
        JMP32_X_JSLT => {}
        JMP32_K_JSLE => {}
        JMP32_X_JSLE => {}

        LD_IMM_B => {}
        LD_IMM_H => {}
        LD_IMM_W => {}
        LD_IMM_DW => {}
        LD_ABS_B => {}
        LD_ABS_H => {}
        LD_ABS_W => {}
        LD_ABS_DW => {}
        LD_IND_B => {}
        LD_IND_H => {}
        LD_IND_W => {}
        LD_IND_DW => {}
        LD_MEM_B => {}
        LD_MEM_H => {}
        LD_MEM_W => {}
        LD_MEM_DW => {}
        LD_XADD_B => {}
        LD_XADD_H => {}
        LD_XADD_W => {}
        LD_XADD_DW => {}

        LDX_IMM_B => {}
        LDX_IMM_H => {}
        LDX_IMM_W => {}
        LDX_IMM_DW => {}
        LDX_ABS_B => {}
        LDX_ABS_H => {}
        LDX_ABS_W => {}
        LDX_ABS_DW => {}
        LDX_IND_B => {}
        LDX_IND_H => {}
        LDX_IND_W => {}
        LDX_IND_DW => {}
        LDX_MEM_B => {}
        LDX_MEM_H => {}
        LDX_MEM_W => {}
        LDX_MEM_DW => {}
        LDX_XADD_B => {}
        LDX_XADD_H => {}
        LDX_XADD_W => {}
        LDX_XADD_DW => {}

        ST_IMM_B => {}
        ST_IMM_H => {}
        ST_IMM_W => {}
        ST_IMM_DW => {}
        ST_ABS_B => {}
        ST_ABS_H => {}
        ST_ABS_W => {}
        ST_ABS_DW => {}
        ST_IND_B => {}
        ST_IND_H => {}
        ST_IND_W => {}
        ST_IND_DW => {}
        ST_MEM_B => {}
        ST_MEM_H => {}
        ST_MEM_W => {}
        ST_MEM_DW => {}
        ST_XADD_B => {}
        ST_XADD_H => {}
        ST_XADD_W => {}
        ST_XADD_DW => {}

        STX_IMM_B => {}
        STX_IMM_H => {}
        STX_IMM_W => {}
        STX_IMM_DW => {}
        STX_ABS_B => {}
        STX_ABS_H => {}
        STX_ABS_W => {}
        STX_ABS_DW => {}
        STX_IND_B => {}
        STX_IND_H => {}
        STX_IND_W => {}
        STX_IND_DW => {}
        STX_MEM_B => {}
        STX_MEM_H => {}
        STX_MEM_W => {}
        STX_MEM_DW => {}
        STX_XADD_B => {}
        STX_XADD_H => {}
        STX_XADD_W => {}
        STX_XADD_DW => {}
        _ => {}
    }
}
