use super::Part;
use crate::util::opcode::*;
use crate::util::opcode::OpCode::*;


pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{}",result)
}

fn parse_ip_program(input:&str) -> (usize, Vec<(OpCode,usize,usize,usize)>) {
    let mut lines:Vec<&str> = input.lines().collect();
    let mut instructions = vec![];
    // First line is #ip directive
    let first_line = lines.remove(0);
    let ip_reg:usize = first_line.split_whitespace().last().unwrap().parse().ok().unwrap();

    for instr in lines {
        let fields:Vec<&str> = instr.split_whitespace().collect();
        let instr_name = fields[0];
        let a:usize = fields[1].parse().ok().unwrap();
        let b:usize = fields[2].parse().ok().unwrap();
        let c:usize = fields[3].parse().ok().unwrap();

        let opcode = match instr_name {
            "addi" => ADDI,
            "addr" => ADDR,
            "seti" => SETI,
            "setr" => SETR,
            "muli" => MULTI,
            "mulr" => MULTR,
            "bani" => BANI,
            "banr" => BANR,
            "bori" => BORI,
            "borr" => BORR,
            "gtrr" => GTRR,
            "gtir" => GTIR,
            "grri" => GTRI,
            "eqrr" => EQRR,
            "eqri" => EQRI,
            "eqir" => EQIR,
            _ => panic!("unknown:{}",instr_name),

        };

        instructions.push((opcode,a,b,c));
    }

    (ip_reg,instructions)
}

fn part1(input:&str) -> i64 {
    let mut comp = Computer::new();
    let (pc_reg_no, program) = parse_ip_program(input);
    comp.run_program_with_pc(pc_reg_no, program);
    comp.get_register_value(0)
}


fn part2(input:&str) -> i64 {
    let mut comp = Computer::new();
    let (pc_reg_no, program) = parse_ip_program(input);

    let mut pc = 0;

    comp.set_register_value(0,1);

    while comp.get_register_value(1) != 1 {
        // Load instruction pointer to register linked to it
        comp.set_register_value(pc_reg_no, pc as i64);

        let instr = program[pc];
        comp.run_instruction(instr);

        // Fetch PC from reg
        pc = comp.get_register_value(pc_reg_no) as usize + 1;
    }

    let mut sum = 0;
    let number = *comp.get_registers().iter().max().unwrap();
    for i in 1..=number {
        if number % i == 0 {
            sum += i;
        }
    }

    sum
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::util::opcode::*;

    const INPUT_REAL:&str = "#ip 3
addi 3 16 3
seti 1 2 1
seti 1 1 2
mulr 1 2 5
eqrr 5 4 5
addr 5 3 3
addi 3 1 3
addr 1 0 0
addi 2 1 2
gtrr 2 4 5
addr 3 5 3
seti 2 3 3
addi 1 1 1
gtrr 1 4 5
addr 5 3 3
seti 1 6 3
mulr 3 3 3
addi 4 2 4
mulr 4 4 4
mulr 3 4 4
muli 4 11 4
addi 5 5 5
mulr 5 3 5
addi 5 15 5
addr 4 5 4
addr 3 0 3
seti 0 6 3
setr 3 5 5
mulr 5 3 5
addr 3 5 5
mulr 3 5 5
muli 5 14 5
mulr 5 3 5
addr 4 5 4
seti 0 5 0
seti 0 1 3";

    const INPUT:&str = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";


    #[test]
    fn test1() {
        let res = part1(INPUT);
        println!("res={}",res);
        assert_eq!(6,res);
    }

    #[test]
    fn test_part1() {
        let res = part1(INPUT_REAL);
        println!("res={}",res);
        assert_eq!(993,res);
    }

    #[test]
    fn test_part2() {
        let res = part2(INPUT_REAL);
        println!("res={}",res);
        assert_eq!(10708912,res);
    }

}
