use std::fs::File;
use crate::instruction::{add_num, add_reg, call_inst, jmp, mov_reg, ret};
use crate::program::Program;
use crate::registry::{I32Reg, Register, Registers};

pub struct VM {
    program: Program,
    regs: Registers,
}

impl VM {
    pub fn new(input_file: String, entrypoint: i32, stack: i32) -> Self {
        Self { program: Program::new(input_file), regs: Registers::new(entrypoint, stack) }
    }

    pub fn next_inst(&mut self) -> bool {
        let code = &self.program.read_shift(&mut self.regs.ip);
        if *code == 0 {
            return true;
        }
        match code {
            0 => {
                let offset = self.program.read_i32_shift(&mut self.regs.ip);
                call_inst(&mut self.program, &mut self.regs.ip, &mut self.regs.sp, offset)
            },
            1 => {
                let byte_reg = self.program.read_i32(&self.regs.ip);
                jmp(&mut self.regs.ip, byte_reg);
            }
            2 => {
                let byte_reg = self.program.pop(&mut self.regs.sp);
                ret(&mut self.regs.ip, byte_reg);
            }
            3 => {
                let ip = &mut self.regs.ip;
                add_num(ip, self.program.read_i32(ip))
            },
            4 => {
                let byte_reg = self.program.read(&self.regs.ip);
                add_reg(&mut self.regs, byte_reg as u8);
            },
            5 => {
                let byte_reg = self.program.read(&self.regs.ip);
                mov_reg(&mut self.regs, byte_reg as u8)
            },
            _ => {}
        };
        return false;
    }
}