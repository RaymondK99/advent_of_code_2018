use super::Part;
use std::collections::{VecDeque, HashSet};

pub fn solve(input : String, part: Part) -> String {

    let lines:VecDeque<char> = input.chars()
        .collect();

    let result = match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    };

    format!("{}",result)
}



fn part1(mut input:VecDeque<char>) -> u32 {

    let mut found_reaction = true;
    let mut next_index = 0;
    
    while found_reaction {
        found_reaction = false;

        for i in next_index..input.len()-1 {
            let j = i+1;
            let a = input[i];
            let b = input[j];
            let a_upper = a.is_ascii_uppercase();
            let b_upper = b.is_ascii_uppercase();

            if a.eq_ignore_ascii_case(&b) && a_upper != b_upper {
                input.remove(i).unwrap();
                input.remove(i).unwrap();
                //println!("Found reaction:{} and {}",ch1, ch2);
                found_reaction = true;
                if i == 0 {
                    next_index = 0;
                } else {
                    next_index = i - 1;
                }
                break;
            }
        }
    }

    input.len() as u32
}

fn part2(input:VecDeque<char>) -> u32 {
    let mut set = HashSet::new();
    input.iter().for_each(|ch| { set.insert(ch.to_ascii_lowercase());} );

    set.iter()
        .map(|ch| input.iter().filter(  move |&item| !item.eq_ignore_ascii_case(ch) ))
            .map(|perm| part1(perm.into_iter().map(|c| c.clone()).collect()))
            .min().unwrap()

}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = "dabAcCaCBAcCcaDA";
        let res = part1(input.trim().chars().collect());
        println!("{}", res);
        assert_eq!(10, res);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_05.txt");
        let res = part1(input.trim().chars().collect());
        println!("{}", res);
        assert_eq!(11298, res);

    }


    #[test]
    fn test2() {
        let input = "dabAcCaCBAcCcaDA";
        let res = part2(input.trim().chars().collect());
        println!("{}", res);
        //assert_eq!(10, res);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_05.txt");
        let res = part2(input.trim().chars().collect());
        println!("{}", res);
        assert_eq!(5148, res);

    }



}
