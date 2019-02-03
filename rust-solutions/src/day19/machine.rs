use std::result;
use std::option::NoneError;
use itertools::Itertools;
use std::str::FromStr;
use std::convert::TryFrom;
use std::thread;
use std::time;
use std::fmt::Display;
use std::fmt::Formatter;
use std::collections::HashMap;


pub type MResult<T> = result::Result<T, Box<NoneError>>;


pub struct Machine {
    registers: Vec<i128>,
    instructions: Vec<Instruction>,
    ip_binding: usize,
    ip: i128
}

impl Machine {
    pub fn new(instructions: &Vec<Instruction>, ip_id: usize, register_zero: i128) -> Machine {
        
        Machine { registers: vec![0, 0, 0, 0, 0, register_zero], instructions: instructions.to_vec(), ip: 0, ip_binding: ip_id }
    }
    
    pub fn run(&mut self) {
        let mut count = 0;
        let mut c = 0;
        let mut max = 0;
        let mut finishing_values : HashMap<i128, usize> = HashMap::new();
        
        
        loop {
            self.registers[self.ip_binding] = self.ip;
            if self.ip >= 0 && self.ip < self.instructions.len() as i128 {
                let i = &self.instructions[self.ip as usize];
                
                self.exec(i.get_id(), i.a(), i.b(), i.c());
//                thread::sleep(time::Duration::from_millis(3));
                self.ip = self.registers[self.ip_binding];
                self.ip += 1;
                count += 1;
                c+=1;

                if  self.ip == 28 {
                    let ii =  format!("{}", self.registers.iter().join(", "));
                    let result = self.registers[4];
//                    println!("{:15},  {:10}    {:20}", result, c, ii);

                    if let Some(_) = finishing_values.get(&result) {
                        println!("{} is dupe", result);
                        let max = finishing_values.values().max().unwrap_or_else(|| panic!("no x "));
                        

                        for (k, v) in finishing_values.iter().filter(| (_, &v)| v == *max ) {
                            println!("{:15} {:10}", k, v);

                        }
                        println!("{:15},  {:10}    {:20}", result, c, ii);
                        break;
                    }
                    finishing_values.insert(result, c);
                    

                    if c >=  max {
                        max = c;
                        //println!("{:20}   [{}]", ii, self.registers.iter().join(", "));
                    }
                    //c = 0;

                }
               
            } else {
                println!("terminated");
                break;
            }
        }
    }

    pub fn addr(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        *self.registers.get_mut(c as usize)? = self.registers.get(a as usize)?.saturating_add(*self.registers.get(b as usize)?);
        Ok(self.registers.clone())
    }

    pub fn addi(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        *self.registers.get_mut(c as usize)? = self.registers.get(a as usize)?.saturating_add(b);
        Ok(self.registers.clone())
    }

    pub fn mulr(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        *self.registers.get_mut(c as usize)? = self.registers.get(a as usize)?.saturating_mul(*self.registers.get(b as usize)?);
        Ok(self.registers.clone())
    }

    pub fn muli(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        *self.registers.get_mut(c as usize)? = self.registers.get(a as usize)?.saturating_mul(b);
        Ok(self.registers.clone())
    }

    pub fn banr(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        *self.registers.get_mut(c as usize)? = *self.registers.get(a as usize)? & *self.registers.get(b as usize)?;
        Ok(self.registers.clone())
    }

    pub fn bani(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        *self.registers.get_mut(c as usize)? = *self.registers.get(a as usize)? & b;
        Ok(self.registers.clone())
    }

    pub fn borr(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        *self.registers.get_mut(c as usize)? = *self.registers.get(a as usize)? | *self.registers.get(b as usize)?;
        Ok(self.registers.clone())
    }

    pub fn bori(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        *self.registers.get_mut(c as usize)? = *self.registers.get(a as usize)? | b;
        Ok(self.registers.clone())
    }

    pub fn setr(&mut self, a: i128, _b: i128, c: i128) -> MResult<Vec<i128>> {
        *self.registers.get_mut(c as usize)? = *self.registers.get(a as usize)?;
        Ok(self.registers.clone())
    }

    pub fn seti(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        *self.registers.get_mut(c as usize)? = a;
        Ok(self.registers.clone())
    }

