use super::Part;
use crate::util::opcode::*;


pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{}",result)
}



fn part1(input:&str) -> i64 {
    let mut comp = Computer::new();
    comp.parse_input_file(input).0.iter().filter(|(_,l)|l.len()>=3).count() as i64
}


fn part2(input:&str) -> i64 {
    let mut comp = Computer::new();
    let  (mappings,program) = comp.parse_input_file(input);

    // Map codes with instructions
    comp.solve_opcodes(mappings);

    // Run program
    comp.run_program(program);

    // Result is value in register 0
    comp.get_register_value(0)
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::util::opcode::OpCode::*;


    const INPUT:&str = "Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";

    const INPUT2:&str = "Before: [0, 3, 3, 0]
5 0 2 1
After:  [0, 0, 3, 0]

Before: [0, 2, 3, 2]
3 3 2 3
After:  [0, 2, 3, 4]

Before: [2, 1, 0, 0]
10 1 2 3
After:  [2, 1, 0, 2]

Before: [0, 2, 3, 3]
14 2 2 3
After:  [0, 2, 3, 2]

Before: [3, 2, 2, 2]
10 0 3 1
After:  [3, 9, 2, 2]

Before: [2, 2, 1, 0]
10 0 3 2
After:  [2, 2, 6, 0]

Before: [1, 1, 2, 0]
7 0 2 2
After:  [1, 1, 2, 0]";

    #[test]
    fn test1() {
        let mut comp = Computer::new();
        let instr1 = (OpCode::SETI,10,0,0);
        let instr2 = (OpCode::SETI,5,0,1);
        let instr3 = (OpCode::ADDR,0,1,2);

        comp.run_instruction(instr1);
        comp.run_instruction(instr2);
        comp.run_instruction(instr3);
        println!("res={:?}",comp.get_register_value(2));
    }

    #[test]
    fn test2() {
        let mut comp = Computer::new();
        let in_state = vec![3, 2, 1, 1];
        let out_state = vec![3, 2, 2, 1];
        let instr = vec![9, 2, 1, 2];

        let  res = comp.test_opcodes(&instr,&in_state, &out_state);
        println!("{:?}",res);
        assert_eq!(vec![ADDI,MULTR,SETI],res);
    }

    #[test]
    fn test3() {
        let res = part1(INPUT);
        println!("res = {} ", res);
    }

    #[test]
    fn test4() {
        let res = part1(INPUT2);
        println!("res = {} ", res);
    }

    #[test]
    fn test_part1() {
        let res = part1(include_str!("../../input_16.txt"));
        println!("res = {} ", res);
        assert_eq!(500, res);
    }

    #[test]
    fn test_part2() {
        let res = part2(include_str!("../../input_16.txt"));
        println!("res = {} ", res);
        assert_eq!(533,res);
    }

}
