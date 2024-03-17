#[repr(i32)]
pub enum Instructions {
    CALL,
    JMP,
    JZE,
    JZNE,
    RET,
    END,
    AddNum,
    AddReg,
    SubNum,
    SubReg,
    MovNum,
    MovReg,
    Print,
}