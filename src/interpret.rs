use crate::types::*;

const STACK_SIZE: usize = 512;

pub fn interpret(insts: &[u64]) -> u64 {
    let mut pc: u16 = 0;
    let mut reg: [u64; 16] = [0; 16];
    let stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    unsafe {
        reg[10] = stack.as_ptr().add(STACK_SIZE) as u64;
    }
    loop {
        let inst = insts[pc as usize];
        pc += 1;
        let imm: u64 = ((inst >> 32) & u32::MAX as u64) as u64;
        let off: u16 = ((inst >> 16) & u16::MAX as u64) as u16;
        let src: usize = ((inst >> 12) & 0x0f) as usize;
        let dst: usize = ((inst >> 8) & 0x0f) as usize;
        let op: u8 = (inst & u8::MAX as u64) as u8;
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

            JMP_K_JA => {
                pc = pc.wrapping_add(off);
            }
            JMP_K_JEQ => {
                if reg[dst] == imm {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_X_JEQ => {
                if reg[dst] == reg[src] {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_K_JGT => {
                if reg[dst] > imm {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_X_JGT => {
                if reg[dst] > reg[src] {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_K_JGE => {
                if reg[dst] >= imm {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_X_JGE => {
                if reg[dst] >= reg[src] {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_K_JSET => {
                if reg[dst] & imm != 0 {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_X_JSET => {
                if reg[dst] & reg[src] != 0 {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_K_JNE => {
                if reg[dst] != imm {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_X_JNE => {
                if reg[dst] != reg[src] {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_K_JSGT => {
                if reg[dst] as i64 > imm as i64 {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_X_JSGT => {
                if reg[dst] as i64 > reg[src] as i64 {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_K_JSGE => {
                if reg[dst] as i64 >= imm as i64 {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_X_JSGE => {
                if reg[dst] as i64 >= reg[src] as i64 {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_K_CALL => {
                // reg[0] = funcs[imm](reg[1], reg[2], reg[3], reg[4], reg[5]);
                // if (inst.imm == vm->unwind_stack_extension_index && reg[0] == 0) {
                //    ret = reg[0];
                //    return;
                // }
            }
            JMP_K_EXIT => {
                return reg[0];
            }
            JMP_K_JLT => {
                if reg[dst] < imm {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_X_JLT => {
                if reg[dst] < reg[src] {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_K_JLE => {
                if reg[dst] <= imm {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_X_JLE => {
                if reg[dst] <= reg[src] {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_K_JSLT => {
                if (reg[dst] as i64) < imm as i64 {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_X_JSLT => {
                if (reg[dst] as i64) < reg[src] as i64 {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_K_JSLE => {
                if reg[dst] as i64 <= imm as i64 {
                    pc = pc.wrapping_add(off);
                }
            }
            JMP_X_JSLE => {
                if reg[dst] as i64 <= reg[src] as i64 {
                    pc = pc.wrapping_add(off);
                }
            }
            LD_IMM_DW => {
                let next = insts[pc as usize];
                pc += 1;
                reg[dst] = imm + ((next >> 32) << 32);
            }
            /*
            TODO: non generic inst
            LD_ABS_B => {}
            LD_ABS_H => {}
            LD_ABS_W => {}
            LD_ABS_DW => {}
            LD_IND_B => {}
            LD_IND_H => {}
            LD_IND_W => {}
            LD_IND_DW => {}
            */
            LDX_MEM_B => unsafe {
                reg[dst] = *((reg[src] as *mut u8).offset(off as isize) as *mut u8) as u64;
            },
            LDX_MEM_H => unsafe {
                reg[dst] = *((reg[src] as *mut u8).offset(off as isize) as *mut u16) as u64;
            },
            LDX_MEM_W => unsafe {
                reg[dst] = *((reg[src] as *mut u8).offset(off as isize) as *mut u32) as u64;
            },
            LDX_MEM_DW => unsafe {
                reg[dst] = *((reg[src] as *mut u8).offset(off as isize) as *mut u64) as u64;
            },
            ST_MEM_B => unsafe {
                *((reg[dst] as *mut u8).offset(off as isize) as *mut u8) = imm as u8;
            },
            ST_MEM_H => unsafe {
                *((reg[dst] as *mut u8).offset(off as isize) as *mut u16) = imm as u16;
            },
            ST_MEM_W => unsafe {
                *((reg[dst] as *mut u8).offset(off as isize) as *mut u32) = imm as u32;
            },
            ST_MEM_DW => unsafe {
                *((reg[dst] as *mut u8).offset(off as isize) as *mut u64) = imm as u64;
            },
            STX_MEM_B => unsafe {
                *((reg[dst] as *mut u8).offset(off as isize) as *mut u8) = reg[src] as u8;
            },
            STX_MEM_H => unsafe {
                *((reg[dst] as *mut u8).offset(off as isize) as *mut u16) = reg[src] as u16;
            },
            STX_MEM_W => unsafe {
                *((reg[dst] as *mut u8).offset(off as isize) as *mut u32) = reg[src] as u32;
            },
            STX_MEM_DW => unsafe {
                *((reg[dst] as *mut u8).offset(off as isize) as *mut u64) = reg[src] as u64;
            },
            _ => {
                unimplemented!("{:x}", inst);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::interpret::interpret;
    #[test]
    fn gauss() {
        let prog = include_bytes!("tests/gauss.bin");
        let ret = interpret(
            &prog
                .chunks_exact(8)
                .map(|x| {
                    u64::from_le_bytes({
                        let mut buf: [u8; 8] = Default::default();
                        buf.copy_from_slice(x);
                        buf
                    })
                })
                .collect::<Vec<u64>>(),
        );
        assert_eq!(ret, 5050);
    }
}
