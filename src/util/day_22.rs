use super::Part;
use std::collections::{HashMap, BinaryHeap};
use regex::Regex;
use crate::util::day_22::Tool::{Torch, ClimbingGear, Neither};
use std::cmp::Ordering;


pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{}",result)
}

fn parse(input:&str) -> ((i64,i64),i64) {
    let lines:Vec<&str> = input.lines().collect();
    /*
    depth: 3339
    target: 10,715
    */
    let r1 = Regex::new(r"depth: ([\d]+)$").unwrap();
    let r2 = Regex::new(r"target: ([\d]+),([\d]+)$").unwrap();

    let depth:i64 = r1.captures(lines[0]).unwrap().get(1).unwrap().as_str().parse().ok().unwrap();
    let x:i64 = r2.captures(lines[1]).unwrap().get(1).unwrap().as_str().parse().ok().unwrap();
    let y:i64 = r2.captures(lines[1]).unwrap().get(2).unwrap().as_str().parse().ok().unwrap();

    ((x,y),depth)
}


#[derive(PartialEq,Debug,Eq,Copy,Clone,Hash)]
enum Tool {
    Neither,
    Torch,
    ClimbingGear
}

#[derive(PartialEq,Debug,Eq)]
struct Node {
    dist:i64,
    tool:Tool,
    x:i64,
    y:i64,
}


impl Node {
    fn new(dist:i64,x:i64,y:i64,tool:Tool) -> Node {
        Node{dist:dist,x:x,y:y,tool:tool}
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist.cmp(&self.dist))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

fn get_other_tool(region:i64,tool:Tool) -> Tool {
    for t in  [ClimbingGear, Neither, Torch].iter() {
        if *t != tool && match_region(region, *t) {
            return *t;
        }
    }
    panic!("...")
}

fn match_region(region:i64,tool:Tool) -> bool {
    match (region, tool) {
        (0, ClimbingGear) => true,
        (0, Torch) => true,
        (1, ClimbingGear) => true,
        (1, Neither) => true,
        (2, Neither) => true,
        (2, Torch) => true,
        _ => false,
    }
}

fn calc_erosion(geo:i64, depth:i64) -> i64 {
    (geo + depth) % 20183
}

fn calc_map((x_target,y_target):(i64,i64),depth:i64) -> HashMap<(i64,i64),i64> {
    let mut map = HashMap::new();

    for y in 0..=(y_target * 4) {
        for x in 0..=(x_target * 4) {
            let value = if (x == x_target && y == y_target) || (x == 0 && y == 0) {
                calc_erosion(0, depth)
            } else if y == 0 {
                calc_erosion(x * 16807, depth)
            } else if x == 0 {
                calc_erosion(y * 48271, depth)
            } else {
                let geo_value = map.get(&(x-1,y)).unwrap() * map.get(&(x,y-1)).unwrap();
                calc_erosion(geo_value, depth)
            };

            map.insert((x,y),value);
        }
    }

    map.iter_mut().for_each(|(_,v)|{
        *v = *v % 3;
    });

    map
}


fn part1(input:&str) -> i64 {
    let ((x_t,y_t),depth) = parse(input);
    let map = calc_map((x_t,y_t),depth);
    let sum:i64 = map.iter().filter(|((x,y),_)| *x <= x_t && *y <= y_t).map(|(_,v)| *v).sum();
    sum as i64
}

fn part2(input:&str) -> i64 {
    let ((x_t,y_t),depth) = parse(input);
    let map = calc_map((x_t,y_t),depth);

    let mut queue = BinaryHeap::new();
    let mut distances = HashMap::new();

    queue.push(Node::new(0,0,0,Torch));

    while !queue.is_empty() {

        let node = queue.pop().unwrap();
        let x = node.x;
        let y = node.y;

        // Find exit condition
        if x == x_t && y == y_t && node.tool == Torch {
            return node.dist;
        }

        // Generate list of adjacent nodes
        let dirs = vec![(x-1,y),(x+1,y),(x,y+1),(x,y-1)];

        let mut adjacent_nodes = vec![];
        let current_region = *map.get(&(x,y)).unwrap();
        let other_tool = get_other_tool(current_region, node.tool);

        // Change tool
        adjacent_nodes.push(Node::new(node.dist+7,x,y,other_tool));

        // Traverse to other nodes by not changing tool
        for (x1,y1) in dirs.iter() {
            let next_pos = map.get(&(*x1,*y1));
            if next_pos.is_none() {
                continue;
            }

            let next_region = *next_pos.unwrap();
            let matches_next_region = match_region(next_region, node.tool);

            // Traverse to next position by using the same tool
            if matches_next_region {
                adjacent_nodes.push(Node::new(node.dist + 1,*x1,*y1,node.tool));
            }
        }

        for next_node in adjacent_nodes {
            let next_dist = next_node.dist;
            let x1 = next_node.x;
            let y1 = next_node.y;
            let tool = next_node.tool;
            let prev_dist_opt = distances.get(&(x1,y1,next_node.tool));
            if prev_dist_opt.is_some()  {
                let prev_dist = *prev_dist_opt.unwrap();
                if prev_dist <= next_dist {
                    continue;
                }
            }

            queue.push(next_node);
            distances.insert((x1,y1,tool), next_dist);
        }
    }

    panic!("No solution!")
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let _map = calc_map((10,10),510);
    }

    #[test]
    fn test2() {
        let input = "depth: 510
target: 10,10";
        let res = part1(input);
        println!("res={}",res);
        assert_eq!(114,res);
    }


    #[test]
    fn test3() {
        let input = "depth: 3339
target: 10,715";
        let res = part1(input);
        println!("res={}",res);
    }

    #[test]
    fn test_part2_test1() {
        let input = "depth: 510
target: 10,10";
        let res = part2(input);
        println!("res={}",res);
        assert_eq!(45,res);
    }

    #[test]
    fn test_part2_test2() {
        let input = "depth: 3339
target: 10,715";

        let res = part2(input);
        println!("res={}",res);
        assert_eq!(980,res);
    }

}
