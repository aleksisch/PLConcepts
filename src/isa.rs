#[repr(i32)]
pub enum Instructions {
    CALL,
    PUSH,
    POP,
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