    pub fn gtir(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        let c_value = match a > *self.registers.get(b as usize)? {
            true => 1,
            _ => 0
        };
        *self.registers.get_mut(c as usize)? = c_value;
        Ok(self.registers.clone())
    }

    pub fn gtri(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        let c_value = match *self.registers.get(a as usize)? > b {
            true => 1,
            _ => 0
        };
        *self.registers.get_mut(c as usize)? = c_value;
        Ok(self.registers.clone())
    }

    pub fn gtrr(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        let c_value = match self.registers.get(a as usize)? > self.registers.get(b as usize)? {
            true => 1,
            _ => 0
        };
        *self.registers.get_mut(c as usize)? = c_value;
        Ok(self.registers.clone())
    }

    pub fn eqir(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        let c_value = match a == *self.registers.get(b as usize)? {
            true => 1,
            _ => 0
        };
        *self.registers.get_mut(c as usize)? = c_value;
        Ok(self.registers.clone())
    }

    pub fn eqri(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        let c_value = match *self.registers.get(a as usize)? == b {
            true => 1,
            _ => 0
        };
        *self.registers.get_mut(c as usize)? = c_value;
        Ok(self.registers.clone())
    }

    pub fn eqrr(&mut self, a: i128, b: i128, c: i128) -> MResult<Vec<i128>> {
        let c_value = match *self.registers.get(a as usize)? == *self.registers.get(b as usize)? {
            true => 1,
            _ => 0
        };
        *self.registers.get_mut(c as usize)? = c_value;
        Ok(self.registers.clone())
    }
    
    pub fn reset(&mut self, registers: Vec<i128>) {
        self.registers = registers;
    }
    
    pub fn exec(&mut self, instruction_id: i128, a: i128, b: i128, c : i128) -> MResult<Vec<i128>> {
        match instruction_id {
            6 => self.muli(a, b, c),
            2 => self.borr(a, b, c),
            12 => self.bori(a, b, c),
            0 => self.addi(a, b, c),
            15 => self.mulr(a, b, c),
            5 => self.seti(a, b, c),
            14 => self.eqir(a, b, c),
            13 => self.eqri(a, b, c),
            3  => self.gtri(a, b, c),
            1 => self.eqrr(a, b, c),
            9 => self.gtrr(a, b, c),
            11 => self.gtir(a, b, c),
            10  => self.setr(a, b, c),
            7 => self.bani(a, b, c),
            8 => self.banr(a, b, c),
            4 => self.addr(a, b, c),
            _ => panic!("nope")
        }
        
 
    }

    pub fn get_reg0(&self) -> i128 {
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

#[derive(Clone)]
pub struct Instruction {
    instruction: Vec<i128>,
    name: String,
}

impl Instruction {
 

    pub fn with_name(name: &str, instruction: Vec<i128>) -> Instruction {
        let id = match name {
            "muli" => 6,
            "borr" => 2,
            "bori" => 12,
            "addi" => 0,
            "mulr" => 15,
            "seti" => 5,
            "eqir" => 14,
            "eqri" => 13,
            "gtri" => 3 ,
            "eqrr" => 1,
            "gtrr" => 9,
            "gtir" => 11,
            "setr" => 10 ,
            "bani" => 7,
            "banr" => 8,
            "addr" => 4,
            _ => panic!("Unkown instructon name")
        };
        
        let mut inst = vec![id];
        inst.extend(instruction.clone().iter());

        Instruction { name: name.to_string(), instruction: inst }
    }


    pub fn get_id(&self) -> i128 {
        self.instruction[0]
    }

    pub fn a(&self) -> i128 {
        self.instruction[1]
    }

    pub fn b(&self) -> i128 {
        self.instruction[2]
    }

    pub fn c(&self) -> i128 {
        self.instruction[3]
    }
}

impl FromStr for Instruction {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let (name, args) = s.split_at(4);
        
        let instruction = args.trim().split(" ").map(|n| n.parse::<i128>().unwrap_or_else(|_| panic!("oops"))).collect_vec(); 
        Ok(Instruction::with_name(name, instruction))
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {} {}", self.name, self.instruction[1], self.instruction[2], self.instruction[3])
    }
}

