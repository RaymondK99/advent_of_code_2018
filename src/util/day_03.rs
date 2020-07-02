use super::Part;
use std::collections::HashMap;
use regex::Regex;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    let result = match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    };

    format!("{}",result)
}


fn get_map(input:&Vec<&str>) -> HashMap<(u32,u32),u32> {
    let mut map = HashMap::new();
    let r = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    for &line in input.iter() {
        let caps = r.captures(line).unwrap();

        if caps.len() == 6 {
            let x: u32 = caps.get(2).unwrap().as_str().parse().ok().unwrap();
            let y: u32 = caps.get(3).unwrap().as_str().parse().ok().unwrap();
            let w: u32 = caps.get(4).unwrap().as_str().parse().ok().unwrap();
            let h: u32 = caps.get(5).unwrap().as_str().parse().ok().unwrap();

            for i in y..(y+h) {
                for j in x..(x+w) {
                    let entry = map.entry((i,j)).or_insert(0);
                    *entry += 1;
                }
            }
        }
    }

    map
}

fn part1(input:Vec<&str>) -> u32 {
    let map = get_map(&input);

    // Sum entries with more than 1 claim
    map.iter().filter( |(_,&v)| v > 1).count() as u32
}

fn part2(input:Vec<&str>) -> u32 {
    let map = get_map(&input);
    let r = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    // iterate through all points
    for &line in input.iter() {
        let caps = r.captures(line).unwrap();

        if caps.len() == 6 {
            let num: u32 = caps.get(1).unwrap().as_str().parse().ok().unwrap();
            let x: u32 = caps.get(2).unwrap().as_str().parse().ok().unwrap();
            let y: u32 = caps.get(3).unwrap().as_str().parse().ok().unwrap();
            let w: u32 = caps.get(4).unwrap().as_str().parse().ok().unwrap();
            let h: u32 = caps.get(5).unwrap().as_str().parse().ok().unwrap();

            let mut only_ones = true;
            for i in y..(y+h) {
                for j in x..(x+w) {
                    let count = map.get(&(i,j)).unwrap();
                    if *count != 1 {
                        only_ones = false;
                        break;
                    }
                }
            }

            if only_ones {
                return num
            }
        }
    }

    0
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n";
        let res = part1(input.lines().collect());
        println!("{}", res);
        assert_eq!(4, res);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_03.txt");
        let res = part1(input.lines().collect());
        println!("{}", res);
        assert_eq!(118322, res);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_03.txt");
        let res = part2(input.lines().collect());
        println!("{}", res);
        assert_eq!(1178, res);
    }
}
