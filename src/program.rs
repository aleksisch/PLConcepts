use std::fs::File;
use std::io::Read;
use crate::registry::{I32Reg, Register, SPReg};
use std::vec;

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

    pub fn read(&self, reg: &I32Reg) -> i8 {
        self.data[reg.load() as u32]
    }

    pub fn read_i32(&self, reg: &I32Reg) -> i32 {
        let mut res = self.read(reg) as i32;
        for i in 1..4 {
            res <<= 8;
            res += self.data[reg.load() + i];
        }
        res
    }

    pub fn read_i32_shift(&self, reg: &mut I32Reg) -> i32 {
        let res = self.read_i32(&reg);
        reg.store(reg.load() + 4);
        res
    }

    pub fn read_shift(&self, reg: &mut I32Reg) -> i8 {
        let res = self.read(reg);
        reg.inc();
        res
    }

    pub fn write(&mut self, reg: &I32Reg, val: i8) {
        self.data[reg.load()] = val;
    }

    pub fn write_shift(&mut self, reg: &mut I32Reg, val: i8) {
        self.write(reg, val);
        reg.inc();
    }


    pub fn write_i32(&mut self, reg: &I32Reg, val: i32) {
        self.write(reg, (val >> 24) as i8);
        for i in 1..4 {
            self.data[reg.load() + i] = (val >> ((3 - i) * 8)) & 0xFF;
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
        self.read_i32_shift(sp)
    }
}