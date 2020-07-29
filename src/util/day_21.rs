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
    let (pc_reg_no, program) = Computer::parse_ip_program(input);
    comp.set_register_value(0,700_000_000);
    comp.run_program_with_pc(pc_reg_no, program);
    comp.get_register_value(0)
}


fn part2(input:&str) -> i64 {
    2
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::sync::Arc;

    const INPUT_REAL:&str = "#ip 4
seti 123 0 2
bani 2 456 2
eqri 2 72 2
addr 2 4 4
seti 0 0 4
seti 0 0 2
bori 2 65536 5
seti 5234604 6 2
bani 5 255 3
addr 2 3 2
bani 2 16777215 2
muli 2 65899 2
bani 2 16777215 2
gtir 256 5 3
addr 3 4 4
addi 4 1 4
seti 27 2 4
seti 0 0 3
addi 3 1 1
muli 1 256 1
gtrr 1 5 1
addr 1 4 4
addi 4 1 4
seti 25 6 4
addi 3 1 3
seti 17 7 4
setr 3 4 5
seti 7 8 4
eqrr 2 0 3
addr 3 4 4
seti 5 6 4";


}
