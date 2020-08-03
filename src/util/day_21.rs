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


fn part2(_input:&str) -> i64 {
    2
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.



}
