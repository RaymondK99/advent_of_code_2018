use super::Part;
use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{}",result)
}

fn perform_generation(map:&mut HashMap<(i32,i32),char>) {
    let mut updates = vec![];
    map.iter().for_each(|(pos, ch)| {
        // Get Adjacent nodes
        let (x, y) = *pos;
        let adjacent_pos = vec![(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1), (x - 1, y - 1), (x - 1, y + 1), (x + 1, y + 1), (x + 1, y - 1)];
        let adjacent_nodes: Vec<char> = adjacent_pos.iter().map(|(x, y)| map.get(&(*x, *y))).filter(|item|item.is_some()).map(|c| *c.unwrap()).collect();

        let trees = adjacent_nodes.iter().filter(|&ch| *ch == '|').count();
        let lumber_yard = adjacent_nodes.iter().filter(|&ch| *ch == '#').count();

        if *ch == '.' && trees > 2 {
            updates.push(((x, y), '|'));
        } else if *ch == '|' && lumber_yard > 2 {
            updates.push(((x, y), '#'));
        } else if *ch == '#' && (lumber_yard == 0 || trees == 0) {
            updates.push(((x, y), '.'));
        }
    });

    while !updates.is_empty() {
        let ((x, y), ch) = updates.pop().unwrap();
        map.insert((x, y), ch);
    }
}

fn part1(input:&str) -> usize {
    let mut map = parse(input);

    for _ in 0..10 {
        perform_generation(&mut map);
    }

    map.iter().filter(|&(_,ch)| *ch == '|').count() * map.iter().filter(|&(_,ch)| *ch == '#').count()
}


fn part2(input:&str) -> usize {
    let mut map = parse(input);
    2
}

fn parse(input:&str) -> HashMap<(i32,i32),char> {
    let mut map = HashMap::new();

    input.lines().enumerate().for_each(|(y,line)|{
        line.chars().enumerate().for_each(|(x,ch)|{
            map.insert((x as i32,y as i32),ch);
        });
    });


    map
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    const INPUT:&str = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    const INPUT_REAL:&str = "..|.#.....|.....##...#|..||...#.|#.#.||...#.....#|
#...|.....|.|.#....#.|.|...|.#.||..#....#||.#||#..
.......|.....|....|...||.|##|.#|.....|#..##.|...||
.......#.##|...|.|.#....||.#....|#.....#..|..|....
|...#...##.|#...|...#..|#|#.|...#...||#.|..||.....
...#...#..|..#.||...|..#..#...#.|#||#|...#|..#..|.
....|..|...#||.|.|#|...|#....|.#...##..|..|#.|##..
##.##..|#|...##......###.......||.|...|..|........
..#|....###......|..##.|..|...##..|.|...###.#..#..
#|#.|#.|#.#.......#....#...#.|...#||.#||||......|.
|#|.........#|..|.||........#...|.|#..#.#..##.##..
.|...#|..||.|...|...|#|.||#..#.||..#|#.|..#.....#.
||.....#...||......#|..##....||#.|.|...#..||.#.||.
......#.##||.#.|..#.....|.......|.#.....||..|#.#|.
.....##|#....#.#.#.#|..#.#.....#..|.#.|##||.#...##
##.#.#|.###.#.#...|.|......|#...#......|.|........
##....|##.|...#..#......#.|....|........#.......#.
#..#..#.#......#|#.|#....||.#|..#..|.#.|.#.||....|
|.#.....|....#..#.#..#.....||.#..|....#|.#...##...
.###....#...|.|.|.#.###.|..#|....|..|......|......
....|..#...........##.##...|........#...#..#....||
.#.|#......#....#|||.#......#|#.|..|#|..#.|||..|..
#.|#.#||.|#.##||.#..#..|#...#.......|..#|#..##|.#.
#||##..|.................|...|#...#...|#.#..|..|.|
....|.#.|.#.|.|.##.|...|.#.|#.........#..|..#||.#.
....#.|.||..|..|.#.....#.##...#....||#|#..|.|##..|
..#.||#...#.#....|...........#.....#.#|..##.......
|........#..#|.#|....#.|..#..#.....###|#||..#...#.
...#.|...|##..||..|..#||..|.#.|.|..#.|.|.|#.|..|#|
##|.#|#|||..|........|#...|#.|.#..||#||.#|......#.
.....|#..##...|.......#.#.....#..|#..||...|###...|
..|###.#...|....#.|||.|#.|#|.#|....#.|.|..|#...#..
...|||##||...#.##|...||..|.#.#.||.||.|.#...|.|.|..
.#....#..|#...||........|.....|.|...#..#.|...|.|.|
.|...|#....|.|.#.||##|#|.#....#.#|#...|#|.#.|.|...
##|....||.|...|...#..#......#..#..#.|.#.#|##|...##
||...#||...#|..|.|#.#...#.#.|#.|........#....#..||
##...|#|..|....|......#..||...#|.|#|.......#....|.
##.#|.#...###.##...#.|......#.#|#|..||.#..#.#|.||.
###.....|##.........|.|..|||.......#..#....|..#.|.
..###||#..|#......|.|...|##..|.....#..#....#..|.#.
||....|.||..#..|.|||#|....#.#.|..|...#.|.......|.|
.....#.........##||.....#.|....#.##..#.##|.#|..||#
#..|#|||.||....|..#.#..|.|....#.|.||##.|.|.#.#....
##|....#.|.#|.|##..#.|#....|.|..#.....#|..#..|.#|#
.#|#.#..#..#....#......|.......#|.#.#|#..#....#...
......|..|##.#..|.....|#...#.#.|..|.#.|##.........
#...#....||.|#.....|..#|.......|.|..#..#.|...|#...
..#|.|##.#.#.|.#...#||.##...|......##.#.|..#||#.#.
..|.###.#.#|.|..#.#..#||.|..#|#..#...#.|.#|....|..";


    #[test]
    fn test1() {
        let res = part1(INPUT);
        println!("res={}",res);
    }

    #[test]
    fn test_part1() {
        let res = part1(INPUT_REAL);
        println!("res={}",res);
    }

}