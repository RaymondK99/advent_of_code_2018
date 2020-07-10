use super::Part;
use std::collections::{HashMap, VecDeque, HashSet};
use regex::Regex;

pub fn solve(input : String, part: Part) -> String {
    let lines:Vec<&str> = input.lines()
        .collect();

    let result = match part {
        Part::Part1 => part1(parse(lines),20),
        Part::Part2 => part2(parse(lines))
    };

    format!("{}",result)
}


fn parse(mut input:Vec<&str>) -> (VecDeque<(i64,char)>,HashMap<&str,&str>)   {
    // initial state: #..#.#..##......###...###
    // ...## => #
    let r_state = Regex::new(r"initial state: ([\.#]+)$").unwrap();
    let r_mut =  Regex::new(r"([\.#]+) => ([\.#])$").unwrap();

    let caps_initial_state = r_state.captures(input.remove(0)).unwrap();
    let initial_state = caps_initial_state.get(1).unwrap().as_str();

    let mut rules = HashMap::new();
   // let mut map = HashMap::new();
    let mut state = VecDeque::new();
    input.iter().map(|s|s.trim()).filter(|s|s.len()>0)
        .for_each(|line|{
            let caps_rule = r_mut.captures(line).unwrap();
            let key = caps_rule.get(1).unwrap().as_str();
            let value = caps_rule.get(2).unwrap().as_str();
            rules.insert(key.clone(), value.clone());
    });

    initial_state.chars().enumerate().for_each(|(x,v)| {state.push_back((x as i64,v));});
    (state, rules)
}


fn next_state((mut state, rules):(VecDeque<(i64,char)>,&HashMap<&str,&str>)) -> VecDeque<(i64,char)> {
    let mut next_state = VecDeque::new();
    // Pop empty items in the beginning
    while (*state.front().unwrap()).1 == '.' {
        state.pop_front();
    }

    // Pop empty items in the end
    while (*state.back().unwrap()).1 == '.' {
        state.pop_back();
    }

    let first = (*state.front().unwrap()).0;
    let last = (*state.back().unwrap()).0;

    state.push_front( (first-1,'.'));
    state.push_front( (first-2,'.'));
    state.push_front( (first-3,'.'));
    state.push_front( (first-4,'.'));

    state.push_back( (last+1,'.'));
    state.push_back( (last+2,'.'));
    state.push_back( (last+3,'.'));
    state.push_back( (last+4,'.'));

    for i in 2..state.len()-2 {
        let mut s = String::new();
        let index = state[i].0;
        s.push(state[i-2].1);
        s.push(state[i-1].1);
        s.push(state[i].1);
        s.push(state[i+1].1);
        s.push(state[i+2].1);

        if rules.contains_key( s.as_str()) && (*rules.get(s.as_str()).unwrap()).eq("#") {
            next_state.push_back((index as i64,'#'));
        } else {
            next_state.push_back((index as i64,'.'));
        }
    }

    next_state
}

fn part1((mut state, rules):(VecDeque<(i64,char)>,HashMap<&str,&str>), generations:usize) -> i64 {
    for _ in 0..generations {
        state = next_state((state, &rules));
    }

    state.iter()
        .filter(|(_,ch)| *ch == '#' )
        .map(|(i,_)|*i)
        .sum()
}


fn part2((mut state, rules):(VecDeque<(i64,char)>,HashMap<&str,&str>)) -> i64 {
    let mut set = HashSet::new();
    let mut generations = 0;
    let total_score;
    let mut last_score = 0;

    loop {
        state = next_state((state,&rules));
        generations += 1;

        let score:i64 = state.iter()
            .filter(|(_,ch)| *ch == '#' )
            .map(|(i,_)|*i)
            .sum();

        let s:String = state.iter().map(|(_,ch)| *ch).collect();
        if set.contains(&s) {
            let remaining_generations = 50000000000 - generations;
            let increase_per_gen = score - last_score;
            total_score = score + (remaining_generations * increase_per_gen);
            break;
        } else {
            set.insert(s);
        }
        last_score = score;
    }

    total_score
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const INPUT:&str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn test1() {
        let res = part1(parse(INPUT.lines().collect()),20);
        println!("{:?}",res);
        assert_eq!(325,res);
    }

    #[test]
    fn test2() {
        let res = part2(parse(INPUT.lines().collect()));
        println!("{:?}",res);
        assert_eq!(999999999374,res);
    }

}
