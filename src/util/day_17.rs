use super::Part;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{}",result)
}

fn part1(input:&str) -> i64 {
    let mut map = parse(input);
    water_path(&mut map);
    1
}


fn part2(_input:&str) -> i64 {
    2
}

fn water_path(map:&mut HashMap<(i32,i32),char>) {
    let y_max = *map.iter().map(|((_x,y),_ch)|y).max().unwrap();
    let start = (500,1);
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited.insert(start);
    parent.insert((500,1),(500,0));
    while !queue.is_empty() {
        // Pop first item
        let current = queue.pop_back().unwrap();

        // Fetch coordinates of current pos
        let (x,y) = current;

        // Only allow water to flow left/right into open space if parent in on an edge
        let (parent_x,parent_y) = *parent.get(&(x,y)).unwrap();

        // Move left/right?
        if parent_y == y {
            let down = map.get(&(x,y+1));
            let up = map.get(&(x,y-1));
            let below_parent = map.get(&(parent_x,parent_y+1));

            if down.is_some() && *down.unwrap() == '.' && up.is_some() && *up.unwrap() == '.' {

                if below_parent.is_some() && *below_parent.unwrap() != '#' {
                    continue;
                    //println!("Below parent={}",*below_parent.unwrap());
                }
            }
        }

        // Update map
        map.insert((x,y),'~');
        print_map(&map);

        if y == y_max {
            continue;
        }

        // Check neighbor nodes
        let adjacent:Vec<(i32,i32)> = vec![(x+1,y),(x-1,y),(x,y+1)];

        adjacent.iter()
            .filter(|(x1,y1)| map.contains_key(&(*x1,*y1)))
            .filter(|(x1,y1)| *map.get(&(*x1,*y1)).unwrap() == '.')
            .for_each( |(x1,y1)| {
                if !visited.contains(&(*x1,*y1)) {
                    queue.push_back((*x1,*y1));
                    visited.insert((*x1,*y1));
                    parent.insert((*x1,*y1),(x,y));
                }

            });
    }

}


fn parse_vector(input:&str) -> Vec<i32> {
    if input.contains("..") {
        let range:Vec<i32> = input.split("..").map(|c|c.parse().ok().unwrap()).collect();
        range
    } else {
        let num:i32 = input.parse().ok().unwrap();
        vec![num]
    }
}

fn parse(input:&str) -> HashMap<(i32,i32),char> {
    let mut pair_list = vec![];

    input.lines().for_each(|line|{
        let cols:Vec<&str> = line.split(',').map(|c|c.trim()).collect();
        let mut x_range = vec![];
        let mut y_range = vec![];

        for col in cols {
            //x=495 or y=2..7
            let _variable_name = col.get(0..1).unwrap();
            let range = col.get(2..).unwrap();
            let range_vec = parse_vector(range);

            if col.get(0..1).unwrap() == "x" {
                //println!("x vec:{:?}",range_vec);
                x_range = range_vec;
            } else {
                //println!("y vec:{:?}",range_vec);
                y_range = range_vec;
            }
        }

        pair_list.push((x_range,y_range));
    });

    //println!("{:?}",pair_list);

    let min_x = *pair_list.iter().map(|(x,_y)|x.first().unwrap()).min().unwrap();
    let max_x = *pair_list.iter().map(|(x,_y)|x.last().unwrap()).max().unwrap();
    let min_y = 0; //pair_list.iter().map(|(x,y)|y.first().unwrap()).min().unwrap();
    let max_y = *pair_list.iter().map(|(_x,y)|y.last().unwrap()).max().unwrap();
    let mut map = HashMap::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            map.insert((x,y), '.');
        }
    }

    pair_list.iter().for_each(|(x_range,y_range)| {
        let x_start = *x_range.first().unwrap();
        let x_end = *x_range.last().unwrap();
        let y_start = *y_range.first().unwrap();
        let y_end = *y_range.last().unwrap();

        for x in x_start..=x_end {
            for y in y_start..=y_end {
                map.insert((x,y),'#');
            }
        }
    });

    map.insert((500,0),'+');

    map
}

fn print_map(map:&HashMap<(i32,i32),char>) {
    let max_x = *map.iter().map(|((x,_y),_)|x).max().unwrap();
    let min_x = *map.iter().map(|((x,_y),_)|x).min().unwrap();
    let max_y = *map.iter().map(|((_x,y),_)|y).max().unwrap();
    let min_y = *map.iter().map(|((_x,y),_)|y).min().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.contains_key(&(x,y)) {
                print!("{}", map.get(&(x, y)).unwrap());
            }
        }
        println!();
    }
    println!("------------------------------------------------");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const INPUT:&str = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    #[test]
    fn test1() {
        let map = parse(INPUT);
        print_map(&map);
    }

    #[test]
    fn test2() {
        let res = part1(INPUT);
        println!("{}",res);
    }

}
