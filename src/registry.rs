
pub trait Register<T> {
    fn load(&self) -> T;
    fn load_u32(&self) -> T;
    fn store(&mut self, val: T);
    fn inc(&mut self);
}

#[derive(Default)]
pub struct I32Reg {
    val: Option<i32>,
}

pub type SPReg = I32Reg;
pub type IPReg = I32Reg;

pub struct Registers {
    pub ip: IPReg,
    pub sp: SPReg,
    pub ax: I32Reg,
    pub bx: I32Reg,
    pub cx: I32Reg,
    pub dx: I32Reg,
}

impl I32Reg {
    pub fn new(val: Option<i32>) -> I32Reg {
        Self { val }
    }
}

impl Registers {
    pub fn new(ip: i32, sp: i32) ->Registers {
        Self {
            ip: I32Reg::new(Some(ip)),
            sp: I32Reg::new(Some(sp)),
            ax: I32Reg::new(None),
            bx: I32Reg::new(None),
            cx: I32Reg::new(None),
            dx: I32Reg::new(None),
        }
    }

    pub fn get_mut_reg(&mut self, x: u8) -> & mut I32Reg {
        match x {
            0 => &mut self.ip,
            1 => &mut self.sp,
            2 => &mut self.ax,
            3 => &mut self.bx,
            4 => &mut self.cx,
            5 => &mut self.dx,
            _ => unreachable!("incorrect argument for register")
        }
    }

    pub fn get_reg(&self, x: u8) -> & I32Reg {
        match x {
            0 => &self.ax,
            1 => &self.bx,
            2 => &self.cx,
            3 => &self.dx,
            _ => unreachable!("incorrect argument for register")
        }
    }
}

impl Register<i32> for I32Reg {
    fn load(&self) -> i32 {
        return self.val.unwrap();
    }

    fn load_u32(&self) -> u32 {
        return self.val.unwrap() as u32;
    }

    fn store(&mut self, val: i32) {
        self.val = Some(val);
    }

    fn inc(&mut self) {
        self.store(self.load() + 1);
    }
}
