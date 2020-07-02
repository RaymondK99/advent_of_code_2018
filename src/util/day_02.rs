use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    let result = match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    };

    format!("{}",result)
}

fn part1(input:Vec<&str>) -> String {
    let mut twos = 0;
    let mut threes = 0;
    for &line in input.iter() {
        let mut vec :[i32; 128] = [0;128];
        for ch in line.chars() {
            let index = ch as usize;
            vec[index] += 1;
        }

        for freq in vec.iter() {
            if *freq == 2 {
                twos += 1;
                break;
            }
        }

        for freq in vec.iter() {
            if *freq == 3 {
                threes += 1;
                break;
            }
        }
    }

    format!("{}",twos * threes)
}

fn part2(input:Vec<&str>) -> String {
    let mut res = String::new();

    for i in 0..input.len()-1 {
        for j in i+1..input.len() {
            let word1: Vec<char> = input[i].chars().collect();
            let word2: Vec<char> = input[j].chars().collect();

            let mut diff_sum = 0;
            res.clear();
            for n in 0..word1.len() {
                if word1[n] != word2[n] {
                    diff_sum += 1;
                } else {
                    res.push(word1[n]);
                }
            }

            if diff_sum == 1 {
                return res
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
        let res = part1(input.lines().collect());
        println!("{}", res);
        assert_eq!("12", res);
    }

    #[test]
    fn test2() {
        let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        let res = part2(input.lines().collect());
        println!("{}", res);
        assert_eq!("fgij", res);
    }
}
