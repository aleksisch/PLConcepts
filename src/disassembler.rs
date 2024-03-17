use byteorder::{BigEndian, ReadBytesExt}; // 1.2.7
use std::fs;
use crate::instruction::{add_num, add_reg, call_inst, jmp, jmp_zeq, jmp_zneq, mov_num, mov_reg, pop, printf, push, ret, sub_num, sub_reg};
use crate::isa::Instructions;
use crate::registry::Registers;

fn convert_u32(data: &[u8], start: &mut usize) -> u32 {
    let res = u32::from_be_bytes(data[*start..*start + 4].try_into().expect("slice with incorrect length"));
    *start += 4;
    res
}

fn convert_u8(data: &[u8], start: &mut usize) -> u8 {
    let res = u8::from_be_bytes(data[*start..*start + 1].try_into().expect("slice with incorrect length"));
    *start += 1;
    res
}

pub fn disassembly(input_file: String) {
    let data = fs::read_to_string(&input_file).expect("Failed to read file");
    let mut bytes = data.as_bytes();
    let mut tmp = 0;
    let mut num = convert_u32(&bytes, &mut tmp) as usize;
    loop {
        let cmd = convert_u8(bytes, &mut num);
        let enum_code: Instructions = unsafe { ::std::mem::transmute(cmd as i32) };

        match enum_code {
            Instructions::CALL => {
                let num = convert_u32(bytes, &mut num);
                println!("CALL {}", num);
            },
            Instructions::PUSH => {
                let reg_id = convert_u8(bytes, &mut num);
                let reg = Registers::id_to_str(reg_id);
                println!("PUSH {}", reg);
            },
            Instructions::POP => {
                let reg_id = convert_u8(bytes, &mut num);
                let reg = Registers::id_to_str(reg_id);
                println!("POP {}", reg);
            },
            Instructions::JMP => {
                let num = convert_u32(bytes, &mut num);
                println!("JMP {}", num);
            }
            Instructions::JZE => {
                let reg_id = convert_u8(bytes, &mut num);
                let reg = Registers::id_to_str(reg_id);
                let num = convert_u32(bytes, &mut num);
                println!("JZE {} {}", reg, num);
            }
            Instructions::JZNE => {
                let reg_id = convert_u8(bytes, &mut num);
                let reg = Registers::id_to_str(reg_id);
                let num = convert_u32(bytes, &mut num);
                println!("JZNE {} {}", reg, num);
            }
            Instructions::RET => {
                println!("RET");
            }
            Instructions::AddNum => {
                let reg_id = convert_u8(bytes, &mut num);
                let reg = Registers::id_to_str(reg_id);
                let num = convert_u32(bytes, &mut num);
                println!("ADDN {} {}", reg, num);
            },
            Instructions::AddReg => {
                let reg_id = convert_u8(bytes, &mut num);
                let reg = Registers::id_to_str(reg_id);
                let reg_id2 = convert_u8(bytes, &mut num);
                let reg2 = Registers::id_to_str(reg_id2);
                println!("ADD {} {}", reg, reg2);
            },
            Instructions::SubNum => {
                let reg_id = convert_u8(bytes, &mut num);
                let reg = Registers::id_to_str(reg_id);
                let num = convert_u32(bytes, &mut num);
                println!("SUBN {} {}", reg, num);
            },
            Instructions::SubReg => {
                let reg_id = convert_u8(bytes, &mut num);
                let reg = Registers::id_to_str(reg_id);
                let reg_id2 = convert_u8(bytes, &mut num);
                let reg2 = Registers::id_to_str(reg_id2);
                println!("SUB {} {}", reg, reg2);
            },
            Instructions::MovReg => {
                let reg_id = convert_u8(bytes, &mut num);
                let reg = Registers::id_to_str(reg_id);
                let reg_id2 = convert_u8(bytes, &mut num);
                let reg2 = Registers::id_to_str(reg_id2);
                println!("MOV {} {}", reg, reg2);
            },
            Instructions::MovNum => {
                let reg_id = convert_u8(bytes, &mut num);
                let reg = Registers::id_to_str(reg_id);
                let num = convert_u32(bytes, &mut num);
                println!("MOVN {} {}", reg, num);
            },
            Instructions::Print => {
                println!("println");
            },
            Instructions::END => {
                println!("END");
                break;
            }
        }
    }
}
