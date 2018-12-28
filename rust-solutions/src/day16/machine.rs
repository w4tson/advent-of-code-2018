use std::result;
use std::option::NoneError;

pub type Result<T> = result::Result<T, Box<NoneError>>;


pub struct Machine {
    registers: Vec<i32>
}

impl Machine {
    pub fn new(registers: Vec<i32>) -> Machine {
        Machine { registers }
    }

    pub fn addr(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        *self.registers.get_mut(c as usize)? = self.registers.get(a as usize)? + self.registers.get(b as usize)?;
        Ok(self.registers.clone())
    }

    pub fn addi(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        *self.registers.get_mut(c as usize)? = *self.registers.get(a as usize)? + b;
        Ok(self.registers.clone())
    }

    pub fn mulr(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        *self.registers.get_mut(c as usize)? = *self.registers.get(a as usize)? * *self.registers.get(b as usize)?;
        Ok(self.registers.clone())
    }

    pub fn muli(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        *self.registers.get_mut(c as usize)? = *self.registers.get(a as usize)? * b;
        Ok(self.registers.clone())
    }

    pub fn banr(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        *self.registers.get_mut(c as usize)? = *self.registers.get(a as usize)? & *self.registers.get(b as usize)?;
        Ok(self.registers.clone())
    }

    pub fn bani(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        *self.registers.get_mut(c as usize)? = *self.registers.get(a as usize)? & b;
        Ok(self.registers.clone())
    }

    pub fn borr(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        *self.registers.get_mut(c as usize)? = *self.registers.get(a as usize)? | *self.registers.get(b as usize)?;
        Ok(self.registers.clone())
    }

    pub fn bori(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        *self.registers.get_mut(c as usize)? = *self.registers.get(a as usize)? | b;
        Ok(self.registers.clone())
    }

    pub fn setr(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        *self.registers.get_mut(c as usize)? = *self.registers.get(a as usize)?;
        Ok(self.registers.clone())
    }

    pub fn seti(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        *self.registers.get_mut(c as usize)? = a;
        Ok(self.registers.clone())
    }

    pub fn gtir(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        let c_value = match a > *self.registers.get(b as usize)? {
            true => 1,
            _ => 0
        };
        *self.registers.get_mut(c as usize)? = c_value;
        Ok(self.registers.clone())
    }

    pub fn gtri(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        let c_value = match *self.registers.get(a as usize)? > b {
            true => 1,
            _ => 0
        };
        *self.registers.get_mut(c as usize)? = c_value;
        Ok(self.registers.clone())
    }

    pub fn gtrr(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        let c_value = match self.registers.get(a as usize)? > self.registers.get(b as usize)? {
            true => 1,
            _ => 0
        };
        *self.registers.get_mut(c as usize)? = c_value;
        Ok(self.registers.clone())
    }

    pub fn eqir(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        let c_value = match a == *self.registers.get(b as usize)? {
            true => 1,
            _ => 0
        };
        *self.registers.get_mut(c as usize)? = c_value;
        Ok(self.registers.clone())
    }

    pub fn eqri(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        let c_value = match *self.registers.get(a as usize)? == b {
            true => 1,
            _ => 0
        };
        *self.registers.get_mut(c as usize)? = c_value;
        Ok(self.registers.clone())
    }

    pub fn eqrr(&mut self, a: i32, b: i32, c: i32) -> Result<Vec<i32>> {
        let c_value = match *self.registers.get(a as usize)? == *self.registers.get(b as usize)? {
            true => 1,
            _ => 0
        };
        *self.registers.get_mut(c as usize)? = c_value;
        Ok(self.registers.clone())
    }
    
    pub fn reset(&mut self, registers: Vec<i32>) {
        self.registers = registers;
    }
    
    pub fn exec(&mut self, instruction: Instruction) -> Result<Vec<i32>> {
        match instruction.get_id() {
            6 => self.muli(instruction.a(), instruction.b(), instruction.c()),
            2 => self.borr(instruction.a(), instruction.b(), instruction.c()),
            12 => self.bori(instruction.a(), instruction.b(), instruction.c()),
            0 => self.addi(instruction.a(), instruction.b(), instruction.c()),
            15 => self.mulr(instruction.a(), instruction.b(), instruction.c()),
            5 => self.seti(instruction.a(), instruction.b(), instruction.c()),
            14 => self.eqir(instruction.a(), instruction.b(), instruction.c()),
            13 => self.eqri(instruction.a(), instruction.b(), instruction.c()),
            3  => self.gtri(instruction.a(), instruction.b(), instruction.c()),
            1 => self.eqrr(instruction.a(), instruction.b(), instruction.c()),
            9 => self.gtrr(instruction.a(), instruction.b(), instruction.c()),
            11 => self.gtir(instruction.a(), instruction.b(), instruction.c()),
            10  => self.setr(instruction.a(), instruction.b(), instruction.c()),
            7 => self.bani(instruction.a(), instruction.b(), instruction.c()),
            8 => self.banr(instruction.a(), instruction.b(), instruction.c()),
            4 => self.addr(instruction.a(), instruction.b(), instruction.c()),
            _ => panic!("nope")
        }
        
 
    }

    pub fn get_reg0(&self) -> i32 {
        *self.registers.get(0).unwrap()
    }
//    0 addr
//    1 addi
//    2 mulr
//    3 muli
//    4 banr
//    5 bani
//    6 borr
//    7 bori
//    8 setr
//    9 seti
//    10 gtir
//    11 gtri
//    12 gtrr
//    13 eqir
//    14 eqri
//    15 eqrr
}

pub struct Instruction {
    instruction: Vec<i32>
}

impl Instruction {
    pub fn new(instruction: Vec<i32>) -> Instruction {
        Instruction { instruction }
    }

    pub fn get_id(&self) -> i32 {
        self.instruction[0]
    }

    pub fn a(&self) -> i32 {
        self.instruction[1]
    }

    pub fn b(&self) -> i32 {
        self.instruction[2]
    }

    pub fn c(&self) -> i32 {
        self.instruction[3]
    }
}
