use super::Part;
use std::collections::{VecDeque, HashMap, HashSet};


pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{}",result)
}


fn part1(input:&str) -> i64 {
    let map = parse(input.chars().collect());

    let mut queue = VecDeque::new();
    let mut dist_max = 0;
    let mut visited = HashSet::new();
    queue.push_back((0,0,0));

    while !queue.is_empty() {

        let (x,y,dist) = queue.pop_front().unwrap();

        // Check if visited already
        if visited.contains(&(x,y)) {
            continue;
        }

        visited.insert((x,y));

        let next_dist = dist + 1;
        dist_max = std::cmp::max(dist_max, dist);

        // Up?
        let up_pos = map.get(&(x,y-1));
        if up_pos.is_some() && *up_pos.unwrap() == '-' {
            queue.push_back((x,y-2,next_dist));
        }

        // Down?
        let down_pos = map.get(&(x,y+1));
        if down_pos.is_some() && *down_pos.unwrap() == '-' {
            queue.push_back((x,y+2,next_dist));
        }

        // Right?
        let left_pos = map.get(&(x+1,y));
        if left_pos.is_some() && *left_pos.unwrap() == '|' {
            queue.push_back((x+2,y,next_dist));
        }

        // Left?
        let right_pos = map.get(&(x-1,y));
        if right_pos.is_some() && *right_pos.unwrap() == '|' {
            queue.push_back((x-2,y,next_dist));
        }
    }

    dist_max as i64
}


fn part2(input:&str) -> i64 {
    2
}

fn get_options(input:&Vec<char>, curr:&Position) -> Vec<Position> {
    let mut options = vec![Position{x:curr.x,y:curr.y,index:curr.index+1}];
    let mut depth = 1;
    let mut index = curr.index+1;

    while depth > 0 {
        let ch = input[index];
        if ch == '(' {
            depth += 1;
        } else if ch == ')' {
            depth -= 1;
        } else if ch == '|' && depth == 1 {
            options.push(Position{x:curr.x,y:curr.y,index:index+1})
        }

        index += 1;
    }

    options
}

#[derive(Debug)]
struct Position {
    x:i32,
    y:i32,
    index:usize,
}

fn print_map(map:&HashMap<(i32,i32),char>) -> String {
    let mut map_str = String::new();
    let max_y = map.iter().map(|((_,y),_)| *y).max().unwrap() + 1;
    let min_y = map.iter().map(|((_,y),_)| *y).min().unwrap() - 1;
    let max_x = map.iter().map(|((x,_),_)| *x).max().unwrap() + 1;
    let min_x = map.iter().map(|((x,_),_)| *x).min().unwrap() - 1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {

            map_str.push(match map.get(&(x,y))  {
                None => '#',
                Some(ch) => *ch,
            });
        }
        map_str.push('\n');
    }

    map_str
}

fn parse(input:Vec<char>) -> HashMap<(i32,i32),char> {
    let mut map = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back( Position{x:0,y:0,index:0});

    map.insert((0,0),'X');

    while !queue.is_empty() {

        let current = queue.pop_front().unwrap();

        // Get direction
        let dir = input[current.index];

        if dir == '$' {
            continue;
        } else if dir == '^' {
            queue.push_back(Position{x:current.x,y:current.y,index:current.index+1});
        } else if dir.is_alphabetic() {
            let (x,y) = (current.x,current.y);
            let next = if dir == 'W' {
                map.insert((x-1,y),'|');
                map.insert((x-2,y),'.');
                Position{x:current.x-2,y:current.y,index:current.index+1}
            } else if dir == 'E' {
                map.insert((x+1,y),'|');
                map.insert((x+2,y),'.');
                Position{x:current.x+2,y:current.y,index:current.index+1}
            } else if dir == 'N' {
                map.insert((x,y-1),'-');
                map.insert((x,y-2),'.');
                Position{x:current.x,y:current.y-2,index:current.index+1}
            } else if dir == 'S' {
                map.insert((x,y+1),'-');
                map.insert((x,y+2),'.');
                Position{x:current.x,y:current.y+2,index:current.index+1}
            } else {
                panic!("....");
            };

            queue.push_back(next);
        } else if dir == '(' {
            let options = get_options(&input, &current);

            //println!("----------");
            for p in options {
                //println!("Option={:?}",&p);
                queue.push_back(p);
            }
        } else if dir == ')' {
            queue.push_back(Position{x:current.x,y:current.y,index:current.index+1});
        }
    }

    map
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input ="^WNE$";
        let map = parse(input.chars().collect());
        println!("{}",print_map(&map));

    }

    #[test]
    fn test2() {
        let input ="^ENWWW(NEEE|SSE)$";
        let map = parse(input.chars().collect());
        println!("{}",print_map(&map));
    }

    #[test]
    fn test3() {
        let input ="^ENWWW(NEEE|SSE(EE|N))$";
        let map = parse(input.chars().collect());
        println!("{}",print_map(&map));
    }

    #[test]
    fn test4() {
        let res = "###########
#.|.#.|.#.#
#-###-#-#-#
#.|.|.#.#.#
#-#####-#-#
#.#.#X|.#.#
#-#-#####-#
#.#.|.|.|.#
#-###-###-#
#.|.|.#.|.#
###########
";
        let input ="^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        let map = parse(input.chars().collect());
        let map_str = print_map(&map);
        println!("{}",map_str);
        assert_eq!(res,map_str.as_str());
    }

    #[test]
    fn test5() {
        let res = "#############
#.|.|.|.|.|.#
#-#####-###-#
#.#.|.#.#.#.#
#-#-###-#-#-#
#.#.#.|.#.|.#
#-#-#-#####-#
#.#.#.#X|.#.#
#-#-#-###-#-#
#.|.#.|.#.#.#
###-#-###-#-#
#.|.#.|.|.#.#
#############
";
        let input ="^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        let map = parse(input.chars().collect());
        let map_str = print_map(&map);
        println!("{}",map_str);
        assert_eq!(res,map_str.as_str());
    }

    #[test]
    fn test6() {
        let input ="^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        let res = part1(input);
        println!("res={}",res);
        assert_eq!(23,res);
    }

    #[test]
    fn test7() {
        let input ="^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        let res = part1(input);
        println!("res={}",res);
        assert_eq!(31,res);
    }


}
