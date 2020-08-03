use super::Part;
use std::collections::{VecDeque};

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{}",result)
}

fn parse(input:&str) -> Vec<(i32,i32,i32,i32)>{
    let mut list = vec![];

    input.lines()
        .for_each(|line| {
            let elements:Vec<&str> = line.trim().split(',').collect();
            let x:i32 = elements[0].trim().parse().ok().unwrap();
            let y:i32 = elements[1].trim().parse().ok().unwrap();
            let z:i32 = elements[2].trim().parse().ok().unwrap();
            let w:i32 = elements[3].trim().parse().ok().unwrap();
            list.push((x,y,z,w));
        });
    list
}

fn dist_point(a:(i32,i32,i32,i32),b:(i32,i32,i32,i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()
}

fn dist_const(a:&Vec<(i32,i32,i32,i32)>, b:&Vec<(i32,i32,i32,i32)>) -> i32 {
    let mut dist = std::i32::MAX;
    for i in 0..a.len() {
        for j in 0..b.len() {
            dist = std::cmp::min(dist, dist_point( a[i], b[j]));
        }
    }

    dist
}

fn part1(input:&str) -> i32 {
    let list = parse(input);
    let mut constellations : VecDeque<Vec<(i32,i32,i32,i32)>> = list.iter().map(|item| vec![*item]).collect();

    loop {

        let mut no_const = constellations.len();
        let mut n = 0;
        let mut merges = 0;

        while n < no_const {
            let mut first = constellations.pop_front().unwrap();

            let cnt = constellations.len();
            let mut i = 0;
            while i < cnt {
                let mut second = constellations.pop_front().unwrap();
                if dist_const(&first, &second) <= 3 {
                    // Merge
                    first.append(&mut second);
                    no_const -= 1;
                    merges += 1;
                } else {
                    constellations.push_back(second);
                }
                i += 1;
            }

            constellations.push_back(first);
            n += 1;
        }

        if merges == 0 {
            break;
        }

    }

    constellations.len() as i32
}

fn part2(_input:&str) -> i32 {
    2
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let test = " 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0";

        let res = part1(test);
        println!("{:?}",res);
        assert_eq!(2, res);


    }

    #[test]
    fn test2() {
        let test = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";

        let res = part1(test);
        println!("{:?}",res);
        assert_eq!(4, res);

    }

    #[test]
    fn test3() {
        let test = "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";

        let res = part1(test);
        println!("{:?}",res);
        assert_eq!(8, res);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_25.txt");

        let res = part1(input);
        println!("{:?}",res);
        assert_eq!(430, res);
    }

}
