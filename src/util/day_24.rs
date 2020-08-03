use super::Part;
use regex::{Regex};
use std::cmp::Ordering;
use crate::util::day_24::GroupType::{Infection, ImmuneSystem};

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{}",result)
}

#[derive(Debug,Copy, Clone,Eq, PartialEq)]
enum GroupType {
    Infection,
    ImmuneSystem,
}

#[derive(Debug,Eq, PartialEq, Clone)]
struct Group {
    no:usize,
    units:i32,
    hp:i32,
    power:i32,
    damage_type:String,
    weakness:Vec<String>,
    immune:Vec<String>,
    initiative:i32,
    group_type:GroupType,
}

impl Group {
    fn new(units:i32, hp:i32, power:i32, damage_type:&str, weakness:&str, immune:&str, init:i32, no:usize, group_type:GroupType) -> Group {

        let list_weak = if !weakness.is_empty() {
            weakness.split(',').map(|s| String::from(s.trim())).collect()
        } else {
            vec![]
        };

        let list_immune = if !immune.is_empty() {
            immune.split(',').map(|s| String::from(s.trim())).collect()
        } else {
            vec![]
        };

        Group{no,units,hp:hp,power:power,damage_type:String::from(damage_type),initiative:init,weakness:list_weak,immune:list_immune, group_type }
    }

    fn effective_power(&self) -> i32 {
        &self.power * &self.units
    }

    fn attack_damage(&self, other:&Group) -> i32 {
        if other.is_immune(&self.damage_type) {
            0
        } else if other.is_weak(&self.damage_type) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }

    fn is_immune(&self,damage_type:&String) -> bool {
        for it in self.immune.iter() {
            if damage_type.eq(it) {
                return true;
            }
        }
        false
    }

    fn is_weak(&self,damage_type:&String) -> bool {
        for it in self.weakness.iter() {
            if damage_type.eq(it) {
                return true;
            }
        }
        false
    }

    fn sort_by_attack_order(&self, other:&Group) -> Ordering {
        let power_cmp = other.effective_power().cmp(&self.effective_power());
        let cmp  = if power_cmp == Ordering::Equal {
            other.initiative.cmp(&self.initiative)
        } else {
            power_cmp
        };

        cmp
    }
}

fn parse_input(input:&str) -> Vec<Group> {
    let mut parse_immune_system = true;
    let mut groups = vec![];
    let mut immune_group_no = 1;
    let mut infection_group_no = 1;

    input.lines().filter(|s|s.trim().len()>0).for_each(|line| {
        if line.contains("Immune System:") {
            parse_immune_system = true;
        } else if line.contains("Infection:") {
            parse_immune_system = false;
        } else {
            if parse_immune_system {
                groups.push(parse_line(line,immune_group_no, GroupType::ImmuneSystem));
                immune_group_no += 1;
            } else {
                groups.push(parse_line(line,infection_group_no, GroupType::Infection));
                infection_group_no += 1;
            }
        }
    });

    groups
}

fn parse_line(line:&str, no:usize, group_type:GroupType) -> Group {
    /*
    17 units each with 5390 hit points (weak to radiation, bludgeoning) with
 an attack that does 4507 fire damage at initiative 2
    */

    let r1 = Regex::new(r"([\d]+) units each with ([\d]+) hit points (\(([a-z ,;]+)\) )?with an attack that does (\d+) ([a-z]+) damage at initiative (\d+)$").unwrap();
    let r_ability = Regex::new(r"(weak|immune) to ([a-z,; ]+)").unwrap();

    let caps = r1.captures(line).unwrap();

    let units:i32 = caps.get(1).unwrap().as_str().parse().ok().unwrap();
    let hit_points:i32 = caps.get(2).unwrap().as_str().parse().ok().unwrap();
    let abilities = if caps.get(4).is_some() {
        caps.get(4).unwrap().as_str()
    } else {
        ""
    };

    let power:i32 = caps.get(5).unwrap().as_str().parse().ok().unwrap();
    let damage_type = caps.get(6).unwrap().as_str();
    let initiative:i32 = caps.get(7).unwrap().as_str().parse().ok().unwrap();

    let mut weak  = String::new();
    let mut immune  = String::new();

    abilities.split(';').map(|s|s.trim()).for_each(|s| {
        if s.starts_with("weak") {
            weak.push_str(r_ability.captures(s).unwrap().get(2).unwrap().as_str());
        } else if s.starts_with("immune") {
            immune.push_str(r_ability.captures(s).unwrap().get(2).unwrap().as_str());
        }
    });

    Group::new(units, hit_points, power, damage_type, weak.as_str(), immune.as_str(), initiative, no, group_type)
}

fn get_target_selection(list:&Vec<Group>) -> Vec<(usize,usize)>{
    let mut targets = vec![];
    let mut attack_pair = vec![];

    for (attacker_index,group) in list.iter().enumerate() {
        // Gen list of opponents?
        let mut opponents:Vec<(usize,&Group)> = list.iter().enumerate()
            .filter(| &(_,a)| !a.eq(group) && a.group_type.ne(&group.group_type))
            .filter( |&(_,a)| group.attack_damage(a) > 0)
            .filter( |&(_,a)| !targets.contains(&a))
            .collect();

        // Sort list to select the target
        opponents.sort_by( |&(_,a),&(_,b)| {
            let cmp_attack = group.attack_damage(b).cmp(&group.attack_damage(a));
            let cmp_power = b.effective_power().cmp(&a.effective_power());
            let cmp_init = b.initiative.cmp(&a.initiative);

            if cmp_attack == Ordering::Equal {
                if cmp_power == Ordering::Equal {
                    cmp_init
                } else {
                    cmp_power
                }
            } else {
                cmp_attack
            }
        });

        let first = opponents.first();
        if first.is_some() {
            let (targt_index,target) = *first.unwrap();
            targets.push(target);
            attack_pair.push((attacker_index,targt_index) );
        }
    }

    // Sort attack pair by the attackers initiative
    attack_pair.sort_by( |(a1,_),(a2,_)|{
        let attacker1:&Group = list.get(*a1).unwrap();
        let attacker2:&Group = list.get(*a2).unwrap();

        attacker1.initiative.cmp(&attacker2.initiative).reverse()
    });

    attack_pair
}

