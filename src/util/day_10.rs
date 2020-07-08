use super::Part;
use std::collections::{HashMap};
use regex::Regex;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(parse(input.as_str())),
        Part::Part2 => part2(parse(input.as_str()))
    };

    format!("{}",result)
}

fn parse(input:&str) -> Vec<((i64,i64),(i64,i64))> {
    // position=< 9,  1> velocity=< 0,  2>
    let mut list = vec![];
    input.lines().for_each( |line| {
        let r = Regex::new(r"[^0-9-]+([-\d]+)[^0-9-]+([-\d]+)[^0-9-]+([-\d]+)[^0-9-]+([-\d]+)").unwrap();
        let caps = r.captures(line).unwrap();
        let x:i64 = caps.get(1).unwrap().as_str().parse().ok().unwrap();
        let y:i64 = caps.get(2).unwrap().as_str().parse().ok().unwrap();
        let v_x:i64 = caps.get(3).unwrap().as_str().parse().ok().unwrap();
        let v_y:i64 = caps.get(4).unwrap().as_str().parse().ok().unwrap();

        let el = ((x,y),(v_x,v_y));
        list.push(el);
    });

    list
}

fn advance(input:&mut Vec<((i64,i64),(i64,i64))>) {
    input.iter_mut().for_each(|((x,y),(d_x,d_y))|{
        *x += *d_x;
        *y += *d_y;
    });
}

fn reverse(input:&mut Vec<((i64, i64), (i64, i64))>) {
    input.iter_mut().for_each(|((x,y),(d_x,d_y))|{
        *x -= *d_x;
        *y -= *d_y;
    });
}

fn size(input:&Vec<((i64,i64),(i64,i64))>) -> i64 {
    let max_x = input.iter().map( |((x,_),(_,_)) | *x ).max().unwrap();
    let min_x = input.iter().map( |((x,_),(_,_)) | *x ).min().unwrap();

    let max_y = input.iter().map( |((_,y),(_,_)) | *y ).max().unwrap();
    let min_y = input.iter().map( |((_,y),(_,_)) | *y ).min().unwrap();

    (max_x - min_x).abs() * (max_y - min_y).abs()
}

fn to_string(input:&Vec<((i64, i64), (i64, i64))>) -> String {
    let mut s = String::new();
    let max_x = input.iter().map( |((x,_),(_,_)) | *x ).max().unwrap();
    let min_x = input.iter().map( |((x,_),(_,_)) | *x ).min().unwrap();

    let max_y = input.iter().map( |((_,y),(_,_)) | *y ).max().unwrap();
    let min_y = input.iter().map( |((_,y),(_,_)) | *y ).min().unwrap();

    let mut map = HashMap::new();

    input.iter().for_each(|((x,y),(_,_))| {
       map.insert((*x,*y),1);
    });

    for y in min_y..max_y+1 {
        for x in min_x..max_x + 1 {
            if map.contains_key(&(x,y)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }

    s
}

fn solve_message(mut input:Vec<((i64,i64),(i64,i64))>) -> (String,String) {
    let mut current_size = size(&input);
    let mut n = 0;
    loop {
        // Advance points
        advance(&mut input);
        n += 1;
        let next_size = size(&input);

        // Is size increasing?
        if next_size > current_size {
            reverse(&mut input);
            break;
        }

        current_size = next_size;
    }

    (format!("{}",n-1), to_string(&input))
}

fn part1(input:Vec<((i64,i64),(i64,i64))>) -> String {
    solve_message(input).1
}

fn part2(input:Vec<((i64,i64),(i64,i64))>) -> String {
    solve_message(input).0
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const INPUT:&str = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

    #[test]
    fn test1() {
        let res = part1(parse(INPUT));
        println!("{}",res);
        let result = "#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
";
        assert_eq!(result,res);
    }

    #[test]
    fn test2() {
        let res = part2(parse(INPUT));
        println!("{}",res);
        assert_eq!("3",res);
    }


}
