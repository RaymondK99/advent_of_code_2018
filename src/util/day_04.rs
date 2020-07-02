use super::Part;
use std::collections::HashMap;
use regex::Regex;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    let result = match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    };

    format!("{}",result)
}


fn get_map(input:&Vec<&str>) -> HashMap<u32,[u32;60]> {
    let mut map = HashMap::new();
    // [1518-10-03 00:47] falls asleep
    let r = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.*)$").unwrap();

    // Guard #659 begins shift
    let r2 = Regex::new(r"Guard #(\d+) begins shift$").unwrap();

    let mut guard_no:u32 = 0;
    let mut fell_asleep:(u32, u32) = (0, 0);

    for &line in input.iter() {
        let caps = r.captures(line).unwrap();

        if caps.len() == 7 {
            let _: u32 = caps.get(1).unwrap().as_str().parse().ok().unwrap();
            let _: u32 = caps.get(2).unwrap().as_str().parse().ok().unwrap();
            let _: u32 = caps.get(3).unwrap().as_str().parse().ok().unwrap();
            let h: u32 = caps.get(4).unwrap().as_str().parse().ok().unwrap();
            let m: u32 = caps.get(5).unwrap().as_str().parse().ok().unwrap();
            let s = caps.get(6).unwrap().as_str();

            //println!("{}-{}-{} {}:{} => {}",yyyy,mm,dd,h,m,s);

            // Begins shift
            if s.contains("Guard") {
                let guard_cap = r2.captures(s).unwrap();
                guard_no = guard_cap.get(1).unwrap().as_str().parse().ok().unwrap();

            } else if s.contains("falls asleep") {
                fell_asleep = (h, m);
            } else if s.contains("wakes up") {
                let entry = map.entry(guard_no).or_insert([0;60]);
                let (h_start,m_start) = fell_asleep;

                //println!("Guard {} feel asleep {}:{} and woke up {}:{}",guard_no, h_start, m_start, h,m);
                let mut hour = h_start;
                let mut min = m_start;
                while hour != h || min != m {

                    entry[min as usize] += 1;

                    min = (min + 1) % 60;

                    if min == 0 {
                        hour = (hour + 1) % 24;
                    }
                }
            }
        }
    }

    map
}

fn part1(mut input:Vec<&str>) -> u32 {
    input.sort();
    let map = get_map(&input);

    let mut max_guard_no = 0;
    let mut max_min = 0;
    let mut max_min_index = 0;
    for (num, mins) in map.iter() {
        let sum:u32 = mins.iter().sum();
        //println!("Guard {} sleeps for {} min", num, sum);
        if sum > max_min {
            max_min = sum;
            max_guard_no = *num;

            let mut max_min_value = 0;
            for i in 0..mins.len() {
                if mins[i] > max_min_value {
                    max_min_value = mins[i];
                    max_min_index = i as u32;
                }
            }
        }
    }
    //println!("Guard {} sleeps the most at min {}", max_guard_no, max_min_index);

    max_guard_no * max_min_index

}

