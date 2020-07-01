use super::Part;
use std::collections::HashSet;

pub fn solve(input : String, part: Part) -> String {

    let input:Vec<i32> = input.lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    };

    format!("{}",result)
}

fn part1(input:Vec<i32>) -> i32 {
    input.iter().sum()
}

fn part2(input:Vec<i32>) -> i32 {
    let mut curr_freq = 0;
    let mut set = HashSet::new();

    // insert first element
    set.insert(curr_freq);

    loop {
        for elem in input.iter() {
            curr_freq += *elem;

            if set.contains(&curr_freq) {
                return curr_freq;
            } else {
                set.insert(curr_freq);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let freq = vec![1,1,1];
        let res = part1(freq);
        println!("{}", res);
        assert_eq!(3, res);
    }

    #[test]
    fn test2() {
        let freq = vec![1,1,-2];
        let res = part1(freq);
        println!("{}", res);
        assert_eq!(0, res);
    }

    #[test]
    fn test3() {
        let freq = vec![3,3,4,-2,-4];
        let res = part2(freq);
        println!("{}", res);
        assert_eq!(10, res);
    }


}
