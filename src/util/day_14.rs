use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str().trim().parse().ok().unwrap()),
        Part::Part2 => part2(input.as_str().trim().parse().ok().unwrap())
    };

    format!("{}",result)
}

fn part1(input:i64) -> u64 {
    let mut recipes = vec![3,7];
    let mut first = 0;
    let mut second = 1;
    let mut result:u64 = 0;

    loop {
        // Add new recipes
        let sum = recipes[first] + recipes[second];
        if sum >= 10 {
            recipes.push(1);
        }

        recipes.push(sum % 10);

        first = (first + recipes[first]+1) % recipes.len();
        second = (second + recipes[second]+1) % recipes.len();

        if recipes.len() >= (input as usize + 10) {
            break;
        }
    }

    for i in input..input+10 {
        result *= 10;
        result += recipes[i as usize] as u64;
    }

    result
}

fn part2(input:i64) -> u64 {
    let mut recipes = vec![3,7];
    let mut first = 0;
    let mut second = 1;
    let result;

    let pattern:Vec<usize> = input.to_string().as_bytes().iter().map(|b| *b as usize - 48).collect();

    loop {
        // Add new recipes
        let sum = recipes[first] + recipes[second];
        if sum >= 10 {
            recipes.push(1);
        }

        if recipes.ends_with(pattern.as_slice()) {
            result = recipes.len() - pattern.len();
            break;
        }

        recipes.push(sum % 10);

        if recipes.ends_with(pattern.as_slice()) {
            result = recipes.len() - pattern.len();
            break;
        }

        first = (first + recipes[first]+1) % recipes.len();
        second = (second + recipes[second]+1) % recipes.len();

    }

    result as u64
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = 9;
        let res = part1(input);
        println!("{}", res);
        assert_eq!(5158916779, res);
    }

    #[test]
    fn test2() {
        let input = 2018;
        let res = part1(input);
        println!("{}", res);
        assert_eq!(5941429882, res);
    }

    #[test]
    fn test_part1() {
        let input = 380621;
        let res = part1(input);
        println!("{}", res);
        assert_eq!(6985103122, res);
    }

    #[test]
    fn test_21() {
        let input = 51589;
        let res = part2(input);
        println!("{}", res);
        assert_eq!(9, res);
    }

    #[test]
    fn test_22() {
        let input = 5941429882;
        let res = part2(input);
        println!("{}", res);
        assert_eq!(2018, res);
    }

}
