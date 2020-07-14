use super::Part;
use std::collections::{HashMap};
use crate::util::day_15::Position::{Wall, Empty, Goblin, Elf};
use std::cmp::Ordering;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{:?}",result)
}

#[derive(Eq, PartialEq,Copy, Clone,Debug,Ord, PartialOrd)]
enum Position {
    Empty,
    Wall,
    Goblin{id:i32,hp:i32},
    Elf{id:i32,hp:i32},
}


struct Map {
    map:HashMap<(i32,i32),Position>,
    elf_attack:i32,
    num_elfs:i32,
    num_goblins:i32,
}


impl Map {

    fn new(input: &str,elf_attack:i32) -> Map {
        let mut elf_id = 0;
        let mut goblin_id = 0;
        let mut map = HashMap::new();
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| {
                let item = match ch {
                    '#' => Wall,
                    '.' => Empty,
                    'G' => { goblin_id +=1; Goblin {id:goblin_id,hp:200}},
                    'E' => {  elf_id +=1; Elf{ id:elf_id,hp:200}},
                    _ => panic!("..."),
                };
                map.insert((x as i32, y as i32), item);
            });
        });

        Map { map: map,elf_attack:elf_attack,num_goblins:goblin_id,num_elfs:elf_id }
    }

    fn calc_hitpoints(&self) -> i32 {
        let sum:Vec<i32> = self.map.iter()
            .filter( |&(_,v)| *v != Empty && *v != Wall )
            .map(|(_,v)|{
                match v {
                    Elf {hp,..} => *hp,
                    Goblin {hp,..} => *hp,
                    _ => 0,
                }
            }).collect();

        sum.iter().sum()
    }

    fn is_finished(&self) -> bool {
        self.num_elfs == 0 || self.num_goblins == 0
    }

    fn do_moves(&mut self) -> bool {
        let mut creatures:Vec<((i32,i32),Position)> = self.map.iter()
            .filter( |&(_,v)| *v != Empty && *v != Wall ).map(|(k,v)|(k.clone(),v.clone())).collect();

        creatures.sort_by(|((x0,y0),_),((x1,y1),_)|{
            let cmp1 = y0.cmp(y1);
            if cmp1 == Ordering::Equal{
                x0.cmp(x1)
            } else {
                cmp1
            }
        });

        for (pos, creature) in creatures {

            // is Creature still alive?
            if *self.map.get(&pos).unwrap() == Empty {
                continue;
            }

            if self.is_finished() {
                return false;
            }

            let target = self.next_target(pos, creature);
            let mut next_pos = pos;
            if target.is_some() {
                // Calculate next move in order to reach target
                next_pos = self.next_move(target.unwrap(), pos);

                // Move creature
                self.map.insert(next_pos, creature);
                self.map.insert(pos, Empty);
            }


            // Can creature attack?
            let attack_pos_opt = self.can_attack(next_pos, creature);
            if attack_pos_opt.is_some() {
                let attack_pos = attack_pos_opt.unwrap();
                let target_creature = self.map.get_mut(&attack_pos).unwrap();

                match target_creature {
                    Elf {ref mut hp,..} => {
                        *hp -= 3;
                        if *hp <= 0 {
                            self.map.insert(attack_pos, Empty);
                            self.num_elfs -= 1;
                        }
                    }
                    Goblin {ref mut hp,..} => {
                        *hp -= self.elf_attack;
                        if *hp <= 0 {
                            self.map.insert(attack_pos, Empty);
                            self.num_goblins -= 1;
                        }
                    }
                    _ => panic!(",,"),
                }
            }
        }

        true
    }

    fn can_attack(&self, (x,y):(i32, i32), creature:Position) -> Option<(i32, i32)> {
        let is_elf = match creature {
            Elf {..} => true,
            _ => false,
        };

        // Next to an enemy?
        let mut enemies:Vec<(i32,(i32,i32))> = vec![(x+1,y),(x-1,y),(x,y-1),(x,y+1)].iter()
            .filter(|&n| self.map.contains_key(n) && match *self.map.get(n).unwrap() {
                Elf {..} => !is_elf,
                Goblin {..} => is_elf,
                _ => false,
            }).map(|p| {
            let hp = match *self.map.get(p).unwrap() {
                Elf {hp,..} => hp,
                Goblin {hp,..} => hp,
                _ => panic!(),
            };
            (hp, *p)
        }).collect();

        if enemies.is_empty() {
            None
        } else {
            enemies.sort_by(|(hp1,(x0,y0)),(hp2,(x1,y1))|{
                let a = (*hp1,*y0,*x0);
                let b = (*hp2,*y1,*x1);
                a.cmp(&b)
            });

            let first = *enemies.first().unwrap();
            Some(first.1)
        }
    }

    fn next_move(&self, target:(i32,i32),(x0,y0):(i32,i32)) -> (i32,i32) {

        let mut queue = vec![target];
        let mut distances = HashMap::new();
        let mut target_nodes = vec![];
        let mut best = 1000_000;
        distances.insert( target,0);

        while !queue.is_empty() {

            // Pick first element
            let pos = queue.remove(0);
            let (x, y) = pos;
            let dist = *distances.get(&pos).unwrap();

            if best < dist {
                continue;
            }

            // Next to the origin?
            let creature_adj_nodes:Vec<(i32,i32)> = vec![(x+1,y),(x-1,y),(x,y-1),(x,y+1)].iter()
                .filter(|&(a,b)| {
                    self.map.contains_key(&(*a,*b)) &&
                        *a == x0 && *b == y0
                })
                .map(|p|*p).collect();

            if !creature_adj_nodes.is_empty() {
                target_nodes.push((dist,(x,y)));
                best = std::cmp::min(best, dist);
            }

            // Next moves?
            let adjacent_nodes:Vec<(i32,i32)> = vec![(x+1,y),(x-1,y),(x,y-1),(x,y+1)].iter()
                .filter(|&n| self.map.contains_key(n) && *self.map.get(n).unwrap() == Empty)
                .filter(|&n| !distances.contains_key(n)).map(|p|*p)
                .collect();

            adjacent_nodes.iter()
                .for_each( |n| {
                    distances.insert( n.clone(), dist+1);
                    queue.push(n.clone());
                });
        }

        target_nodes.sort_by(|(dist_a,(x_a,y_a)),(dist_b,(x_b,y_b))|
            {
                let cmp_dist = dist_a.cmp(dist_b);
                if cmp_dist == Ordering::Equal {
                    let pa = (y_a, x_a);
                    let pb = (y_b, x_b);
                    pa.cmp(&pb)
                } else {
                    cmp_dist
                }
            }
        );

        let next_move = target_nodes.first().unwrap().1;
        //println!("next_move={:?}",next_move);
        next_move
    }

    fn next_target(&self, pos:(i32,i32),creature:Position) -> Option<(i32,i32)> {
        let is_elf = match creature {
            Elf {..} => true,
            _ => false,
        };

        let start_pos = pos;
        let mut queue = vec![start_pos];
        let mut distances = HashMap::new();
        let mut target_nodes = vec![];
        let mut best = 1000_000;
        distances.insert( start_pos,0);

        //println!("Eval moves for {:?}", creature);

        while !queue.is_empty() {

            // Pick first element
            let pos = queue.remove(0);
            let (x,y) = pos;
            let dist = *distances.get(&pos).unwrap();

            if dist > best {
                continue;
            }

            // Next to an enemy?
            let enemies:Vec<(i32,i32)> = vec![(x+1,y),(x-1,y),(x,y-1),(x,y+1)].iter()
                .filter(|&n| self.map.contains_key(n) && match *self.map.get(n).unwrap() {
                    Elf {..} => !is_elf,
                    Goblin {..} => is_elf,
                    _ => false,
                }).map(|p|*p).collect();

            if !enemies.is_empty() {
                target_nodes.push((dist,(x,y)));
                best = std::cmp::min(best, dist);
            }

            // Next moves?
            let adjacent_nodes:Vec<(i32,i32)> = vec![(x+1,y),(x-1,y),(x,y-1),(x,y+1)].iter()
                .filter(|&n| self.map.contains_key(n) && *self.map.get(n).unwrap() == Empty)
                .filter(|&n| !distances.contains_key(n)).map(|p|*p)
                .collect();

            adjacent_nodes.iter()
                .for_each( |n| {
                    distances.insert( n.clone(), dist+1);
                    queue.push(n.clone());
                });
        }

        target_nodes.sort_by(|(dist_a,(x_a,y_a)),(dist_b,(x_b,y_b))|
            {
                let cmp_dist = dist_a.cmp(dist_b);
                if cmp_dist == Ordering::Equal {
                    let pa = (y_a, x_a);
                    let pb = (y_b, x_b);
                    pa.cmp(&pb)
                } else {
                    cmp_dist
                }
            }
        );

        //println!("target nodes={:?}",target_nodes);
        let target = if target_nodes.len()>0 {
            let (dist,(x,y)) = target_nodes.remove(0);
            if dist > 0 {
                //println!("{:?} should move to {},{}",creature, x,y);
                Some((x, y))
            } else {
                //println!("{:?} should stay",creature);
                None
            }
        } else {
            //println!("{:?} should stay",creature);
            None
        };

        target
    }
}



