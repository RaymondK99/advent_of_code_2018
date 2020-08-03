use super::Part;
use regex::Regex;


pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{}",result)
}

fn parse(input:&str) -> Vec<(i64,i64,i64,i64)> {
    /*
    pos=<0,0,0>, r=4
    */
    input.lines().map(|line| {
        let r1 = Regex::new(r"pos=<(\-?[\d]+),(\-?[\d]+),(\-?[\d]+)>, r=(\-?[\d]+)$").unwrap();

        let x: i64 = r1.captures(line).unwrap().get(1).unwrap().as_str().parse().ok().unwrap();
        let y: i64 = r1.captures(line).unwrap().get(2).unwrap().as_str().parse().ok().unwrap();
        let z: i64 = r1.captures(line).unwrap().get(3).unwrap().as_str().parse().ok().unwrap();
        let r: i64 = r1.captures(line).unwrap().get(4).unwrap().as_str().parse().ok().unwrap();
        (x,y,z,r)
    }).collect()

}

fn dist((x1,y1,z1,_):(i64,i64,i64,i64),(x2,y2,z2,_):(i64,i64,i64,i64)) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
}
fn part1(input:&str) -> i64 {
    let bots = parse(input);
    let max_bot = *bots.iter().max_by(|(_,_,_,r1),(_,_,_,r2)| r1.cmp(r2)).unwrap();
    bots.iter()
        .filter(|&bot| dist(*bot,max_bot) <= max_bot.3 )
        .count() as i64
}

fn part2(_input:&str) -> i64 {
    2
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const TEST_INPUT:&str = "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

    #[test]
    fn test1() {
        let res = part1(TEST_INPUT);
        println!("{:?}",res);
        assert_eq!(7,res);
    }

    #[test]
    fn test_part1() {
        let input  = include_str!("../../input_23.txt");
        let res = part1(input);
        println!("{:?}",res);
        assert_eq!(491,res);
    }

}
