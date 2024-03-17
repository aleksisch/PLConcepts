use crate::instruction::{add_num, add_reg, call_inst, jmp, jmp_zeq, jmp_zneq, mov_num, mov_reg, pop, printf, push, ret, sub_num, sub_reg};
use crate::isa::Instructions;
use crate::program::Program;
use crate::registry::{Register, Registers};

pub struct VM {
    program: Program,
    regs: Registers,
}

impl VM {
    pub fn new(input_file: String) -> Self {
        let mut prog = Program::new(input_file);
        let (ip, sp) = prog.get_entrypoint();
        Self { program: prog, regs: Registers::new(ip as i32, sp as i32) }
    }

    pub fn next_inst(&mut self) -> bool {
        let code = self.program.read_shift(&mut self.regs.ip) as i32;
        let enum_code: Instructions = unsafe { ::std::mem::transmute(code) };

        match enum_code {
            Instructions::CALL => {
                let offset = self.program.read_i32_shift(&mut self.regs.ip);
                call_inst(&mut self.program, &mut self.regs.ip, &mut self.regs.sp, offset)
            },
            Instructions::PUSH => {
                let reg_id = self.program.read_u8_shift(&mut self.regs.ip);
                let reg = self.regs.get_reg(reg_id).load();
                push(&mut self.program, &mut self.regs.sp, reg);
            },
            Instructions::POP => {
                let reg_id = self.program.read_u8_shift(&mut self.regs.ip);
                pop(&mut self.program, &mut self.regs, reg_id);
            },
            Instructions::JMP => {
                let byte_reg = self.program.read_i32(&self.regs.ip);
                jmp(&mut self.regs.ip, byte_reg);
            }
            Instructions::JZE => {
                let reg_id = self.program.read_u8_shift(&mut self.regs.ip);
                let reg = self.regs.get_reg(reg_id).load();
                let byte_reg = self.program.read_i32_shift(&mut self.regs.ip);
                jmp_zeq(&mut self.regs.ip, reg, byte_reg);
            }
            Instructions::JZNE => {
                let reg_id = self.program.read_u8_shift(&mut self.regs.ip);
                let reg = self.regs.get_reg(reg_id).load();
                let byte_reg = self.program.read_i32_shift(&mut self.regs.ip);
                jmp_zneq(&mut self.regs.ip, reg, byte_reg);
            }
            Instructions::RET => {
                let byte_reg = self.program.pop(&mut self.regs.sp);
                ret(&mut self.regs.ip, byte_reg);
            }
            Instructions::AddNum => {
                let reg_id = self.program.read_u8_shift(&mut self.regs.ip);
                let val = self.program.read_i32_shift(&mut self.regs.ip);
                let reg = &mut self.regs.get_mut_reg(reg_id);
                add_num(reg, val)
            },
            Instructions::AddReg => {
                let dst = self.program.read_u8_shift(&mut self.regs.ip);
                let src = self.program.read_u8_shift(&mut self.regs.ip);
                add_reg(&mut self.regs, dst, src);
            },
            Instructions::SubNum => {
                let reg_id = self.program.read_u8_shift(&mut self.regs.ip);
                let val = self.program.read_i32_shift(&mut self.regs.ip);
                let reg = &mut self.regs.get_mut_reg(reg_id);
                sub_num(reg, val)
            },
            Instructions::SubReg => {
                let dst = self.program.read_u8_shift(&mut self.regs.ip);
                let src = self.program.read_u8_shift(&mut self.regs.ip);
                sub_reg(&mut self.regs, dst, src);
            },
            Instructions::MovReg => {
                let to = self.program.read_u8_shift(&mut self.regs.ip);
                let from = self.program.read_u8_shift(&mut self.regs.ip);
                mov_reg(&mut self.regs, to, from)
            },
            Instructions::MovNum => {
                let reg_id = self.program.read_u8_shift(&mut self.regs.ip);
                let val = self.program.read_i32_shift(&mut self.regs.ip);
                let reg = &mut self.regs.get_mut_reg(reg_id);
                mov_num(reg, val)
            },
            Instructions::Print => {
                printf(&mut self.program, &mut self.regs)
            }
            Instructions::END => {
                print!("{}", self.regs.ax.load());
                return false
            }
        };
        return true;
    }
}