fn part1(input:&str) -> i32 {
    let mut  map = Map::new(input,3);
    let mut i = 1;
    //map.print();
    loop {
        let full_move = map.do_moves();
        if map.is_finished() {
            let hp_left = map.calc_hitpoints();
            let rounds = match full_move { true => i, false => i-1 };
            println!("Game finished after {} rounds, total hp:{}",rounds,hp_left);
            return rounds * hp_left;
        }
        i += 1;
    }
}

fn part2(input:&str) -> i32 {
    let mut attack = 3;

    loop {
        let mut moves = 0;
        let mut  map = Map::new(input,attack);
        let num_elves = map.num_elfs;
        while !map.is_finished() {
            let full_move = map.do_moves();
            if full_move {
                moves += 1;
            }
        }

        if num_elves == map.num_elfs {
            return moves * map.calc_hitpoints();
        }

        attack += 1;
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let input = r"#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########";

        let res = part1(input);
        println!("{:?}",res);
        assert_eq!(27828, res);

    }

    #[test]
    fn test2() {
        let input = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

        let res = part1(input);
        println!("{:?}",res);
        assert_eq!(27730, res);

    }

    #[test]
    fn test3() {
        let input = r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

        let res = part1(input);
        println!("{:?}",res);
        assert_eq!(36334, res);
    }

    #[test]
    fn test4() {
        let input = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

        let res = part1(input);
        println!("{:?}",res);
        assert_eq!(18740, res);
    }


    #[test]
    fn test_21() {
        let input = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

        let res = part2(input);
        println!("{:?}",res);
        assert_eq!(4988, res);
    }

    #[test]
    fn test_22() {
        let input = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

        let res = part2(input);
        println!("{:?}",res);
        assert_eq!(1140, res);
    }


}
