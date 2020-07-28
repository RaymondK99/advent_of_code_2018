use super::Part;
use std::collections::HashMap;
use regex::Regex;


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

    println!("Depth:{},x:{},y:{}",depth,x,y);
    ((x,y),depth)
}

fn part1(input:&str) -> i64 {
    let ((x_t,y_t),depth) = parse(input);
    let map = calc_map((x_t,y_t),depth);
    let sum:i64 = map.iter().map(|(_,v)| *v).sum();
    sum as i64
}


fn part2(input:&str) -> i64 {
    2
}

fn calc_erosion(geo:i64, depth:i64) -> i64 {
    (geo + depth) % 20183
}

fn calc_map((x_target,y_target):(i64,i64),depth:i64) -> HashMap<(i64,i64),i64> {
    let mut map = HashMap::new();

    for y in 0..=y_target {
        for x in 0..=x_target {
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

fn print_map(map:&HashMap<(i64,i64),i64>) {
    let mut map_str = String::new();
    let max_y = map.iter().map(|((_,y),_)| *y).max().unwrap();
    let min_y = map.iter().map(|((_,y),_)| *y).min().unwrap();
    let max_x = map.iter().map(|((x,_),_)| *x).max().unwrap();
    let min_x = map.iter().map(|((x,_),_)| *x).min().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {

            map_str.push(match map.get(&(x,y)).unwrap()  {
                0 => '.',
                1 => '=',
                2 => '|',
                _ => panic!("..."),

            });
        }
        map_str.push('\n');
    }

    println!("{}",map_str);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {

        let map = calc_map((10,10),510);
        print_map(&map);
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




}
