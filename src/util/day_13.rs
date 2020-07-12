use super::Part;
use std::collections::{HashMap, HashSet};
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

#[derive(Debug)]
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

    fn print_state(&self) {
        let max_x = self.map.keys().map(|(x,_)| *x).max().unwrap();
        let max_y = self.map.keys().map(|(y,_)| *y).max().unwrap();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let item = self.map.get(&(x,y));
                let cart = self.carts.iter().find(|&c| c.x == x && c.y == y);
                if item.is_some(){
                    if cart.is_some() {
                        print!("{}",cart.unwrap().num);
                    } else {
                        print!("{}", *item.unwrap());
                    }
                }
            }
            println!();
        }
    }

    fn run(&mut self) -> (i32,i32) {

        loop {

            let mut list:Vec<&mut Cart> = self.carts.iter_mut().collect();
            list.sort_by(|a,b| {
                let ord1 = a.y.cmp(&b.y);
                let ord2 = a.x.cmp(&b.x);
                if ord1 == Ordering::Equal {
                    ord2
                } else {
                    ord1
                }
            });

            for _ in 0..list.len() {
                let cart = list.remove(0);
                cart.next_pos(&self.map);

                // Check collisions with other carts
                let mut set = HashSet::new();
                for n in 0..list.len() {
                    set.insert((list[n].x,list[n].y));
                }

                // Is next position a collision with other carts?
                if set.contains(&(cart.x, cart.y)) {
                    return (cart.x,cart.y);
                }

                list.push(cart);
            }
        }
    }
}



fn part1(input:&str) -> (i32,i32) {
    let mut map = Map::new(input);
    map.run()

}

fn part2(input:&str) -> (i32,i32) {
    (0,0)
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


}
