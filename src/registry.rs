
pub trait Register<T> {
    fn load(&self) -> T;
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

    pub fn load_usize(&self) -> usize {
        self.val.unwrap() as usize
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
            0 => &mut self.ax,
            1 => &mut self.bx,
            2 => &mut self.cx,
            3 => &mut self.dx,
            4 => &mut self.sp,
            _ => unreachable!("incorrect argument for register")
        }
    }

    pub fn get_reg(&self, x: u8) -> & I32Reg {
        match x {
            0 => &self.ax,
            1 => &self.bx,
            2 => &self.cx,
            3 => &self.dx,
            4 => &self.sp,
            _ => unreachable!("incorrect argument for register")
        }
    }

    pub fn from_str(x: &str) -> u8 {
        match x {
            "ax" => 0,
            "bx" => 1,
            "cx" => 2,
            "dx" => 3,
            "sp" => 4,
            _ => unreachable!("incorrect register")
        }
    }
}

impl Register<i32> for I32Reg {
    fn load(&self) -> i32 {
        return self.val.unwrap();
    }

    fn store(&mut self, val: i32) {
        self.val = Some(val);
    }

    fn inc(&mut self) {
        self.store(self.load() + 1);
    }
}
