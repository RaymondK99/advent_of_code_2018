use super::Part;
use std::collections::{VecDeque};

pub fn solve(input : String, part: Part) -> String {
    let input:Vec<i32> = input.split_whitespace().map(|s| s.parse().ok().unwrap()).collect();

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    };

    format!("{}",result)
}

#[derive(Debug)]
struct Header {
    nodes:i32,
    datas:i32,
}

fn part1(input:Vec<i32>) -> i32 {
    let mut stack:VecDeque<Header> = VecDeque::new();
    let mut sum = 0;
    let mut i = 0;

    // Create first entry
    stack.push_front(Header{nodes:input[i],datas:input[i+1]});
    i+=2;

    while !stack.is_empty() {
        let mut header = stack.pop_front().unwrap();
        //println!("Process header {:?}",header);
        if header.nodes > 0 {
            header.nodes -= 1;
            // push to front
            stack.push_front(header);

            // Read next item
            let next = Header{nodes:input[i],datas:input[i+1]};
            i += 2;
            stack.push_front(next);
        } else {
            // Process meta data
            let mut n = 0;
            //print!("Process {} meta datas:",header.datas);
            while n < header.datas {
                let meta_data = input[i];
                i += 1;
                n += 1;
                sum += meta_data;
                //print!("{} ",meta_data);
            }
            //println!();
        }

    }


    sum
}

fn part2(input:Vec<i32>) -> i32 {
   2
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


}
