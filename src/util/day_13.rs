use super::Part;
use std::collections::{HashMap};
use std::cmp::Ordering;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{:?}",result)
}

#[derive(Eq, PartialEq,Copy, Clone,Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug,Clone,Copy)]
struct Cart {
    x:i32,
    y:i32,
    dir:Direction,
    state:i32,
    num:i32,
}

struct Map {
    map:HashMap<(i32,i32),char>,
    carts:Vec<Cart>,
}

impl Cart {

    fn next_pos(&mut self, map:&HashMap<(i32,i32),char>)  {
        let (x,y) = (self.x, self.y);
        let next_pos = match self.dir {
            Direction::UP => (x, y-1),
            Direction::DOWN => (x, y+1),
            Direction::LEFT => (x-1, y),
            Direction::RIGHT => (x+1, y),
        };

        let next_ch = *map.get(&next_pos).unwrap();
        let next_dir = if (next_ch == '/' && self.dir == Direction::LEFT) || (next_ch == '\\' && self.dir == Direction::RIGHT) {
            Direction::DOWN
        } else if (next_ch == '/' && self.dir == Direction::UP) || (next_ch == '\\' && self.dir == Direction::DOWN) {
            Direction::RIGHT
        } else if (next_ch == '\\' && self.dir == Direction::LEFT) || (next_ch == '/' && self.dir == Direction::RIGHT) {
            Direction::UP
        } else if (next_ch == '/' && self.dir == Direction::DOWN) || (next_ch == '\\' && self.dir == Direction::UP) {
            Direction::LEFT
        } else if next_ch == '+' {
            // Intersection
            let dirs = vec![Direction::UP, Direction::RIGHT, Direction::DOWN, Direction::LEFT];
            let next_dir = if self.state == 0 {
                // Turn left
                let mut i:i32 = 0;
                while dirs[i as usize] != self.dir {
                    i += 1;
                }
                i -= 1;
                if i < 0 {
                    i = 3;
                }
                dirs[i as usize]
            } else if self.state == 1 {
                // Cont.
                self.dir
            } else {
                // Turn right
                let mut i = 0;
                while dirs[i] != self.dir {
                    i += 1;
                }
                i = (i+1) % 4;
                dirs[i]
            };
            self.state = (self.state + 1) % 3;
            next_dir
        } else {
            self.dir.clone()
        };

        self.dir = next_dir;
        self.x = next_pos.0;
        self.y = next_pos.1;
    }
}
impl Map {
    fn new(input:&str) -> Map {
        let mut map = HashMap::new();
        input.lines().enumerate().for_each(|(y,line)|{
            line.chars().enumerate().for_each(|(x,ch)|{
                map.insert((x as i32,y as i32),ch);
            });
        });
        let mut n = 0;
        let carts:Vec<Cart> = map.iter().filter(|&((_,_),ch)|
        *ch == '<' ||*ch == '>' || *ch == '^' || *ch == 'v')
            .map(|((x,y),ch)| {
                n +=1;
                let dir = match ch {
                    '^' => Direction::UP,
                    'v' => Direction::DOWN,
                    '<' => Direction::LEFT,
                    '>' => Direction::RIGHT,
                    _ => panic!(".."),
                };
                Cart{x:*x,y:*y,state:0,num:n,dir:dir}
            }).collect();

        Map{map:map,carts:carts}
    }


    fn run(&mut self, remove_crashed:bool) -> (i32,i32) {
        let mut to_move:Vec<Cart> = self.carts.iter_mut().map(|c| (*c).clone()).collect();
        let mut moved: Vec<Cart> = vec![];

        loop {

            // Move the first carts from the top
            to_move.sort_by(|a, b| {
                let ord1 = a.y.cmp(&b.y);
                let ord2 = a.x.cmp(&b.x);
                if ord1 == Ordering::Equal {
                    ord2
                } else {
                    ord1
                }
            });

            // Move each cart one by one
            while !to_move.is_empty() {
                let mut cart = to_move.remove(0);
                cart.next_pos(&self.map);

                // Check collisions with other carts
                let mut crash = false;

                for n in 0..to_move.len() {
                    if cart.x == to_move[n].x && cart.y == to_move[n].y {
                        // Crash
                        if remove_crashed {
                            to_move.remove(n);
                            crash = true;
                            break;
                        } else {
                            return (cart.x, cart.y);
                        }
                    }
                }

                for n in 0..moved.len() {
                    if cart.x == moved[n].x && cart.y == moved[n].y {
                        // Crash
                        if remove_crashed {
                            moved.remove(n);
                            crash = true;
                            break;
                        } else {
                            return (cart.x, cart.y);
                        }
                    }
                }

                if !crash {
                    moved.push(cart);
                }
            }

            // Push back carts in "to be moved" list
            while !moved.is_empty() {
                to_move.push(moved.pop().unwrap());
            }

            // Check for exit critera
            if remove_crashed && to_move.len() == 1 {
                let cart = to_move.remove(0);
                return (cart.x, cart.y);
            }
        }
    }
}



fn part1(input:&str) -> (i32,i32) {
    let mut map = Map::new(input);
    map.run(false)

}

fn part2(input:&str) -> (i32,i32) {
    let mut map = Map::new(input);
    map.run(true)
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let input = r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";

        let res = part1(input);
        println!("{:?}",res);
        assert_eq!((7,3), res);

    }

    #[test]
    fn test2() {
        let input = r"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";

        let res = part2(input);
        println!("{:?}",res);
        assert_eq!((6,4), res);

    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_13.txt");
        let res = part1(input);
        println!("{:?}",res);
        assert_eq!((100,21), res);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_13.txt");
        let res = part2(input);
        println!("{:?}",res);
        assert_eq!((113,109), res);
    }


}
