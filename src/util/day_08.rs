use super::Part;
use std::collections::{VecDeque, HashMap};

pub fn solve(input : String, part: Part) -> String {
    let input:Vec<i32> = input.split_whitespace().map(|s| s.parse().ok().unwrap()).collect();

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    };

    format!("{}",result)
}

#[derive(Debug,Eq, PartialEq,Clone)]
struct Header {
    nodes:i32,
    datas:i32,
    name:i32,
    child_nodes:Vec<i32>,
    meta_data:Vec<i32>,
}

fn part1(input:Vec<i32>) -> i32 {
    let mut stack:VecDeque<Header> = VecDeque::new();
    let mut sum = 0;
    let mut i = 0;
    let mut num = 1;

    // Create first entry
    stack.push_front(Header{nodes:input[i],datas:input[i+1],name:1,child_nodes:vec![],meta_data:vec![]});
    i+=2;
    num += 1;

    while !stack.is_empty() {
        let mut header = stack.pop_front().unwrap();
        //println!("Process header {:?}",header);
        if header.nodes > 0 {
            header.nodes -= 1;

            // Read next item
            let next = Header{nodes:input[i],datas:input[i+1],name:num,child_nodes:vec![],meta_data:vec![]};
            num += 1;
            i += 2;

            // push parent to front
            stack.push_front(header);

            // Push child to front
            stack.push_front(next);
        } else {
            // Process meta data
            let mut n = 0;
            while n < header.datas {
                let meta_data = input[i];
                i += 1;
                n += 1;
                sum += meta_data;
            }
        }
    }

    sum
}

fn part2(input:Vec<i32>) -> i32 {
    let mut stack:VecDeque<Header> = VecDeque::new();
    let mut nodes_map:HashMap<i32,Header> = HashMap::new();
    let mut sum = 0;
    let mut i = 0;
    let mut num = 1;

    // Create first entry
    stack.push_front(Header{nodes:input[i],datas:input[i+1],name:1,child_nodes:vec![],meta_data:vec![]});
    i+=2;
    num += 1;

    while !stack.is_empty() {
        let mut header = stack.pop_front().unwrap();

        if !nodes_map.contains_key(&header.name) {
            nodes_map.insert(header.name, header.clone());
        }

        //println!("Process header {:?}",header);
        if header.nodes > 0 {
            header.nodes -= 1;

            // Read next item
            let next = Header{nodes:input[i],datas:input[i+1],name:num,child_nodes:vec![],meta_data:vec![]};
            num += 1;
            i += 2;

            let entry = nodes_map.entry(header.name).or_insert(header.clone());
            entry.child_nodes.push(next.name);

            // push parent to front
            stack.push_front(header);

            // Push child to front
            stack.push_front(next);
        } else {
            // Process meta data
            let mut n = 0;
            while n < header.datas {
                let meta_data = input[i];
                i += 1;
                n += 1;
                nodes_map.get_mut(&header.name).unwrap().meta_data.push(meta_data);
            }
        }
    }

    stack.push_front(nodes_map.remove(&1).unwrap());
    while !stack.is_empty() {
        let node = stack.pop_front().unwrap();

        // Has child nodes?
        if node.child_nodes.is_empty() {
            // Sum meta data entries
            node.meta_data.iter().for_each(|m| {sum += *m;});
        } else {
            for meta_data in node.meta_data.iter() {
                let next = node.child_nodes.get((meta_data-1) as usize);

                if next.is_some() {
                    // Push node to stack
                    stack.push_front(nodes_map.get(next.unwrap()).unwrap().clone());
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input="2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let res = part1(input.split_whitespace().map(|s|s.parse().ok().unwrap()).collect());
        println!("{}", res);
        assert_eq!(138, res);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_08.txt");
        let res = part1(input.split_whitespace().map(|s|s.parse().ok().unwrap()).collect());
        println!("{}", res);
        assert_eq!(45194, res);
    }

    #[test]
    fn test2() {
        let input="2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let res = part2(input.split_whitespace().map(|s|s.parse().ok().unwrap()).collect());
        println!("{}", res);
        assert_eq!(66, res);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_08.txt");
        let res = part2(input.split_whitespace().map(|s|s.parse().ok().unwrap()).collect());
        println!("{}", res);
        assert_eq!(22989, res);
    }




}
