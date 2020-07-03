use super::Part;
use std::collections::{HashSet};
use regex::Regex;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    let result = match part {
        Part::Part1 => part1(parse(lines)),
        Part::Part2 => part2(lines)
    };

    format!("{}",result)
}


fn parse(input:Vec<&str>) -> Vec<(char,char)> {
    // Step C must be finished before step A can begin.
    let r = Regex::new(r"Step (.) must be finished before step (.) can begin\.$").unwrap();

    input.iter().map( |&line|{
        let caps = r.captures(line).unwrap();
        let b1:Vec<char> = caps.get(1).unwrap().as_str().chars().collect();
        let b2:Vec<char> = caps.get(2).unwrap().as_str().chars().collect();
        (b1[0],b2[0])
    }).collect()

}

fn part1( mut input:Vec<(char,char)>) -> String {
    let mut result = vec![];

    // Which of the parents is not a child node?
    while !input.is_empty() {
        let parent_nodes: HashSet<char> = input.iter().map(|(c1, _)| *c1).collect();
        let child_nodes: HashSet<char> = input.iter().map(|(_, c2)| *c2).collect();

        let mut root_nodes: Vec<char> = parent_nodes.iter().filter(|&p| !child_nodes.contains(p)).map(|p| *p).collect();
        root_nodes.sort();

        // Find item with no parent
        let item = root_nodes.remove(0);

        // Find index in input vector
        let mut added = false;
        loop {
            let index = input.iter().enumerate().find(|(_, (c1, _))| *c1 == item);
            if index.is_some() {
                let (ind,(ch1,ch2)) = index.unwrap();
                if !added {
                    result.push(*ch1);
                    added = true;
                    if input.len() == 1 {
                        result.push(*ch2);
                    }
                }
                input.remove(ind);
            } else {
                break;
            }
        }
    }

    result.iter().collect()
}

fn part2( input:Vec<&str>) -> String {
    String::from("2323")
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    const INPUT:&str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn test_part1() {
        let res = part1(parse(INPUT.lines().collect()));
        println!("{:?}",res);
        assert_eq!("CABDFE",res);
    }


}
