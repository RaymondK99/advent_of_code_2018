use super::Part;
use std::collections::{HashMap};

pub fn solve(input : String, part: Part) -> String {
    let result = match part {
        Part::Part1 => part1(300,300,input.as_str().trim().parse().ok().unwrap(),3),
        Part::Part2 => part2(input.as_str().trim().parse().ok().unwrap())
    };

    result
}

fn calc_power(pos:(i32,i32),serial_no:i32) -> i32 {
    let (x,y) = pos;
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += serial_no;
    power *= rack_id;
    power = (power / 100) % 10;
    power - 5
}

fn gen_map(width:i32,height:i32, serial_no:i32) -> HashMap<(i32,i32),i32> {
    let mut map = HashMap::new();
    for x in 1..width+1 {
        for y in 1..height+1{
            map.insert((x,y), calc_power((x,y),serial_no));
        }
    }
    map
}

fn calc_power_list(map:&HashMap<(i32,i32),i32>,width:i32,height:i32,size:i32) -> Vec<(i32,(i32,i32))> {
    let mut power_list = vec![];
    let mut rows = HashMap::new();

    for y in 1..height+1 {
        // create rows
        for x in 1..width-(size-1) {
            let mut sum = 0;
            let mut x2 = x;
            while x2 < x+size {
                let value  = map.get(&(x2,y)).unwrap();
                sum += *value;
                x2 +=1;
            }
            // Insert row
            rows.insert( (x,y), sum);
        }
    }

    for y in 1..height-(size-1) {
        for x in 1..width-(size-1) {
            let mut sum = 0;
            let mut y2 = y;
            while y2 < y+size {
                sum += *rows.get(&(x,y2)).unwrap();
                y2 += 1;
            }
            power_list.push((sum,(x,y)));
        }
    }

    power_list
}

fn part1(width:i32, height:i32, serial_no:i32,size:i32) -> String {
    let map = gen_map(width,height, serial_no);
    let power_list = calc_power_list(&map, width, height, size);

    let (_,(x,y)) = *power_list.iter().max_by(|a,b| a.0.cmp(&b.0)).unwrap();
    format!("{},{}",x,y)
}

fn part2(serial_no:i32) -> String {
    let grid_size = 300;
    let mut res= vec![];
    let map = gen_map(grid_size,grid_size, serial_no);
    for size in 1..299 {
        //println!("Calc for size:{}",size);
        let power_list = calc_power_list(&map, grid_size, grid_size, size);
        let (power,(x,y)) = *power_list.iter().max_by(|a,b| a.0.cmp(&b.0)).unwrap();
        //println!("power={}, ({},{}), size={}",power,x,y,size);
        res.push( (power,(x,y),size) );
        if power < 0 {
            break;
        }
    }

    let (_power,(x,y),size) = res.iter().max_by( |a,b| a.0.cmp(&b.0)).unwrap();
    format!("{},{},{}",x,y,size)
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_map_1() {
        let res1 = *gen_map(300,300,57).get(&(122,79)).unwrap();
        let res2 = *gen_map(300,300,39).get(&(217,196)).unwrap();
        let res3 = *gen_map(300,300,71).get(&(101,153)).unwrap();

        println!("{}",res1);
        println!("{}",res2);
        println!("{}",res3);
    }

    #[test]
    fn test1() {
        let res = part1(300,300,18,3);
        println!("{}",res);
        assert_eq!("33,45",res)
    }

    #[test]
    fn test2() {
        let res = part1(300,300,42,3);
        println!("{}",res);
        assert_eq!("21,61",res);
    }

    #[test]
    fn test_part1() {
        let str = "9005";
        let res = part1(300,300, str.parse().ok().unwrap(),3);
        println!("{:?}",res);
        assert_eq!("20,32",res);
    }


}