fn run_combat(mut all:Vec<Group>) -> (i32,i32) {
    loop {
        let mut total_kills = 0;

        // Sort starting order
        all.sort_by(|a, b| a.sort_by_attack_order(b));

        // Target selection
        let attack_pairs = get_target_selection(&all);

        // Perform battle
        for (index_attacker, index_target) in attack_pairs {
            let damage = all[index_attacker].attack_damage(&all[index_target]);
            let kills = std::cmp::min(damage / all[index_target].hp, all[index_target].units);
            all[index_target].units -= kills;
            total_kills += kills;
        }

        // Remove dead groups
        let mut cnt = all.len();
        let mut i = 0;
        while i < cnt {
            if all[i].units == 0 {
                all.remove(i);
                cnt -= 1;
            } else {
                i += 1;
            }
        }

        // Calculate units
        let infection_units: i32 = all.iter().filter(|g| g.group_type == Infection).map(|g| g.units).sum();
        let immune_units: i32 = all.iter().filter(|g| g.group_type == ImmuneSystem).map(|g| g.units).sum();

        if infection_units == 0 {
            return (immune_units,0);
        } else if immune_units == 0 {
            return (0,infection_units);
        } else if total_kills == 0 {
            // Draw
            return (immune_units,infection_units)
        }
    }
}

fn part1(input:&str) -> i32 {
    let all = parse_input(input);
    run_combat(all).1
}


fn part2(_input:&str) -> i32 {
    let mut groups = parse_input(_input);

    loop {
        let (immune,infection) = run_combat(groups.clone());
        if infection == 0 {
            return immune
        }

        groups.iter_mut().filter(|g| g.group_type == ImmuneSystem).for_each(|g|g.power += 1);
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::util::day_24::GroupType::Infection;

    const TEST_INPUT:&str = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    const REAL_INPUT:&str = "Immune System:
3115 units each with 1585 hit points (weak to slashing, bludgeoning) with an attack that does 4 slashing damage at initiative 7
3866 units each with 6411 hit points (weak to cold, radiation; immune to fire) with an attack that does 14 slashing damage at initiative 11
40 units each with 10471 hit points (weak to bludgeoning, slashing; immune to cold) with an attack that does 2223 cold damage at initiative 3
1923 units each with 2231 hit points (weak to slashing, fire) with an attack that does 10 bludgeoning damage at initiative 13
4033 units each with 10164 hit points (immune to slashing) with an attack that does 22 radiation damage at initiative 5
36 units each with 5938 hit points (weak to bludgeoning, cold; immune to fire) with an attack that does 1589 slashing damage at initiative 4
2814 units each with 7671 hit points (weak to cold) with an attack that does 21 radiation damage at initiative 15
217 units each with 9312 hit points (immune to slashing) with an attack that does 345 radiation damage at initiative 8
38 units each with 7686 hit points (weak to bludgeoning) with an attack that does 1464 radiation damage at initiative 14
5552 units each with 3756 hit points (weak to slashing) with an attack that does 6 fire damage at initiative 10

Infection:
263 units each with 28458 hit points (weak to fire, radiation) with an attack that does 186 cold damage at initiative 9
137 units each with 29425 hit points (immune to fire; weak to cold) with an attack that does 367 radiation damage at initiative 1
2374 units each with 41150 hit points (immune to bludgeoning, slashing, radiation; weak to cold) with an attack that does 34 bludgeoning damage at initiative 6
1287 units each with 24213 hit points (immune to fire) with an attack that does 36 cold damage at initiative 17
43 units each with 32463 hit points (weak to radiation; immune to slashing, bludgeoning) with an attack that does 1347 fire damage at initiative 16
140 units each with 51919 hit points (weak to slashing, bludgeoning) with an attack that does 633 fire damage at initiative 12
3814 units each with 33403 hit points with an attack that does 15 fire damage at initiative 19
3470 units each with 44599 hit points (weak to slashing, radiation) with an attack that does 23 radiation damage at initiative 18
394 units each with 36279 hit points with an attack that does 164 fire damage at initiative 20
4288 units each with 20026 hit points with an attack that does 7 radiation damage at initiative 2";

    #[test]
    fn test1() {
        let line1 = "17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2";
        let line2 = "4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let line3 = "4288 units each with 20026 hit points with an attack that does 7 radiation damage at initiative 2";
        let group1 = parse_line(line1, 1, Infection);
        let group2 = parse_line(line2, 2, Infection);
        let group3 = parse_line(line3, 3, Infection);

        println!("{:?}",group1);
        println!("{:?}",group2);
        println!("{:?}",group3);
    }

    #[test]
    fn test2() {
        let all = parse_input(TEST_INPUT);
        println!("{:?}",all);

    }

    #[test]
    fn test3() {
        let res = part1(TEST_INPUT);
        println!("{}",res);
        assert_eq!(5216,res);
    }

    #[test]
    fn test_part1() {
        let res = part1(REAL_INPUT);
        println!("{}",res);
        assert_eq!(14854,res);
    }


    #[test]
    fn test4() {
        let res = part2(TEST_INPUT);
        println!("{}",res);
        assert_eq!(51,res);
    }

}
