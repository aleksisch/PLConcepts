use crate::program::Program;
use crate::registry::{I32Reg, IPReg, Register, Registers, SPReg};

pub fn jmp(ip: &mut IPReg, offset: i32) {
    ip.store(offset);
}

pub fn ret(ip: &mut IPReg, offset: i32) {
    ip.store(offset);
}

pub fn call_inst(prog: &mut Program, ip: &mut IPReg, sp: &mut SPReg, offset: i32) {
    let offset = prog.read_i32_shift(ip);
    prog.push(sp, ip.load());
    jmp(ip, offset);
}

pub fn add_reg(regs: &mut Registers, vals: u8) {
    let dst = regs.get_reg(vals & 0b11110000).load();
    let src = regs.get_mut_reg(vals & 0b00001111);
    src.store(dst + src.load());
}

pub fn add_num(src: &mut IPReg, dst: i32) {
    src.store(dst + src.load());
}

pub fn mov_reg(regs: &mut Registers, vals: u8) {
    let dst = regs.get_reg(vals & 0b11110000).load();
    let src = regs.get_mut_reg(vals & 0b00001111);
    src.store(dst);
}

pub fn mov_num(src: &mut IPReg, dst: i32) {
    src.store(dst);
}
