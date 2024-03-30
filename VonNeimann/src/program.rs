use std::fs::File;
use std::io::Read;
use crate::registry::{I32Reg, Register, SPReg};

pub struct Program {
    data: Vec<u8>, // We can use mmap or do smth else, to avoid allocations but using vec faster and more convenient
}

impl Program {
    pub fn read_file_vec(read_path: &str) -> Vec<u8> {
        let mut file = File::open(read_path).unwrap();
        let mut file_contents = Vec::<u8>::new();
        file.read_to_end(&mut file_contents).expect("no file found");
        file_contents
    }

    pub fn new(input_file: String) -> Program {
        Self { data: Self::read_file_vec(&input_file)}
    }

    fn read_offset_u32(&self, offset: usize) -> u32 {
        let mut res = self.data[offset] as u32;
        for i in 1..4 {
            res <<= 8;
            res += self.data[offset + i] as u32;
        }
        res
    }

    pub fn get_entrypoint(&self) -> (u32, u32) {
        (self.read_offset_u32(0), self.read_offset_u32(4))
    }

    pub fn read_offset(&self, reg: &I32Reg, offset: i32) -> u8 {
        self.data[(reg.load() + offset) as usize]
    }

    pub fn write_offset(&mut self, reg: &I32Reg, offset: i32, val: u8) {
        self.data[(reg.load() + offset) as usize] = val;
    }

    pub fn read_i32(&self, reg: &I32Reg) -> i32 {
        let mut res = self.read_offset(reg, 0) as i32;
        for i in 1..4 {
            res <<= 8;
            res += self.read_offset(reg, i) as i32;
        }
        res
    }

    pub fn read_u8(&self, reg: &I32Reg) -> u8 {
        self.read_offset(reg, 0)
    }

    pub fn read_u8_shift(&self, reg: &mut I32Reg) -> u8 {
        let res = self.read_offset(reg, 0);
        reg.inc();
        res
    }

    pub fn read_i32_shift(&self, reg: &mut I32Reg) -> i32 {
        let res = self.read_i32(&reg);
        reg.store(reg.load() + 4);
        res
    }

    pub fn read_shift(&self, reg: &mut I32Reg) -> i8 {
        let res = self.read_offset(reg, 0);
        reg.inc();
        res as i8
    }

    pub fn write(&mut self, reg: &I32Reg, val: i8) {
        self.write_offset(reg, 0, val as u8);
    }

    pub fn write_shift(&mut self, reg: &mut I32Reg, val: i8) {
        self.write(reg, val);
        reg.inc();
    }


    pub fn write_i32(&mut self, reg: &I32Reg, val: i32) {
        self.write(reg, (val >> 24) as i8);
        for i in 1..4 {
            self.write_offset(reg, i, ((val >> ((3 - i) * 8)) & 0xFF) as u8);
        }
    }

    pub fn write_i32_shift(&mut self, reg: &mut I32Reg, val: i32) {
        self.write_i32(&reg, val);
        reg.store(reg.load() + 4);
    }


    pub fn push(&mut self, sp: &mut SPReg, val: i32) {
        self.write_i32_shift(sp, val);
    }

    pub fn pop(&mut self, sp: &mut SPReg) -> i32 {
        sp.store(sp.load() - 4);
        self.read_i32(sp)
    }
}