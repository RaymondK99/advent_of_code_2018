use super::Part;
use std::collections::{HashSet, BinaryHeap};
use regex::Regex;
use std::cmp::Ordering;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    let result = match part {
        Part::Part1 => part1(parse(lines)),
        Part::Part2 => part2(parse(lines),5,60)
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

fn next_root_node(input:&mut Vec<(char,char)>) -> Option<(char,char)> {
    let mut res = None;
    // Which of the parents is not a child node?
    if !input.is_empty() {
        let parent_nodes: HashSet<char> = input.iter().map(|(c1, _)| *c1).collect();
        let child_nodes: HashSet<char> = input.iter().map(|(_, c2)| *c2).collect();

        let mut root_nodes: Vec<char> = parent_nodes.iter().filter(|&p| !child_nodes.contains(p)).map(|p| *p).collect();
        root_nodes.sort();

        // Find item with no parent
        let item = root_nodes.remove(0);

        loop {
            let index = input.iter().enumerate().find(|(_, (c1, _))| *c1 == item);
            if index.is_some() {
                let (ind,(ch1,ch2)) = index.unwrap();
                res = Some((*ch1,*ch2));
                input.remove(ind);
            } else {
                break;
            }
        }
    }

    res
}

fn peek_root_nodes(input:&Vec<(char,char)>) -> Vec<char> {
    // Which of the parents is not a child node?
    let parent_nodes: HashSet<char> = input.iter().map(|(c1, _)| *c1).collect();
    let child_nodes: HashSet<char> = input.iter().map(|(_, c2)| *c2).collect();

    let mut root_nodes: Vec<char> = parent_nodes.iter().filter(|&p| !child_nodes.contains(p)).map(|p| *p).collect();
    root_nodes.sort();

    let mut root_pairs:Vec<(char,char)> = input.iter().filter(|(p,_)| root_nodes.contains(p)).map(|(p,c)|(*p,*c)).collect();
    root_pairs.sort();

    //root_pairs
    root_nodes.to_vec()
}

fn remove_node(item:char, input:&mut Vec<(char,char)>) {
    //let mut element = input.iter().find(|(p,_)| *p == item).clone();
    loop {
        let element = input.iter().enumerate().find(|(_,(p,_))| *p == item);
        if element.is_some() {
            let index = element.unwrap().0;
            input.remove(index);
        } else {
            break;
        }
    }

}

fn part1( mut input:Vec<(char,char)>) -> String {
    let mut result = vec![];

    // Which of the parents is not a child node?
    while !input.is_empty() {
        let (parent, child) = next_root_node(&mut input).unwrap();

        result.push(parent);

        if input.len() == 0 {
            result.push(child);
        }
    }

    result.iter().collect()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Worker {
    t_finish:u32,
    item:char,
}

impl Ord for Worker {
    fn cmp(&self, other: &Worker) -> Ordering {
        other.t_finish.cmp(&self.t_finish)
            .then_with(|| other.item.cmp(&self.item))
    }
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Worker) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part2(mut input:Vec<(char,char)>, num_workers:usize, base_cost:u32) -> String {
    let mut workers = BinaryHeap::new();
    let mut taken = vec![];
    let mut t = 0;
    let mut res = vec![];

    // Which of the parents is not a child node?
    while !input.is_empty() {

        let mut root_nodes:Vec<char> = peek_root_nodes(&input).iter()
            .filter( |&p| !taken.contains(p)).map(|p|*p).collect();

        //println!("Fetched {:?}", root_nodes);

        // Fetch
        while workers.len() < num_workers && !root_nodes.is_empty() {
            // Fetch available root nodes, not already taken

            // Allocate worker
            let item = *root_nodes.first().unwrap();
            root_nodes.remove(0);
            taken.push(item );
            let effort = (item as u32 - 'A' as u32 + 1 + base_cost) as u32;
            let worker = Worker { t_finish: t + effort, item: item };
            //println!("Add item {} to worker, will finish as t={}", item, worker.t_finish);
            workers.push(worker);
        }


        // Process first step
        let t1 = workers.peek().unwrap().t_finish;
        t = t1;
        while workers.peek().is_some() && workers.peek().unwrap().t_finish == t {
            let finished_worker = workers.pop().unwrap();
            //println!("Finished {} at time {}", finished_worker.item, finished_worker.t_finish);

            if input.len() == 1 {
                let last = input.pop().unwrap();
                res.push(last.0);
                res.push(last.1);
                let effort = (last.1 as u32 - 'A' as u32 + 1 + base_cost) as u32;
                t += effort;

                //println!("Finished {} at time {}", last.1, t);
                return format!("{}",t);
            } else {
                remove_node(finished_worker.item, &mut input);
                res.push(finished_worker.item);
            }
        }
    }

    String::from("0")
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

    const INPUT_REAL:&str = "Step O must be finished before step W can begin.
Step S must be finished before step V can begin.
Step Z must be finished before step B can begin.
Step F must be finished before step R can begin.
Step I must be finished before step D can begin.
Step W must be finished before step P can begin.
Step J must be finished before step E can begin.
Step P must be finished before step N can begin.
Step Q must be finished before step V can begin.
Step D must be finished before step K can begin.
Step X must be finished before step N can begin.
Step E must be finished before step B can begin.
Step L must be finished before step H can begin.
Step A must be finished before step T can begin.
Step U must be finished before step R can begin.
Step M must be finished before step T can begin.
Step V must be finished before step R can begin.
Step N must be finished before step C can begin.
Step T must be finished before step C can begin.
Step Y must be finished before step B can begin.
Step H must be finished before step B can begin.
Step B must be finished before step C can begin.
Step C must be finished before step K can begin.
Step R must be finished before step K can begin.
Step G must be finished before step K can begin.
Step Q must be finished before step K can begin.
Step U must be finished before step Y can begin.
Step L must be finished before step G can begin.
Step S must be finished before step D can begin.
Step E must be finished before step R can begin.
Step Z must be finished before step M can begin.
Step U must be finished before step K can begin.
Step Q must be finished before step H can begin.
Step T must be finished before step B can begin.
Step J must be finished before step Q can begin.
Step X must be finished before step V can begin.
Step Q must be finished before step U can begin.
Step T must be finished before step K can begin.
Step S must be finished before step B can begin.
Step L must be finished before step C can begin.
Step Q must be finished before step D can begin.
Step E must be finished before step K can begin.
Step N must be finished before step G can begin.
Step L must be finished before step T can begin.
Step E must be finished before step L can begin.
Step A must be finished before step N can begin.
Step V must be finished before step C can begin.
Step D must be finished before step L can begin.
Step O must be finished before step S can begin.
Step V must be finished before step Y can begin.
Step N must be finished before step T can begin.
Step I must be finished before step H can begin.
Step U must be finished before step N can begin.
Step O must be finished before step Y can begin.
Step J must be finished before step C can begin.
Step Y must be finished before step C can begin.
Step W must be finished before step A can begin.
Step M must be finished before step C can begin.
Step X must be finished before step E can begin.
Step S must be finished before step J can begin.
Step U must be finished before step C can begin.
Step H must be finished before step K can begin.
Step Q must be finished before step B can begin.
Step E must be finished before step G can begin.
Step N must be finished before step H can begin.
Step I must be finished before step J can begin.
Step P must be finished before step B can begin.
Step Z must be finished before step T can begin.
Step J must be finished before step M can begin.
Step C must be finished before step G can begin.
Step I must be finished before step B can begin.
Step D must be finished before step G can begin.
Step X must be finished before step T can begin.
Step O must be finished before step F can begin.
Step A must be finished before step Y can begin.
Step S must be finished before step G can begin.
Step X must be finished before step K can begin.
Step L must be finished before step M can begin.
Step A must be finished before step H can begin.
Step D must be finished before step H can begin.
Step U must be finished before step T can begin.
Step B must be finished before step K can begin.
Step S must be finished before step C can begin.
Step W must be finished before step R can begin.
Step M must be finished before step G can begin.
Step M must be finished before step H can begin.
Step J must be finished before step D can begin.
Step W must be finished before step Y can begin.
Step S must be finished before step Y can begin.
Step A must be finished before step G can begin.
Step P must be finished before step M can begin.
Step C must be finished before step R can begin.
Step Q must be finished before step Y can begin.
Step O must be finished before step H can begin.
Step O must be finished before step R can begin.
Step Q must be finished before step M can begin.
Step V must be finished before step B can begin.
Step H must be finished before step G can begin.
Step J must be finished before step V can begin.
Step M must be finished before step R can begin.
Step R must be finished before step G can begin.
";

    #[test]
    fn test1() {
        let res = part1(parse(INPUT.lines().collect()));
        println!("{:?}",res);
        assert_eq!("CABDFE",res);
    }

    #[test]
    fn test_part1() {
        let res = part1(parse(INPUT_REAL.lines().collect()));
        println!("{:?}",res);
        assert_eq!("IOFSJQDUWAPXELNVYZMHTBCRGK",res);
    }


    #[test]
    fn test2() {
        let res = part2(parse(INPUT.lines().collect()),2,0);
        println!("{:?}",res);
        assert_eq!("15",res);
    }

    #[test]
    fn test_part2() {
        let res = part2(parse(INPUT_REAL.lines().collect()),5,60);
        println!("{:?}",res);
        assert_eq!("931",res);
    }
}
