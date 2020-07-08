use super::Part;
use std::collections::{VecDeque};
use regex::Regex;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(parse(input.as_str())),
        Part::Part2 => part2(parse(input.as_str()))
    };

    format!("{}",result)
}

fn parse(input:&str) -> (u32,u32) {
    // 10 players; last marble is worth 1618 points
    let r = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let caps = r.captures(input).unwrap();
    let players:u32= caps.get(1).unwrap().as_str().parse().ok().unwrap();
    let last:u32 = caps.get(2).unwrap().as_str().parse().ok().unwrap();

    (players,last)
}

fn part1(input:(u32,u32)) -> u32 {
    let (players, last_marble) = input;
    let mut circle = VecDeque::new();
    let mut scores = vec![];

    for _ in 0..players {
        scores.push(0);
    }

    for next_marble in 0..last_marble+1 {
        let current_player = next_marble % players;

        if next_marble % 23 == 0 && next_marble > 0 {
            // Special case..
            scores[current_player as usize] += next_marble;

            for _ in 0..7 {
                let marble = circle.pop_back().unwrap();
                circle.push_front(marble);
            }
            scores[current_player as usize] += circle.pop_front().unwrap();
            continue;
        }

        if next_marble == 0 {
            circle.push_back(next_marble);
        } else {
            // Shift the ring clock-wise
            for _ in 0..2 {
                let current_marble = circle.pop_front().unwrap();
                circle.push_back(current_marble);
            }
            circle.push_front(next_marble);
        }
    }

    *scores.iter().max().unwrap()
}

fn part2(input:(u32,u32)) -> u32 {
    part1((input.0, input.1 * 100))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;



    #[test]
    fn test1() {
        let input="9 players; last marble is worth 25 points";
        let res = part1(parse(input));
        println!("{}",res);
        assert_eq!(32,res);
    }

    #[test]
    fn test2() {
        let input="10 players; last marble is worth 1618 points";
        let res = part1(parse(input));
        println!("{}",res);
        assert_eq!(8317,res);
    }

    #[test]
    fn test3() {
        let input="30 players; last marble is worth 5807 points";
        let res = part1(parse(input));
        println!("{}",res);
        assert_eq!(37305,res);
    }

    //13 players; last marble is worth 7999 points: high score is 146373
    #[test]
    fn test4() {
        let input="13 players; last marble is worth 7999 points";
        let res = part1(parse(input));
        println!("{}",res);
        assert_eq!(146373,res);
    }

    #[test]
    fn test_part1() {
        let input="424 players; last marble is worth 71482 points";
        let res = part1(parse(input));
        println!("{}",res);
        assert_eq!(408679,res);
    }

    #[test]
    fn test_part2() {
        let input="424 players; last marble is worth 71482 points";
        let res = part2(parse(input));
        println!("{}",res);
        assert_eq!(3443939356,res);
    }









}