fn part2(input:Vec<&str>) -> u32 {
    2
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const input:&str = "[1518-10-03 00:47] falls asleep
[1518-07-26 23:50] Guard #487 begins shift
[1518-06-22 00:48] wakes up
[1518-08-21 00:30] falls asleep
[1518-11-21 00:55] wakes up
[1518-05-30 00:06] falls asleep
[1518-04-09 00:44] wakes up
[1518-07-22 00:58] wakes up
[1518-06-15 00:57] wakes up
[1518-09-13 00:31] wakes up
[1518-11-03 00:48] falls asleep
[1518-03-09 00:02] Guard #1123 begins shift
[1518-05-01 00:51] falls asleep
[1518-06-21 00:19] falls asleep
[1518-06-18 00:48] wakes up
[1518-05-24 23:59] Guard #2971 begins shift
[1518-09-27 00:45] falls asleep
[1518-06-06 00:44] wakes up
[1518-05-11 23:58] Guard #641 begins shift
[1518-04-21 00:01] Guard #1889 begins shift
[1518-08-30 00:27] falls asleep
[1518-06-27 00:45] falls asleep
[1518-06-15 00:00] Guard #1993 begins shift
[1518-10-04 00:00] Guard #659 begins shift
[1518-10-31 00:01] Guard #1993 begins shift
[1518-04-19 00:04] Guard #2917 begins shift
[1518-05-19 00:59] wakes up
[1518-09-02 00:27] wakes up
[1518-08-12 23:58] Guard #2833 begins shift
[1518-09-28 00:24] wakes up
[1518-08-10 00:56] wakes up
[1518-03-07 00:57] wakes up
[1518-07-31 00:35] falls asleep
[1518-04-21 00:27] falls asleep
[1518-06-26 23:51] Guard #1471 begins shift
[1518-09-01 00:20] falls asleep
[1518-09-05 00:28] falls asleep
[1518-06-26 00:55] falls asleep
[1518-06-11 00:52] wakes up
[1518-03-15 00:57] wakes up
[1518-09-18 00:27] falls asleep
[1518-04-13 00:57] wakes up
[1518-06-28 00:48] wakes up
[1518-03-18 00:51] wakes up
[1518-04-27 00:37] falls asleep
[1518-04-24 00:35] falls asleep
[1518-08-10 00:46] wakes up
[1518-07-14 23:57] Guard #2161 begins shift
[1518-11-14 00:47] wakes up
[1518-08-27 00:13] falls asleep
[1518-11-12 00:45] wakes up
[1518-03-26 00:23] falls asleep
[1518-04-21 00:31] wakes up
[1518-11-09 23:58] Guard #373 begins shift
[1518-05-05 00:50] wakes up
[1518-03-15 00:56] falls asleep
[1518-06-07 00:46] wakes up
[1518-04-21 00:42] wakes up
[1518-08-19 00:46] wakes up
[1518-03-06 00:57] falls asleep
[1518-06-30 00:38] wakes up
[1518-08-26 00:37] falls asleep
[1518-11-17 23:57] Guard #1489 begins shift
[1518-06-11 00:00] Guard #487 begins shift
[1518-09-26 00:59] wakes up
[1518-03-25 00:35] falls asleep
[1518-04-26 23:58] Guard #941 begins shift
[1518-08-27 00:38] falls asleep
[1518-05-18 00:53] wakes up
[1518-06-20 00:28] wakes up
[1518-05-05 00:45] falls asleep
[1518-07-24 00:03] Guard #1811 begins shift
[1518-04-12 00:30] wakes up
[1518-08-02 00:48] wakes up
[1518-07-11 00:31] falls asleep
[1518-06-20 00:17] falls asleep
[1518-04-11 00:32] falls asleep
[1518-03-31 23:57] Guard #349 begins shift
[1518-11-22 00:59] wakes up
[1518-10-28 23:57] Guard #659 begins shift
[1518-11-07 00:34] wakes up
[1518-10-27 00:35] wakes up
[1518-07-31 00:20] falls asleep
[1518-10-11 00:14] falls asleep
[1518-08-19 00:33] falls asleep
[1518-03-05 00:04] Guard #2161 begins shift
[1518-04-21 00:35] falls asleep
[1518-10-13 00:42] falls asleep
[1518-08-08 00:44] wakes up
[1518-09-19 00:47] wakes up
[1518-07-07 00:01] Guard #2441 begins shift
[1518-03-24 00:39] wakes up
[1518-10-20 23:59] Guard #349 begins shift
[1518-10-07 00:52] falls asleep
[1518-08-02 00:54] falls asleep
[1518-09-30 00:22] falls asleep
[1518-08-26 00:42] wakes up
[1518-04-26 00:04] falls asleep
[1518-09-28 00:14] falls asleep
[1518-10-23 00:34] falls asleep
[1518-08-16 00:29] falls asleep
[1518-04-15 00:39] falls asleep
[1518-08-29 00:37] falls asleep
[1518-08-14 00:33] falls asleep
[1518-08-15 00:46] wakes up
[1518-03-07 00:36] wakes up
[1518-10-27 00:48] falls asleep
[1518-05-17 00:06] falls asleep
[1518-04-13 00:03] falls asleep
[1518-10-22 00:43] wakes up
[1518-05-15 00:02] falls asleep
[1518-09-29 00:36] falls asleep
[1518-03-17 00:01] Guard #1889 begins shift
[1518-04-07 00:14] falls asleep
[1518-05-13 00:58] wakes up
[1518-11-17 00:49] falls asleep
[1518-03-28 00:43] falls asleep
[1518-04-15 00:02] Guard #659 begins shift
[1518-09-14 00:04] Guard #2179 begins shift
[1518-08-15 23:58] Guard #2179 begins shift
[1518-05-02 00:04] Guard #2917 begins shift
[1518-10-07 00:48] falls asleep
[1518-07-12 00:49] falls asleep
[1518-07-07 00:19] falls asleep
[1518-04-08 00:31] wakes up
[1518-04-01 00:47] wakes up
[1518-06-09 00:31] falls asleep
[1518-10-24 23:57] Guard #2833 begins shift
[1518-08-25 00:51] falls asleep
[1518-09-19 23:56] Guard #941 begins shift
[1518-03-06 23:59] Guard #2833 begins shift
[1518-04-03 00:13] falls asleep
[1518-06-04 00:01] Guard #3533 begins shift
[1518-10-17 00:40] wakes up
[1518-03-06 00:58] wakes up
[1518-08-31 00:30] wakes up
[1518-07-06 00:39] wakes up
[1518-05-26 00:45] wakes up
[1518-11-08 23:57] Guard #487 begins shift
[1518-06-15 00:15] falls asleep
[1518-09-12 00:28] falls asleep
[1518-08-01 00:09] falls asleep
[1518-03-10 00:49] wakes up
[1518-06-24 00:54] wakes up
[1518-06-06 00:21] wakes up
[1518-10-18 00:34] wakes up
[1518-03-27 00:01] Guard #1489 begins shift
[1518-05-07 00:51] wakes up
[1518-10-14 00:46] wakes up
[1518-05-20 00:27] wakes up
[1518-03-21 00:03] falls asleep
[1518-04-17 00:53] falls asleep
[1518-09-30 00:59] wakes up
[1518-07-23 00:55] wakes up
[1518-03-22 23:58] Guard #349 begins shift
[1518-09-22 00:32] falls asleep
[1518-05-14 23:54] Guard #1489 begins shift
[1518-04-03 00:44] falls asleep
[1518-04-18 00:20] falls asleep
[1518-09-22 00:59] wakes up
[1518-05-18 00:35] falls asleep
[1518-06-18 23:59] Guard #2441 begins shift
[1518-07-13 00:54] wakes up
[1518-09-21 00:35] wakes up
[1518-04-07 00:38] falls asleep
[1518-11-03 00:59] wakes up
[1518-05-02 00:21] falls asleep
[1518-07-02 23:59] Guard #2161 begins shift
[1518-03-20 00:34] falls asleep
[1518-04-07 00:32] wakes up
[1518-03-07 00:33] falls asleep
[1518-05-13 00:56] falls asleep
[1518-08-20 00:27] falls asleep
[1518-06-12 00:33] falls asleep
[1518-04-27 00:23] wakes up
[1518-10-05 23:54] Guard #2833 begins shift
[1518-07-22 00:02] Guard #2161 begins shift
[1518-03-29 00:49] wakes up
[1518-11-19 00:04] Guard #2161 begins shift
[1518-04-29 00:56] wakes up
[1518-06-18 00:14] falls asleep
[1518-11-21 00:00] Guard #2161 begins shift
[1518-05-16 00:09] falls asleep
[1518-09-16 00:00] Guard #1993 begins shift
[1518-10-14 00:09] falls asleep
[1518-04-14 00:32] falls asleep
[1518-04-30 00:11] falls asleep
[1518-10-15 00:13] falls asleep
[1518-11-12 00:03] falls asleep
[1518-08-01 00:51] falls asleep
[1518-05-08 00:58] wakes up
[1518-08-20 00:39] wakes up
[1518-11-07 00:56] wakes up
[1518-10-02 23:57] Guard #1471 begins shift
[1518-10-06 00:27] wakes up
[1518-06-08 00:04] Guard #349 begins shift
[1518-07-20 23:52] Guard #1993 begins shift
[1518-09-04 23:56] Guard #2467 begins shift
[1518-08-05 23:56] Guard #2441 begins shift
[1518-08-30 00:48] wakes up
[1518-06-08 00:39] falls asleep
[1518-03-27 00:55] falls asleep
[1518-03-11 23:56] Guard #659 begins shift
[1518-08-29 23:50] Guard #2179 begins shift
[1518-09-17 00:54] wakes up
[1518-09-06 00:34] falls asleep
[1518-03-25 00:56] wakes up
[1518-06-05 23:48] Guard #1993 begins shift
[1518-10-10 00:56] wakes up
[1518-03-09 00:37] wakes up
[1518-03-06 00:51] wakes up
[1518-11-04 00:22] falls asleep
[1518-09-02 00:11] falls asleep
[1518-05-22 00:59] wakes up
[1518-03-18 00:49] falls asleep
[1518-05-06 00:41] falls asleep
[1518-04-26 00:52] wakes up
[1518-06-28 00:54] falls asleep
[1518-08-17 00:57] wakes up
[1518-08-22 23:49] Guard #1889 begins shift
[1518-07-14 00:52] wakes up
[1518-09-09 23:57] Guard #1993 begins shift
[1518-08-24 00:55] wakes up
[1518-07-13 00:18] wakes up
[1518-09-26 00:29] falls asleep
[1518-03-05 00:57] wakes up
[1518-03-05 00:22] falls asleep
[1518-08-17 00:48] wakes up
[1518-08-03 00:25] wakes up
[1518-08-29 00:34] wakes up
[1518-05-28 00:04] falls asleep
[1518-08-26 23:59] Guard #373 begins shift
[1518-07-16 00:43] wakes up
[1518-07-08 00:07] falls asleep
[1518-10-12 23:58] Guard #3259 begins shift
[1518-06-05 00:45] wakes up
[1518-03-13 00:43] wakes up
[1518-08-05 00:21] falls asleep
[1518-10-03 00:56] wakes up
[1518-07-15 23:56] Guard #1489 begins shift
[1518-11-11 00:00] Guard #659 begins shift
[1518-06-23 00:05] falls asleep
[1518-03-28 00:57] wakes up
[1518-04-14 00:00] Guard #1489 begins shift
[1518-06-11 23:57] Guard #2467 begins shift
[1518-04-28 00:32] wakes up
[1518-07-17 00:35] wakes up
[1518-10-14 00:37] falls asleep
[1518-09-07 00:37] wakes up
[1518-08-15 00:02] Guard #349 begins shift
[1518-07-07 00:41] wakes up
[1518-04-13 00:35] falls asleep
[1518-09-18 00:04] Guard #349 begins shift
[1518-09-04 00:56] wakes up
[1518-08-16 23:59] Guard #1489 begins shift
[1518-07-09 23:56] Guard #373 begins shift
[1518-04-01 00:46] falls asleep
[1518-11-06 00:46] wakes up
[1518-10-23 23:58] Guard #1993 begins shift
[1518-09-06 00:59] wakes up
[1518-06-17 00:16] falls asleep
[1518-06-21 23:50] Guard #2971 begins shift
[1518-10-13 00:24] falls asleep
[1518-06-25 00:25] falls asleep
[1518-11-13 00:16] falls asleep
[1518-09-29 00:55] wakes up
[1518-05-23 00:43] wakes up
[1518-05-13 00:38] wakes up
[1518-11-16 00:33] falls asleep
[1518-09-09 00:55] wakes up
[1518-09-30 00:04] falls asleep
[1518-03-30 00:44] falls asleep
[1518-06-16 00:21] falls asleep
[1518-10-06 23:56] Guard #2917 begins shift
[1518-03-13 00:02] Guard #2833 begins shift
[1518-09-10 23:53] Guard #1889 begins shift
[1518-09-05 00:53] wakes up
[1518-09-09 00:33] falls asleep
[1518-10-02 00:40] wakes up
[1518-11-06 00:52] wakes up
[1518-11-02 00:55] wakes up
[1518-07-29 00:44] wakes up
[1518-10-24 00:29] falls asleep
[1518-05-04 23:57] Guard #2179 begins shift
[1518-05-29 00:33] falls asleep
[1518-09-11 00:02] falls asleep
[1518-11-11 00:12] falls asleep
[1518-09-01 23:57] Guard #2917 begins shift
[1518-07-29 00:01] Guard #1471 begins shift
[1518-07-15 00:14] falls asleep
[1518-03-27 00:35] wakes up
[1518-03-24 00:04] Guard #2467 begins shift
[1518-04-19 00:29] falls asleep
[1518-10-29 00:21] wakes up
[1518-04-19 00:32] wakes up
[1518-06-19 00:44] wakes up
[1518-09-27 00:57] wakes up
[1518-11-08 00:17] wakes up
[1518-07-06 00:11] falls asleep
[1518-07-04 00:01] falls asleep
[1518-04-23 23:57] Guard #3259 begins shift
[1518-09-18 00:57] wakes up
[1518-08-03 00:52] falls asleep
[1518-11-01 00:04] Guard #2441 begins shift
[1518-08-07 23:47] Guard #659 begins shift
[1518-05-12 00:08] falls asleep
[1518-11-16 00:03] falls asleep
[1518-05-17 00:54] falls asleep
[1518-08-12 00:23] falls asleep
[1518-07-02 00:16] falls asleep
[1518-04-09 00:10] falls asleep
[1518-07-19 00:22] falls asleep
[1518-08-23 00:45] wakes up
[1518-05-10 00:00] Guard #2467 begins shift
[1518-10-11 23:56] Guard #2179 begins shift
[1518-05-07 23:57] Guard #2833 begins shift
[1518-08-30 00:10] wakes up
[1518-06-11 00:47] falls asleep
[1518-08-10 00:55] falls asleep
[1518-03-23 00:49] wakes up
[1518-03-22 00:58] wakes up
[1518-10-07 00:49] wakes up
[1518-04-22 00:00] Guard #2467 begins shift
[1518-08-23 00:37] falls asleep
[1518-04-10 00:03] falls asleep
[1518-10-07 00:57] wakes up
[1518-05-04 00:53] wakes up
[1518-03-12 00:28] wakes up
[1518-03-23 00:38] wakes up
[1518-08-04 00:04] Guard #487 begins shift
[1518-08-19 23:58] Guard #1993 begins shift
[1518-11-07 00:47] wakes up
[1518-05-18 00:25] wakes up
[1518-09-04 00:31] falls asleep
[1518-07-01 00:22] wakes up
[1518-05-22 23:47] Guard #1471 begins shift
[1518-09-13 00:35] falls asleep
[1518-06-08 00:13] falls asleep
[1518-09-03 00:51] wakes up
[1518-04-22 00:21] falls asleep
[1518-05-10 00:22] falls asleep
[1518-08-02 00:14] wakes up
[1518-06-01 00:57] wakes up
[1518-11-16 00:59] wakes up
[1518-11-08 00:07] falls asleep
[1518-06-30 00:19] falls asleep
[1518-11-20 00:46] falls asleep
[1518-05-20 00:53] wakes up
[1518-06-29 00:38] wakes up
[1518-03-13 00:27] falls asleep
[1518-09-04 00:00] Guard #641 begins shift
[1518-07-05 23:59] Guard #2441 begins shift
[1518-04-20 00:35] wakes up
[1518-09-03 00:03] Guard #349 begins shift
[1518-08-16 00:36] wakes up
[1518-10-29 00:54] wakes up
[1518-10-05 00:04] Guard #641 begins shift
[1518-08-21 00:46] wakes up
[1518-07-19 23:59] Guard #2179 begins shift
[1518-05-01 00:45] wakes up
[1518-07-23 00:00] Guard #3259 begins shift
[1518-11-15 00:58] wakes up
[1518-11-15 00:49] falls asleep
[1518-07-04 00:43] wakes up
[1518-04-28 00:15] falls asleep
[1518-06-28 00:28] falls asleep
[1518-08-04 00:44] wakes up
[1518-07-04 00:34] wakes up
[1518-10-13 00:55] wakes up
[1518-10-22 00:46] falls asleep
[1518-10-04 00:15] falls asleep
[1518-09-19 00:28] falls asleep
[1518-06-01 00:02] Guard #641 begins shift
[1518-11-16 00:54] falls asleep
[1518-05-01 00:32] falls asleep
[1518-06-13 00:20] falls asleep
[1518-05-29 00:41] wakes up
[1518-05-06 00:56] wakes up
[1518-03-21 23:58] Guard #1123 begins shift
[1518-04-11 00:00] Guard #2971 begins shift
[1518-05-21 00:01] Guard #2833 begins shift
[1518-09-12 00:04] Guard #2441 begins shift
[1518-06-11 00:28] falls asleep
[1518-05-21 00:59] wakes up
[1518-07-14 00:03] falls asleep
[1518-04-13 00:06] wakes up
[1518-08-25 00:53] wakes up
[1518-05-10 00:54] wakes up
[1518-03-04 00:00] Guard #659 begins shift
[1518-05-27 00:34] wakes up
[1518-04-12 23:51] Guard #1123 begins shift
[1518-04-16 00:55] wakes up
[1518-05-26 00:25] falls asleep
[1518-10-02 00:13] wakes up
[1518-04-23 00:37] wakes up
[1518-07-14 00:19] wakes up
[1518-03-30 00:27] falls asleep
[1518-03-20 23:52] Guard #641 begins shift
[1518-07-25 00:58] wakes up
[1518-06-01 00:52] wakes up
[1518-03-22 00:49] falls asleep
[1518-08-31 00:04] falls asleep
[1518-08-23 00:29] wakes up
[1518-05-24 00:49] falls asleep
[1518-07-04 00:53] falls asleep
[1518-04-06 23:58] Guard #3259 begins shift
[1518-07-09 00:55] wakes up
[1518-10-19 00:01] Guard #2971 begins shift
[1518-06-19 00:27] wakes up
[1518-06-10 00:33] falls asleep
[1518-08-03 00:14] falls asleep
[1518-10-21 00:20] falls asleep
[1518-06-27 00:00] falls asleep
[1518-06-26 00:03] Guard #349 begins shift
[1518-11-15 00:01] Guard #659 begins shift
[1518-08-11 00:00] Guard #3533 begins shift
[1518-08-19 00:03] falls asleep
[1518-07-20 00:52] wakes up
[1518-06-16 00:47] falls asleep
[1518-07-08 00:12] wakes up
[1518-07-29 00:13] falls asleep
[1518-08-17 00:31] falls asleep
[1518-08-30 00:00] falls asleep
[1518-10-16 00:23] falls asleep
[1518-04-22 23:58] Guard #2179 begins shift
[1518-08-18 00:24] wakes up
[1518-08-07 00:56] wakes up
[1518-08-18 00:20] falls asleep
[1518-11-02 00:04] Guard #2161 begins shift
[1518-03-19 00:21] falls asleep
[1518-05-22 00:23] falls asleep
[1518-07-28 00:03] Guard #1811 begins shift
[1518-05-02 23:59] Guard #659 begins shift
[1518-03-28 00:51] wakes up
[1518-03-08 00:48] falls asleep
[1518-04-27 00:19] falls asleep
[1518-10-31 00:31] falls asleep
[1518-11-22 00:15] falls asleep
[1518-09-23 00:00] Guard #1993 begins shift
[1518-10-01 00:00] Guard #659 begins shift
[1518-06-05 00:44] falls asleep
[1518-04-11 00:49] falls asleep
[1518-08-01 00:55] wakes up
[1518-08-29 00:02] falls asleep
[1518-04-18 00:55] wakes up
[1518-09-08 00:10] falls asleep
[1518-08-24 00:32] falls asleep
[1518-06-25 00:51] wakes up
[1518-05-31 00:03] falls asleep
[1518-09-07 00:04] Guard #1123 begins shift
[1518-05-16 00:04] Guard #2161 begins shift
[1518-10-12 00:24] wakes up
[1518-08-12 00:10] falls asleep
[1518-10-09 00:10] falls asleep
[1518-10-17 00:02] falls asleep
[1518-09-25 00:37] falls asleep
[1518-09-14 00:07] falls asleep
[1518-06-03 00:45] wakes up
[1518-10-11 00:47] wakes up
[1518-09-21 00:40] falls asleep
[1518-03-04 00:39] falls asleep
[1518-03-25 00:36] wakes up
[1518-10-05 00:45] wakes up
[1518-03-04 00:47] wakes up
[1518-09-08 23:56] Guard #2833 begins shift
[1518-03-16 00:02] Guard #2273 begins shift
[1518-04-11 00:40] wakes up
[1518-10-26 00:55] wakes up
[1518-06-19 00:13] falls asleep
[1518-10-23 00:10] wakes up
[1518-08-27 00:27] wakes up
[1518-07-03 00:55] falls asleep
[1518-09-06 00:31] wakes up
[1518-05-17 00:43] wakes up
[1518-04-24 00:56] falls asleep
[1518-05-24 00:32] wakes up
[1518-10-20 00:28] falls asleep
[1518-03-25 00:53] falls asleep
[1518-06-06 00:01] falls asleep
[1518-05-03 00:34] falls asleep
[1518-10-20 00:55] falls asleep
[1518-09-21 23:59] Guard #641 begins shift
[1518-07-11 00:32] wakes up
[1518-07-27 00:40] falls asleep
[1518-09-24 00:03] Guard #3533 begins shift
[1518-10-14 23:56] Guard #373 begins shift
[1518-10-20 00:56] wakes up
[1518-08-01 23:57] Guard #2441 begins shift
[1518-10-17 00:49] wakes up
[1518-10-14 00:02] Guard #2833 begins shift
[1518-09-26 00:00] Guard #349 begins shift
[1518-06-16 00:54] wakes up
[1518-08-21 23:56] Guard #2273 begins shift
[1518-07-17 00:01] falls asleep
[1518-08-24 00:45] wakes up
[1518-03-22 00:52] wakes up
[1518-09-20 00:47] falls asleep
[1518-07-30 00:59] wakes up
[1518-10-19 00:52] wakes up
[1518-05-22 00:57] falls asleep
[1518-05-25 00:25] falls asleep
[1518-06-29 00:09] falls asleep
[1518-03-29 00:21] falls asleep
[1518-07-27 00:33] wakes up
[1518-05-16 00:40] falls asleep
[1518-04-10 00:59] wakes up
[1518-05-08 23:47] Guard #349 begins shift
[1518-04-10 00:50] falls asleep
[1518-04-12 00:16] falls asleep
[1518-07-25 00:51] wakes up
[1518-03-15 00:53] wakes up
[1518-06-09 23:57] Guard #2971 begins shift
[1518-09-23 00:06] falls asleep
[1518-10-30 00:07] falls asleep
[1518-03-17 00:47] wakes up
[1518-07-13 00:31] falls asleep
[1518-11-08 00:50] wakes up
[1518-11-06 00:40] falls asleep
[1518-10-15 00:48] wakes up
[1518-03-20 00:03] Guard #1811 begins shift
[1518-05-23 00:01] falls asleep
[1518-05-11 00:31] falls asleep
[1518-10-21 23:59] Guard #2833 begins shift
[1518-08-09 00:15] falls asleep
[1518-08-28 00:03] falls asleep
[1518-09-16 00:48] wakes up
[1518-07-26 00:31] wakes up
[1518-05-31 00:32] falls asleep
[1518-09-04 00:45] wakes up
[1518-07-26 00:14] falls asleep
[1518-11-19 00:13] wakes up
[1518-09-30 00:50] wakes up
[1518-08-19 00:06] wakes up
[1518-06-06 00:28] falls asleep
[1518-05-22 00:52] wakes up
[1518-04-02 00:53] wakes up
[1518-06-11 00:29] wakes up
[1518-07-14 00:23] falls asleep
[1518-05-23 00:48] wakes up
[1518-06-24 00:03] Guard #2179 begins shift
[1518-06-27 00:50] wakes up
[1518-08-17 00:41] falls asleep
[1518-06-21 00:23] wakes up
[1518-07-23 00:27] falls asleep
[1518-09-08 00:00] Guard #1471 begins shift
[1518-06-10 00:41] wakes up
[1518-08-30 23:50] Guard #659 begins shift
[1518-06-17 00:01] Guard #2441 begins shift
[1518-09-29 00:03] Guard #2971 begins shift
[1518-04-13 00:16] falls asleep
[1518-06-07 00:07] falls asleep
[1518-04-25 00:43] falls asleep
[1518-11-09 00:57] wakes up
[1518-07-27 00:02] falls asleep
[1518-04-20 00:58] wakes up
[1518-11-07 00:44] falls asleep
[1518-05-31 00:16] wakes up
[1518-04-22 00:41] falls asleep
[1518-06-14 00:00] Guard #1471 begins shift
[1518-09-21 00:26] falls asleep
[1518-09-06 00:26] falls asleep
[1518-07-28 00:13] falls asleep
[1518-05-10 00:57] falls asleep
[1518-11-06 00:24] falls asleep
[1518-05-20 00:31] falls asleep
[1518-10-27 00:20] falls asleep
[1518-03-31 00:42] wakes up
[1518-05-09 00:44] wakes up
[1518-03-27 00:30] falls asleep
[1518-05-12 23:56] Guard #1489 begins shift
[1518-03-10 00:00] Guard #1471 begins shift
[1518-10-14 00:18] wakes up
[1518-08-14 00:53] wakes up
[1518-10-26 00:04] Guard #2833 begins shift
[1518-11-08 00:23] falls asleep
[1518-04-16 23:59] Guard #1471 begins shift
[1518-03-25 00:08] falls asleep
[1518-09-28 00:31] falls asleep
[1518-08-27 00:49] falls asleep
[1518-09-21 00:52] falls asleep
[1518-10-10 00:46] falls asleep
[1518-10-17 00:14] wakes up
[1518-10-12 00:14] falls asleep
[1518-08-10 00:20] falls asleep
[1518-06-13 00:01] Guard #2917 begins shift
[1518-11-23 00:19] falls asleep
[1518-05-23 00:03] wakes up
[1518-11-16 00:36] wakes up
[1518-04-24 00:58] wakes up
[1518-06-17 00:43] wakes up
[1518-04-22 00:48] wakes up
[1518-11-04 00:51] wakes up
[1518-06-09 00:03] falls asleep
[1518-08-13 00:38] falls asleep
[1518-05-23 00:29] falls asleep
[1518-07-03 23:48] Guard #2441 begins shift
[1518-07-27 00:52] wakes up
[1518-04-04 00:27] falls asleep
[1518-07-31 00:29] wakes up
[1518-06-02 00:39] wakes up
[1518-10-19 23:58] Guard #1993 begins shift
[1518-04-15 00:40] wakes up
[1518-06-26 00:56] wakes up
[1518-04-19 00:50] falls asleep
[1518-09-08 00:50] wakes up
[1518-11-10 00:22] falls asleep
[1518-05-09 00:05] falls asleep
[1518-05-19 00:03] Guard #2917 begins shift
[1518-08-02 23:57] Guard #2161 begins shift
[1518-09-30 00:17] wakes up
[1518-10-06 00:02] falls asleep
[1518-08-06 00:11] falls asleep
[1518-07-10 00:48] wakes up
[1518-04-17 23:56] Guard #641 begins shift
[1518-10-22 00:07] falls asleep
[1518-04-09 23:51] Guard #659 begins shift
[1518-06-01 00:51] falls asleep
[1518-06-02 00:44] falls asleep
[1518-07-09 00:32] falls asleep
[1518-08-02 00:58] wakes up
[1518-04-27 23:58] Guard #1489 begins shift
[1518-07-08 00:00] Guard #2917 begins shift
[1518-06-29 00:01] Guard #1489 begins shift
[1518-05-31 00:52] wakes up
[1518-05-30 00:38] wakes up
[1518-05-03 00:53] wakes up
[1518-08-27 23:52] Guard #2467 begins shift
[1518-03-19 00:49] falls asleep
[1518-09-16 23:50] Guard #941 begins shift
[1518-06-23 00:22] falls asleep
[1518-07-23 00:47] wakes up
[1518-11-21 00:51] falls asleep
[1518-11-11 23:47] Guard #659 begins shift
[1518-09-20 00:56] wakes up
[1518-07-15 00:40] wakes up
[1518-05-21 23:59] Guard #2467 begins shift
[1518-10-08 23:58] Guard #2917 begins shift
[1518-03-24 00:19] falls asleep
[1518-04-10 00:55] falls asleep
[1518-10-17 00:43] falls asleep
[1518-03-21 00:49] wakes up
[1518-11-23 00:31] wakes up
[1518-03-26 00:58] wakes up
[1518-06-08 00:34] wakes up
[1518-04-14 00:53] wakes up
[1518-03-17 00:43] falls asleep
[1518-03-28 00:55] falls asleep
[1518-03-19 00:50] wakes up
[1518-08-17 00:52] falls asleep
[1518-05-02 00:46] wakes up
[1518-05-23 23:58] Guard #1993 begins shift
[1518-04-15 00:45] falls asleep
[1518-07-18 23:57] Guard #2441 begins shift
[1518-08-04 00:51] wakes up
[1518-09-20 23:56] Guard #1993 begins shift
[1518-09-05 23:56] Guard #1811 begins shift
[1518-06-05 00:01] Guard #2467 begins shift
[1518-11-23 00:02] Guard #349 begins shift
[1518-07-16 23:47] Guard #1123 begins shift
[1518-05-29 23:57] Guard #1489 begins shift
[1518-08-04 00:39] falls asleep
[1518-03-22 00:57] falls asleep
[1518-09-19 00:58] wakes up
[1518-05-08 00:19] falls asleep
[1518-11-03 00:57] falls asleep
[1518-04-12 00:02] Guard #2917 begins shift
[1518-03-25 00:01] Guard #3259 begins shift
[1518-06-17 23:59] Guard #1993 begins shift
[1518-08-08 00:38] wakes up
[1518-04-15 00:57] wakes up
[1518-04-20 00:02] Guard #1471 begins shift
[1518-10-27 23:56] Guard #2161 begins shift
[1518-03-15 00:01] falls asleep
[1518-09-28 00:00] Guard #659 begins shift
[1518-06-26 00:51] wakes up
[1518-10-01 00:57] wakes up
[1518-09-17 00:04] falls asleep
[1518-04-20 00:41] falls asleep
[1518-11-05 23:56] Guard #487 begins shift
[1518-05-11 00:28] wakes up
[1518-08-13 00:45] wakes up
[1518-09-21 00:53] wakes up
[1518-04-01 00:42] wakes up
[1518-03-09 00:29] falls asleep
[1518-09-19 00:57] falls asleep
[1518-07-17 00:59] wakes up
[1518-03-18 00:03] Guard #3259 begins shift
[1518-07-25 23:57] Guard #1993 begins shift
[1518-08-08 00:59] wakes up
[1518-03-31 00:04] Guard #3259 begins shift
[1518-11-02 00:14] falls asleep
[1518-08-17 00:36] wakes up
[1518-05-26 23:51] Guard #373 begins shift
[1518-06-07 00:01] Guard #1489 begins shift
[1518-05-16 00:44] wakes up
[1518-10-01 00:10] falls asleep
[1518-03-10 00:22] falls asleep
[1518-11-17 00:32] falls asleep
[1518-11-08 00:02] Guard #2971 begins shift
[1518-09-12 00:38] wakes up
[1518-09-10 00:43] falls asleep
[1518-04-03 00:59] wakes up
[1518-08-06 23:50] Guard #1489 begins shift
[1518-07-01 00:02] falls asleep
[1518-11-21 23:56] Guard #2917 begins shift
[1518-03-18 00:56] falls asleep
[1518-08-12 00:12] wakes up
[1518-08-17 23:59] Guard #2179 begins shift
[1518-04-04 00:51] wakes up
[1518-10-23 00:59] wakes up
[1518-11-07 00:25] falls asleep
[1518-04-08 00:10] falls asleep
[1518-05-27 23:47] Guard #3259 begins shift
[1518-07-13 23:47] Guard #487 begins shift
[1518-07-20 00:32] falls asleep
[1518-04-25 23:47] Guard #1489 begins shift
[1518-04-11 00:54] wakes up
[1518-06-20 00:00] Guard #2441 begins shift
[1518-11-07 00:51] falls asleep
[1518-08-09 00:47] wakes up
[1518-04-27 00:57] falls asleep
[1518-08-24 00:50] falls asleep
[1518-06-22 00:01] falls asleep
[1518-11-10 00:41] wakes up
[1518-07-17 23:57] Guard #3259 begins shift
[1518-08-31 23:59] Guard #2917 begins shift
[1518-07-15 00:43] falls asleep
[1518-07-05 00:23] falls asleep
[1518-07-13 00:09] falls asleep
[1518-10-03 00:13] wakes up
[1518-05-21 00:57] falls asleep
[1518-10-10 23:58] Guard #2441 begins shift
[1518-10-22 00:52] wakes up
[1518-03-29 00:01] Guard #2917 begins shift
[1518-10-18 00:29] falls asleep
[1518-10-21 00:26] wakes up
[1518-03-27 23:47] Guard #1471 begins shift
[1518-08-25 00:31] falls asleep
[1518-08-15 00:36] falls asleep
[1518-11-05 00:35] wakes up
[1518-06-30 00:01] Guard #373 begins shift
[1518-08-03 00:56] wakes up
[1518-08-25 00:48] wakes up
[1518-10-17 00:36] falls asleep
[1518-10-27 00:04] Guard #349 begins shift
[1518-05-19 00:25] falls asleep
[1518-09-13 00:39] wakes up
[1518-03-25 00:12] wakes up
[1518-04-24 23:56] Guard #1811 begins shift
[1518-06-22 23:50] Guard #487 begins shift
[1518-04-16 00:03] Guard #2161 begins shift
[1518-04-05 00:41] wakes up
[1518-06-03 00:26] falls asleep
[1518-04-20 00:12] falls asleep
[1518-06-14 00:56] wakes up
[1518-05-15 00:13] wakes up
[1518-06-08 00:51] wakes up
[1518-04-18 00:28] wakes up
[1518-03-11 00:51] wakes up
[1518-10-28 00:53] wakes up
[1518-09-01 00:59] wakes up
[1518-05-07 00:00] Guard #641 begins shift
[1518-10-20 00:39] wakes up
[1518-03-24 00:55] wakes up
[1518-04-02 00:33] falls asleep
[1518-11-15 00:18] falls asleep
[1518-08-27 00:52] wakes up
[1518-06-19 00:40] falls asleep
[1518-10-05 00:50] falls asleep
[1518-10-15 23:56] Guard #1489 begins shift
[1518-07-10 00:23] falls asleep
[1518-04-06 00:04] Guard #1381 begins shift
[1518-08-07 00:00] falls asleep
[1518-07-12 00:53] wakes up
[1518-11-15 00:38] wakes up
[1518-07-13 00:00] Guard #1123 begins shift
[1518-04-30 00:13] wakes up
[1518-07-04 00:38] falls asleep
[1518-05-24 00:53] wakes up
[1518-05-10 23:56] Guard #1489 begins shift
[1518-10-24 00:31] wakes up
[1518-09-11 00:58] wakes up
[1518-10-01 23:57] Guard #1489 begins shift
[1518-08-21 00:01] falls asleep
[1518-07-30 00:21] falls asleep
[1518-05-24 00:19] falls asleep
[1518-09-30 00:57] falls asleep
[1518-03-10 23:59] Guard #2161 begins shift
[1518-05-20 00:26] falls asleep
[1518-03-12 00:14] falls asleep
[1518-11-09 00:16] falls asleep
[1518-05-27 00:02] falls asleep
[1518-05-14 00:01] falls asleep
[1518-05-13 00:33] falls asleep
[1518-04-01 00:21] falls asleep
[1518-04-10 00:39] wakes up
[1518-08-04 00:50] falls asleep
[1518-04-22 00:38] wakes up
[1518-07-25 00:56] falls asleep
[1518-07-08 23:58] Guard #349 begins shift
[1518-07-05 00:49] wakes up
[1518-03-08 00:56] wakes up
[1518-08-04 23:56] Guard #1471 begins shift
[1518-08-29 00:58] wakes up
[1518-10-04 00:44] wakes up
[1518-07-01 00:42] wakes up
[1518-05-12 00:52] wakes up
[1518-07-25 00:40] falls asleep
[1518-09-25 00:50] wakes up
[1518-06-25 00:00] Guard #1489 begins shift
[1518-04-27 00:51] wakes up
[1518-04-18 00:52] falls asleep
[1518-05-30 23:50] Guard #1489 begins shift
[1518-04-17 00:59] wakes up
[1518-09-18 00:55] falls asleep
[1518-04-03 00:01] Guard #659 begins shift
[1518-04-13 00:19] wakes up
[1518-05-13 23:52] Guard #349 begins shift
[1518-11-16 00:22] wakes up
[1518-06-30 23:50] Guard #1889 begins shift
[1518-11-17 00:51] wakes up
[1518-07-23 00:53] falls asleep
[1518-09-28 00:32] wakes up
[1518-07-31 00:02] Guard #1993 begins shift
[1518-04-04 23:56] Guard #2179 begins shift
[1518-06-09 00:25] wakes up
[1518-04-19 00:52] wakes up
[1518-06-13 00:49] wakes up
[1518-07-17 00:42] falls asleep
[1518-04-23 00:32] falls asleep
[1518-05-11 00:46] wakes up
[1518-05-11 00:16] falls asleep
[1518-08-20 23:50] Guard #1889 begins shift
[1518-08-01 00:46] wakes up
[1518-03-18 00:59] wakes up
[1518-05-05 23:56] Guard #641 begins shift
[1518-09-10 00:57] wakes up
[1518-09-13 00:16] falls asleep
[1518-05-04 00:02] Guard #1811 begins shift
[1518-08-27 00:43] wakes up
[1518-03-18 23:59] Guard #2179 begins shift
[1518-07-24 23:57] Guard #373 begins shift
[1518-05-14 00:47] wakes up
[1518-11-15 23:48] Guard #2917 begins shift
[1518-06-23 00:15] wakes up
[1518-09-29 23:50] Guard #2971 begins shift
[1518-08-08 00:04] falls asleep
[1518-10-30 00:49] wakes up
[1518-10-19 00:40] falls asleep
[1518-10-26 00:36] falls asleep
[1518-05-01 00:59] wakes up
[1518-08-21 00:21] wakes up
[1518-06-01 00:56] falls asleep
[1518-11-13 23:53] Guard #2917 begins shift
[1518-10-07 00:36] wakes up
[1518-11-16 23:56] Guard #1471 begins shift
[1518-11-01 00:26] falls asleep
[1518-05-25 00:55] wakes up
[1518-10-02 00:12] falls asleep
[1518-07-10 00:55] falls asleep
[1518-07-10 23:57] Guard #1889 begins shift
[1518-10-25 00:46] wakes up
[1518-08-01 00:00] Guard #1811 begins shift
[1518-09-14 00:42] wakes up
[1518-05-17 00:58] wakes up
[1518-10-28 00:52] falls asleep
[1518-07-01 00:30] falls asleep
[1518-03-14 00:43] falls asleep
[1518-07-21 00:02] falls asleep
[1518-04-07 00:45] wakes up
[1518-05-20 00:02] Guard #1993 begins shift
[1518-04-05 00:28] falls asleep
[1518-11-02 23:58] Guard #3259 begins shift
[1518-04-14 00:50] falls asleep
[1518-07-22 00:37] falls asleep
[1518-10-27 00:53] wakes up
[1518-04-30 00:02] Guard #2179 begins shift
[1518-03-23 00:43] falls asleep
[1518-09-04 00:52] falls asleep
[1518-09-07 00:09] falls asleep
[1518-08-18 23:50] Guard #2179 begins shift
[1518-10-02 00:24] falls asleep
[1518-11-19 00:12] falls asleep
[1518-03-23 00:21] falls asleep
[1518-06-21 00:01] Guard #2917 begins shift
[1518-04-28 23:54] Guard #2441 begins shift
[1518-09-21 00:45] wakes up
[1518-10-18 00:04] Guard #1489 begins shift
[1518-09-23 00:47] wakes up
[1518-09-16 00:33] falls asleep
[1518-08-08 00:57] falls asleep
[1518-10-23 00:08] falls asleep
[1518-09-18 23:59] Guard #2441 begins shift
[1518-03-20 00:54] wakes up
[1518-10-09 00:57] wakes up
[1518-08-12 00:00] Guard #2917 begins shift
[1518-09-13 00:02] Guard #1489 begins shift
[1518-10-13 00:25] wakes up
[1518-11-06 23:58] Guard #373 begins shift
[1518-04-08 00:03] Guard #641 begins shift
[1518-11-13 00:49] wakes up
[1518-07-31 00:53] wakes up
[1518-08-23 23:58] Guard #1471 begins shift
[1518-07-03 00:56] wakes up
[1518-03-28 00:00] falls asleep
[1518-03-07 00:39] falls asleep
[1518-08-28 00:40] wakes up
[1518-04-29 00:03] falls asleep
[1518-04-01 23:56] Guard #1489 begins shift
[1518-06-02 00:00] Guard #659 begins shift
[1518-10-05 00:55] wakes up
[1518-10-29 00:48] falls asleep
[1518-06-12 00:59] wakes up
[1518-07-02 00:26] wakes up
[1518-03-14 00:47] wakes up
[1518-04-03 23:59] Guard #349 begins shift
[1518-07-12 00:04] Guard #941 begins shift
[1518-03-26 00:04] Guard #1123 begins shift
[1518-06-02 00:59] wakes up
[1518-07-04 00:54] wakes up
[1518-03-19 00:34] wakes up
[1518-10-10 00:01] Guard #941 begins shift
[1518-08-05 00:53] wakes up
[1518-06-08 23:53] Guard #373 begins shift
[1518-03-28 00:29] wakes up
[1518-10-22 23:58] Guard #1471 begins shift
[1518-11-14 00:03] falls asleep
[1518-10-25 00:32] falls asleep
[1518-06-24 00:32] falls asleep
[1518-06-23 00:47] wakes up
[1518-04-16 00:16] falls asleep
[1518-04-03 00:21] wakes up
[1518-10-08 00:00] Guard #2273 begins shift
[1518-09-27 00:01] Guard #1811 begins shift
[1518-08-25 00:02] Guard #2833 begins shift
[1518-07-21 00:53] wakes up
[1518-07-05 00:02] Guard #2971 begins shift
[1518-07-28 00:58] wakes up
[1518-07-03 00:51] wakes up
[1518-03-14 00:02] Guard #2833 begins shift
[1518-10-15 00:19] wakes up
[1518-08-20 00:57] wakes up
[1518-05-23 00:47] falls asleep
[1518-11-05 00:10] falls asleep
[1518-03-31 00:58] wakes up
[1518-05-26 00:00] Guard #1811 begins shift
[1518-04-27 00:58] wakes up
[1518-07-24 00:57] wakes up
[1518-03-30 00:39] wakes up
[1518-08-13 23:58] Guard #1889 begins shift
[1518-04-10 00:52] wakes up
[1518-10-31 00:59] wakes up
[1518-10-03 00:11] falls asleep
[1518-10-15 00:29] falls asleep
[1518-08-20 00:54] falls asleep
[1518-09-28 00:50] wakes up
[1518-10-26 00:42] wakes up
[1518-09-25 00:00] Guard #487 begins shift
[1518-07-22 00:47] falls asleep
[1518-06-14 00:54] falls asleep
[1518-08-08 00:43] falls asleep
[1518-08-25 23:56] Guard #487 begins shift
[1518-07-01 23:59] Guard #2917 begins shift
[1518-06-16 00:01] Guard #1811 begins shift
[1518-03-14 23:46] Guard #1993 begins shift
[1518-03-31 00:14] falls asleep
[1518-03-06 00:30] falls asleep
[1518-09-14 23:57] Guard #2273 begins shift
[1518-06-09 00:34] wakes up
[1518-07-03 00:16] falls asleep
[1518-06-16 00:43] wakes up
[1518-08-23 00:04] falls asleep
[1518-10-22 00:42] falls asleep
[1518-05-28 00:41] wakes up
[1518-08-09 23:57] Guard #941 begins shift
[1518-11-17 00:38] wakes up
[1518-03-24 00:51] falls asleep
[1518-07-15 00:54] wakes up
[1518-09-18 00:50] wakes up
[1518-10-05 00:25] falls asleep
[1518-07-18 00:12] falls asleep
[1518-11-01 00:30] wakes up
[1518-05-07 00:42] falls asleep
[1518-09-03 00:24] falls asleep
[1518-11-20 00:56] wakes up
[1518-11-11 00:54] wakes up
[1518-03-11 00:42] falls asleep
[1518-05-04 00:32] falls asleep
[1518-06-26 00:47] falls asleep
[1518-10-29 00:13] falls asleep
[1518-11-04 00:03] Guard #2917 begins shift
[1518-05-18 00:12] falls asleep
[1518-08-02 00:46] falls asleep
[1518-09-21 00:57] falls asleep
[1518-06-28 00:57] wakes up
[1518-03-27 00:57] wakes up
[1518-06-28 00:02] Guard #487 begins shift
[1518-03-07 23:57] Guard #941 begins shift
[1518-04-09 00:03] Guard #2179 begins shift
[1518-06-27 00:26] wakes up
[1518-08-12 00:59] wakes up
[1518-07-22 00:38] wakes up
[1518-08-06 00:34] wakes up
[1518-03-30 00:53] wakes up
[1518-11-05 00:00] Guard #2971 begins shift
[1518-11-06 00:30] wakes up
[1518-03-24 00:08] falls asleep
[1518-08-09 00:02] Guard #2161 begins shift
[1518-04-30 23:56] Guard #2161 begins shift
[1518-09-28 00:42] falls asleep
[1518-04-24 00:42] wakes up
[1518-10-26 00:46] falls asleep
[1518-11-18 00:14] falls asleep
[1518-10-16 23:46] Guard #1123 begins shift
[1518-09-21 00:59] wakes up
[1518-07-10 00:59] wakes up
[1518-04-25 00:46] wakes up
[1518-06-03 00:02] Guard #1811 begins shift
[1518-07-18 00:55] wakes up
[1518-11-03 00:54] wakes up
[1518-10-07 00:07] falls asleep
[1518-07-29 23:56] Guard #3259 begins shift
[1518-03-31 00:51] falls asleep
[1518-07-19 00:26] wakes up
[1518-05-18 00:00] Guard #1123 begins shift
[1518-10-29 23:58] Guard #659 begins shift
[1518-10-16 00:35] wakes up
[1518-03-05 23:58] Guard #2917 begins shift
[1518-03-29 23:57] Guard #1471 begins shift
[1518-05-16 00:22] wakes up
[1518-05-28 23:58] Guard #487 begins shift
[1518-10-22 00:34] wakes up
[1518-03-24 00:11] wakes up
[1518-11-18 00:50] wakes up
[1518-08-02 00:07] falls asleep
[1518-11-13 00:04] Guard #1993 begins shift
[1518-05-17 00:00] Guard #2467 begins shift
[1518-11-20 00:02] Guard #2467 begins shift
[1518-04-14 00:37] wakes up
[1518-07-16 00:17] falls asleep
[1518-07-24 00:38] falls asleep
[1518-08-28 23:50] Guard #1889 begins shift
[1518-11-06 00:49] falls asleep
[1518-06-02 00:11] falls asleep
[1518-05-10 00:59] wakes up";


    #[test]
    fn test_part1() {
        let res = part1(input.lines().collect());
        println!("{}", res);
        assert_eq!(72925, res);
    }

    #[test]
    fn test_part2() {
        let res = part2(input.lines().collect());
        println!("{}", res);
    }
}
