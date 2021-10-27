use crate::binding::bpf::*;

pub fn interpret(inst: u64) {
    let immediate: i32 = (inst >> 32) as i32;
    let offset: i16 = (inst >> 16) as i16;
    let src: u8 = ((inst >> 12) & 0x0f) as u8;
    let dst: u8 = ((inst >> 8) & 0x0f) as u8;
    let op: u32 = (inst & 0xff) as u32;
}
