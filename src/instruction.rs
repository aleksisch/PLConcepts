use crate::program::Program;
use crate::registry::{I32Reg, IPReg, Register, Registers, SPReg};

pub fn jmp(ip: &mut IPReg, offset: i32) {
    ip.store(offset);
}

pub fn jmp_zeq(ip: &mut IPReg, cond: i32, offset: i32) {
    if cond == 0 {
        ip.store(offset);
    }
}

pub fn jmp_zneq(ip: &mut IPReg, cond: i32, offset: i32) {
    if cond != 0 {
        ip.store(offset);
    }
}

pub fn ret(ip: &mut IPReg, offset: i32) {
    ip.store(offset);
}

pub fn push(prog: &mut Program, sp: &mut SPReg, val: i32) {
    prog.push(sp, val);
}

pub fn pop(prog: &mut Program, regs: &mut Registers, reg_id: u8) {
    let val = prog.pop(&mut regs.sp);
    regs.get_mut_reg(reg_id).store(val);
}

pub fn call_inst(prog: &mut Program, ip: &mut IPReg, sp: &mut SPReg, offset: i32) {
    prog.push(sp, ip.load());
    jmp(ip, offset);
}

pub fn add_reg(regs: &mut Registers, to: u8, from: u8) {
    let src = regs.get_reg(from).load();
    let dst = regs.get_mut_reg(to);
    dst.store(dst.load() + src);
}

pub fn add_num(src: &mut IPReg, dst: i32) {
    src.store(dst + src.load());
}

pub fn sub_reg(regs: &mut Registers, to: u8, from: u8) {
    let dst = regs.get_reg(to).load();
    let src = regs.get_mut_reg(from);
    src.store(src.load() - dst);
}

pub fn sub_num(src: &mut IPReg, dst: i32) {
    src.store(src.load() - dst);
}

pub fn mov_reg(regs: &mut Registers, to: u8, from: u8) {
    let src = regs.get_reg(from).load();
    let dst = regs.get_mut_reg(to);
    dst.store(src);
}

pub fn mov_num(src: &mut IPReg, dst: i32) {
    src.store(dst);
}

fn print_string(prog: &mut Program, src: &I32Reg) {
    let mut i = 0;
    while prog.read_offset(src, i) != 0 {
        print!("{}", prog.read_offset(src, i) as char);
        i += 1;
    }
}

pub fn printf(prog: &mut Program, regs: &mut Registers) {
    let reg_id = prog.read_u8_shift(&mut regs.ip);
    let mut dst = prog.read_u8(regs.get_reg(reg_id));
    while dst != 0 {
        match std::char::from_u32(dst as u32).unwrap() {
            '%' => {
                regs.get_mut_reg(reg_id).inc();
                dst = prog.read_u8(regs.get_reg(reg_id));
                regs.ax.store(prog.pop(&mut regs.sp));
                match std::char::from_u32(dst as u32).unwrap() {
                    's' => print_string(prog, &regs.ax),
                    'i' => print!("{}", &regs.ax.load()),
                    'x' => print!("{:#x}", &regs.ax.load()),
                    _ => unreachable!()
                }
            }
            _ => print!("{}", std::char::from_u32(dst as u32).unwrap())
        }
        regs.get_mut_reg(reg_id).inc();
        dst = prog.read_u8(regs.get_reg(reg_id));
    }
}
