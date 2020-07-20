use std::collections::{HashMap};
use regex::Regex;

#[derive(Eq, PartialEq,Debug,Clone,Copy,Ord, PartialOrd)]
pub enum OpCode {
    ADDR,
    ADDI,
    MULTR,
    MULTI,
    SETR,
    SETI,
    BANR,
    BANI,
    BORR,
    BORI,
    GTIR,
    GTRI,
    GTRR,
    EQIR,
    EQRI,
    EQRR,
}

impl OpCode {
    fn is_immediate(&self) -> (bool,bool) {
        match self {
            OpCode::ADDI => (false,true),
            OpCode::MULTI => (false,true),
            OpCode::SETI => (true,true) ,
            OpCode::BORI => (false,true),
            OpCode::BANI => (false,true),
            OpCode::GTIR => (true,false),
            OpCode::GTRI => (false,true),
            OpCode::EQIR => (true,false),
            OpCode::EQRI => (false,true),

            _ => (false,false)
        }
    }

    fn values() -> Vec<OpCode> {
        vec![
            OpCode::ADDR,
            OpCode::ADDI,
            OpCode::MULTR,
            OpCode::MULTI,
            OpCode::SETR,
            OpCode::SETI,
            OpCode::BANR,
            OpCode::BANI,
            OpCode::BORR,
            OpCode::BORI,
            OpCode::GTIR,
            OpCode::GTRI,
            OpCode::GTRR,
            OpCode::EQIR,
            OpCode::EQRI,
            OpCode::EQRR,
        ]
    }

    fn get_op(&self) -> Box<dyn Op> {
        let v = self.clone();
        let op:Box<dyn Op> = match v {
            OpCode::ADDI => Box::new(Add{}),
            OpCode::ADDR => Box::new(Add{}),

            OpCode::MULTI => Box::new(Mult{}),
            OpCode::MULTR => Box::new(Mult{}),

            OpCode::SETR => Box::new(Set{}),
            OpCode::SETI => Box::new(Set {}),

            OpCode::BORR => Box::new(Bor {}),
            OpCode::BORI => Box::new(Bor {}),

            OpCode::BANR => Box::new(Ban {}),
            OpCode::BANI => Box::new(Ban {}),

            OpCode::GTRI => Box::new(GreaterThan {}),
            OpCode::GTIR => Box::new(GreaterThan {}),
            OpCode::GTRR => Box::new(GreaterThan {}),

            OpCode::EQRI => Box::new(Equal {}),
            OpCode::EQIR => Box::new(Equal {}),
            OpCode::EQRR => Box::new(Equal {}),

        };
        op
    }
}

trait Op {
    fn execute(&self, a:i64,b:i64) -> i64;
}

struct Add {}
struct Mult {}
struct Bor {}
struct Ban {}
struct Set {}
struct Equal {}
struct GreaterThan {}

impl Op for Add {
    fn execute(&self, a:i64,b:i64) -> i64 {
        a + b
    }
}

impl Op for Set {
    fn execute(&self, a:i64,_:i64) -> i64 {
        a
    }
}

impl Op for Mult {
    fn execute(&self, a:i64,b:i64) -> i64 {
        a * b
    }
}

impl Op for Ban {
    fn execute(&self, a:i64,b:i64) -> i64 {
        a & b
    }
}

impl Op for Bor {
    fn execute(&self, a:i64,b:i64) -> i64 {
        a | b
    }
}

impl Op for Equal {
    fn execute(&self, a:i64,b:i64) -> i64 {
        match a == b {
            true => 1,
            false => 0,
        }
    }
}

impl Op for GreaterThan {
    fn execute(&self, a:i64,b:i64) -> i64 {
        match a > b {
            true => 1,
            false => 0,
        }
    }
}

pub struct Computer {
    registers:Vec<i64>,
    map_codes:HashMap<usize,OpCode>,
}

impl Computer {
    pub fn new() -> Computer {
        Computer{registers:vec![0,0,0,0,0,0],map_codes:HashMap::new()}
    }

    pub fn run_program_with_pc(&mut self, pc_reg_no:usize, program:Vec<(OpCode,usize,usize,usize)>) {
        let mut pc = 0;

        loop {
            // Load instruction pointer to register linked to it
            self.registers[pc_reg_no] = pc as i64;

            let instr = program[pc];
            self.run_instruction(instr);

            // Fetch PC from reg
            pc = self.registers[pc_reg_no] as usize + 1;

            // Check out of bounds
            if pc >= program.len() {
                println!("End program, reg {} has value:{}",pc_reg_no,self.registers[pc_reg_no]);
                break;
            }
        }
    }

    pub fn run_instruction(&mut self, instr:(OpCode,usize,usize,usize)) {
        let (op_code,a_in,b_in,c_in) = instr;
        let (a_immediate,b_immediate) = op_code.is_immediate();

        let a  = if a_immediate {
            a_in as i64
        } else {
            self.registers[a_in]
        };

        let b  = if b_immediate {
            b_in as i64
        } else {
            self.registers[b_in]
        };

        let op = op_code.get_op();
        let res = op.execute(a,b);
        //println!("op={:?}, res={},a={},b={}, to register={}",op_code,res,a,b,c_in);
        self.registers[c_in] = res;
    }

