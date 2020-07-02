use super::Part;
use std::collections::{HashSet, HashMap};

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(parse(input.as_str())),
        Part::Part2 => part2(parse(input.as_str()), 10000),
    };

    format!("{}",result)
}

fn parse(input:&str) -> Vec<(i32,i32)> {
    input.lines().map(|line| {
        let pair:Vec<&str> = line.split(',').collect();
        (pair[0].trim().parse().ok().unwrap(), pair[1].trim().parse().ok().unwrap()) }).collect()
}



fn part1(points:Vec<(i32,i32)>) -> i32 {
    let mut map = HashMap::new();
    let mut border_points = HashSet::new();
    let max_x = points.iter().map(|item| item.0).max().unwrap();
    let min_x = points.iter().map(|item| item.0).min().unwrap();
    let max_y = points.iter().map(|item| item.1).max().unwrap();
    let min_y = points.iter().map(|item| item.1).min().unwrap();

    for x in min_x..max_x+1 {
        for y in min_y..max_y+1 {

            // Measure distance to each point and select the closest one...
            let mut dist_min = 9999;
            let mut n = 1;
            let mut point_number = 1;
            let mut confl = false;
            for (p_x,p_y) in points.iter() {
                let dist = (x-*p_x).abs() + (y-*p_y).abs();
                if dist_min > dist {
                    dist_min = dist;
                    confl = false;
                    point_number = n;
                } else if dist_min == dist {
                    confl = true;
                }
                n += 1;
            }

            if x == min_x || x == max_x || y == min_y || y == max_y {
                border_points.insert(point_number);
            }

            if !confl {
                let entry = map.entry(point_number).or_insert(0);
                *entry += 1;
            }
        }
    }

    map.iter().filter(|(&k,_)| !border_points.contains(&k)).map(|(_,v)| *v).max().unwrap()
}

fn part2(points:Vec<(i32,i32)>, limit:i32) -> i32 {
    let mut map = HashMap::new();
    let max_x = points.iter().map(|item| item.0).max().unwrap();
    let min_x = points.iter().map(|item| item.0).min().unwrap();
    let max_y = points.iter().map(|item| item.1).max().unwrap();
    let min_y = points.iter().map(|item| item.1).min().unwrap();

    for x in min_x..max_x+1 {
        for y in min_y..max_y+1 {

            // Measure distance to each point and select the closest one...
            let mut dist_sum = 0;
            for (p_x,p_y) in points.iter() {
                let dist = (x-*p_x).abs() + (y-*p_y).abs();
                dist_sum += dist;
            }

            if dist_sum < limit {
                map.insert( (x,y), dist_sum);
            }
        }
    }

    map.iter().count() as i32
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        //let res = part1(input.lines().collect());
        let res = part1(parse(input));
        println!("{:?}", res);
        assert_eq!(17, res);
    }

    #[test]
    fn test2() {
        let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        //let res = part1(input.lines().collect());
        let res = part2(parse(input), 32);
        println!("{:?}", res);
        assert_eq!(16, res);
    }

}
