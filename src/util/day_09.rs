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
    let mut last_marble_pos = 0;
    let mut scores = vec![];

    for _ in 0..players {
        scores.push(0);
    }

    for next_marble in 0..last_marble+1 {
        let current_player = next_marble % players;

        if next_marble % 23 == 0 && next_marble > 0 {
            // Special case..
            scores[current_player as usize] += next_marble;
            if last_marble_pos >= 7 {
                last_marble_pos -= 7;
            } else {
                last_marble_pos = circle.len() - (7 - last_marble_pos);
            }

            scores[current_player as usize] += circle.remove(last_marble_pos as usize).unwrap();
            continue;
        }

        if next_marble >  0 && circle.len() > 1 {
            let pos = last_marble_pos + 2;

            //println!("last_pos={}, new_pos={}, len={}",last_marble_pos,pos,circle.len());
            if pos == circle.len() {
                circle.push_back(next_marble);
                last_marble_pos = pos;
            } else {
                last_marble_pos = pos % circle.len();
                circle.insert(pos % circle.len(), next_marble);
            }
        } else {
            circle.push_back(next_marble);
            last_marble_pos = circle.len()-1;
        }
    }

    *scores.iter().max().unwrap()
}

fn part2(input:(u32,u32)) -> u32 {
    2
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

}