    pub fn run_program(&mut self, program:Vec<Vec<usize>>) {
        for instr in program {
            let opcode = *self.map_codes.get(&instr[0]).unwrap();
            self.run_instruction((opcode, instr[1],instr[2],instr[3]));
        }
    }

    pub fn set_registers(&mut self,a:i64,b:i64,c:i64,d:i64) {
        self.registers[0] = a;
        self.registers[1] = b;
        self.registers[2] = c;
        self.registers[3] = d;
    }

    pub fn test_opcodes(&mut self, instr:&Vec<usize>,state:&Vec<i64>, out_state:&Vec<i64>,) -> Vec<OpCode> {
        let mut result = vec![];
        for opcode in OpCode::values() {
            // Set state
            self.set_registers(state[0],state[1],state[2],state[3]);

            // Build instruction
            let instr = (opcode.clone(), instr[1],instr[2],instr[3]);

            // Run instr
            self.run_instruction(instr);

            if self.registers[0..4].eq(&out_state[0..4]) {
                result.push(opcode)
            }

        }
        // Set register
        result.sort();
        result
    }


    pub fn parse_input_file(&mut self, input:&str) -> (Vec<(usize, Vec<OpCode>)>, Vec<Vec<usize>>) {
        let regex0 = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]$").unwrap();
        let regex1 = Regex::new(r"(\d+) (\d+) (\d+) (\d+)$").unwrap();
        let regex2 = Regex::new(r"After:  \[(\d+), (\d+), (\d+), (\d+)\]$").unwrap();

        let lines:Vec<&str> = input.lines().map(|l|l.trim()).filter(|p|!p.is_empty()).collect();
        let mut in_state:Vec<i64> = vec![0,0,0,0];
        let mut out_state:Vec<i64> = vec![0,0,0,0];
        let mut instr:Vec<usize> = vec![0,0,0,0];
        let mut result = vec![];
        let mut program = vec![];

        let mut n = 0;
        for i in 0..lines.len() {
            let line = lines[i];

            if n == 0 && line.contains("Before") {
                let caps = regex0.captures(line).unwrap();
                in_state[0] = caps.get(1).unwrap().as_str().parse().ok().unwrap();
                in_state[1] = caps.get(2).unwrap().as_str().parse().ok().unwrap();
                in_state[2] = caps.get(3).unwrap().as_str().parse().ok().unwrap();
                in_state[3] = caps.get(4).unwrap().as_str().parse().ok().unwrap();
                n+=1;
            } else if n == 1 {
                let caps = regex1.captures(line).unwrap();
                instr[0] = caps.get(1).unwrap().as_str().parse().ok().unwrap();
                instr[1] = caps.get(2).unwrap().as_str().parse().ok().unwrap();
                instr[2] = caps.get(3).unwrap().as_str().parse().ok().unwrap();
                instr[3] = caps.get(4).unwrap().as_str().parse().ok().unwrap();
                n+=1;
            } else if n == 2 {
                let caps = regex2.captures(line).unwrap();
                out_state[0] = caps.get(1).unwrap().as_str().parse().ok().unwrap();
                out_state[1] = caps.get(2).unwrap().as_str().parse().ok().unwrap();
                out_state[2] = caps.get(3).unwrap().as_str().parse().ok().unwrap();
                out_state[3] = caps.get(4).unwrap().as_str().parse().ok().unwrap();

                let res = self.test_opcodes(&instr, &in_state, &out_state);
                //println!("res={:?}",res);
                result.push((instr[0],res));
                n = 0;
            } else if n == 0 {
                // Program starts here
                // 10 0 1 0
                let program_line:Vec<usize> = line.split_whitespace()
                    .map(|c| c.parse().ok().unwrap()).collect();
                program.push(program_line);
            }

        }

        (result,program)
    }

    pub fn set_register_value(&mut self,index:usize,value:i64) {
        self.registers[index] = value;
    }

    pub fn get_register_value(&self,index:usize) -> i64 {
        self.registers[index]
    }

    pub fn get_registers(&self) -> &Vec<i64> {
        self.registers.as_ref()
    }

    pub fn solve_opcodes(&mut self, mut mappings:Vec<(usize, Vec<OpCode>)>) {
        let mut matched_instr = HashMap::new();

        while matched_instr.len() < 16 {
            // Filter out the ones
            mappings.iter().filter(|(_,l)|l.len() == 1)
                .for_each(|(op,list)| {
                    matched_instr.insert(*op, *list.first().unwrap());
                });

            for i in 0..mappings.len() {
                let (op,item) = mappings.get_mut(i).unwrap();

                for (matched_op, instr_list) in matched_instr.iter() {
                    if *matched_op == *op {
                        continue;
                    }

                    while item.contains(instr_list) {
                        for j in 0..item.len() {
                            if item[j] == *instr_list {
                                item.remove(j);
                                break;
                            }
                        }
                    }
                }
            }
        }

        self.map_codes = matched_instr;
    }
}