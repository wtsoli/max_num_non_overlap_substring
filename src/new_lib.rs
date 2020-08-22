mod front_of_house;

//use crate::front_of_house::DerefMutExample;

pub struct Solution {}

use std::collections::HashSet;
impl Solution {

    fn build_letter_pos(letter_pos: &mut Vec<Vec<usize>>, s: &str) {

        for (position, letter) in s.chars().enumerate() {
            let index = letter as usize - 97;
            letter_pos[index].push(position);
        }

        for (i, l) in letter_pos.iter().enumerate() {
            println!("{:?} : {:?}", (i as u8 + 97) as char, l);
        }
        

    }

    fn cal_range_for(s: &str, letter_pos: &Vec<Vec<usize>>, from: usize) -> (usize, usize) {

        let current_c = s.chars().nth(from).unwrap();
        let index = current_c as usize - 97;

        let mut left = from.clone();
        let mut right = letter_pos[index].last().unwrap().clone();
        let mut end = right;

        for i in left..=right {
            let current_c = s.chars().nth(i).unwrap();
            let index = current_c as usize - 97;
            let new_end = letter_pos[index].last().unwrap().clone();
            if new_end > end {
                end = new_end;
            }
        }

        while right < end {
            println!("right being pushed to new end from {:?} to {:?}", right ,end);
            left = right+1;
            right = end;
            for i in left..=right {
                let current_c = s.chars().nth(i).unwrap();
                let index = current_c as usize - 97;
                let new_end = letter_pos[index].last().unwrap().clone();
                if new_end > end {
                    end = new_end;
                }
            }
        }

        return (from, right);

    }

    fn current_cs_not_sandwiched_by(letter_pos: &Vec<Vec<usize>>, cs: &HashSet<char>, start: usize, end: usize) -> bool {

        let mut flag = true;

        for c in cs.iter() {
            
            flag = Solution::current_c_not_sandwiched_by(letter_pos, *c, start, end);
            if flag == false {
                return false;
            }


        }
        return flag;
    }

    fn current_c_not_sandwiched_by(letter_pos: &Vec<Vec<usize>>, current_c: char, start: usize, end: usize) -> bool {

        let current_index = current_c as usize - 97;
        let positions = &letter_pos[current_index];

        for pos in positions.iter() {
            if *pos > start && *pos < end {
                return false;
            }
        }

        return true;


    } //  badadbeabc

    fn collect_all_push_pos(s: &str, letter_pos: &Vec<Vec<usize>>, current_c: char) -> Vec<bool> {
        let mut result: Vec<bool> = vec![false; 26];
        result[current_c as usize - 97] = true;

        let index = current_c as usize - 97;
        let left = *letter_pos[index].first().unwrap();
        let right = *letter_pos[index].last().unwrap();

        if right - left <= 1 {
            return result;
        }

        // Todo:
        'outer1: for i in left+1 ..= right-1 {
            let range_c = s.chars().nth(i).unwrap();
            if range_c == current_c {
                continue 'outer1;
            }

            let cur_index = range_c as usize - 97;
            let cur_left = *letter_pos[cur_index].first().unwrap();
            let cur_right = *letter_pos[cur_index].last().unwrap();

            let mut i = 0;
            
            while i < result.len() {
                if result[i] && !result[cur_index] {
                    let letter_to_ask = (i as u8 + 97) as char;
                    if !Solution::current_c_not_sandwiched_by(letter_pos, letter_to_ask, cur_left, cur_right) {
                        result[cur_index] = false;
                    }
                }
                i += 1;
            }

        }

        'outer2: for i in (left+1 ..= right-1).rev() {
            let range_c = s.chars().nth(i).unwrap();
            if range_c == current_c {
                continue 'outer2;
            }

            let cur_index = range_c as usize - 97;
            let cur_left = *letter_pos[cur_index].first().unwrap();
            let cur_right = *letter_pos[cur_index].last().unwrap();

            let mut i = 0;
            
            while i < result.len() {
                if result[i] && !result[cur_index] {
                    let letter_to_ask = (i as u8 + 97) as char;
                    if !Solution::current_c_not_sandwiched_by(letter_pos, letter_to_ask, cur_left, cur_right) {
                        result[cur_index] = true;
                    }
                }
                i += 1;
            }

        }


        return result;



    }

    fn find_all_from(s: &str, letter_pos: &Vec<Vec<usize>>, from: usize, original_end: usize) -> Vec<(usize, usize)> {

        println!("find_all_from got called for FROM {:?} TO {:?}", from, original_end);

        if from > original_end {
            return vec![];
        }

        let current_c = s.chars().nth(from).unwrap();
        let index = current_c as usize - 97;

        let mut left = from.clone();
        let mut right = letter_pos[index].last().unwrap().clone();

        if left == right {
            let mut result: Vec<(usize, usize)> = vec![(left,right)];
            result.append(&mut Solution::find_all_from(s, letter_pos, left+1, original_end));
            return result;
        }

        let mut end = right;
        let mut non_push_ranges: Vec<(usize, usize)> = Vec::new();
        // let mut sticky_members: Vec<char> = vec![];
        // sticky_members.push(current_c);

        let sticky_members = Solution::collect_all_push_pos(s, letter_pos, current_c);

        'outer: for i in left..=right {
            let range_c = s.chars().nth(i).unwrap();
            if range_c == current_c {
                continue 'outer;
            }
            let index = range_c as usize - 97;
            let new_start = letter_pos[index].first().unwrap().clone();
            let new_end = letter_pos[index].last().unwrap().clone();
            // if new_end < right {
            println!("deciding sandwich for {:?} from {:?}: {:?}, {:?}", current_c, range_c, new_start, new_end);
            //if Solution::current_c_not_sandwiched_by(letter_pos, current_c, new_start, new_end) {
            if !sticky_members[index] {
                println!("not sandwich for {:?} from {:?}: {:?}, {:?}", current_c, range_c, new_start, new_end);
                
                if non_push_ranges.len() == 0 {
                    let range = (i,i);
                    non_push_ranges.push(range);
                } else {
                    let length = non_push_ranges.len();
                    let last_range = non_push_ranges[length-1];
                    let (pre, post) = last_range;
                    if post == i - 1 {
                        println!("extends last non_push_ranges ele {:?}", non_push_ranges);
                        std::mem::replace(&mut non_push_ranges[length-1] , (pre, i));
                        println!("after extends {:?}", non_push_ranges);
                    } else {
                        let range = (i,i);
                        non_push_ranges.push(range);
                    }
                }
                println!("current non_push_ranges: {:?}", non_push_ranges);
            }

            if new_end > end {
                end = new_end;
            }
        }

        // find all sticky letters during pushing
        while right < end {

        }

        
        while right < end {
            println!("right being pushed to new end from {:?} to {:?}", right ,end);
            left = right+1;
            right = end;
            // for i in left..=right {
            //     let range_c = s.chars().nth(i).unwrap();
            //     let index = range_c as usize - 97;
            //     let new_end = letter_pos[index].last().unwrap().clone();
            //     if new_end > end {
            //         end = new_end;
            //     }
            // }

            'inner: for i in left..=right {
                let range_c = s.chars().nth(i).unwrap();
                if range_c == current_c {
                    continue 'inner;
                }
                let index = range_c as usize - 97;
                let new_start = letter_pos[index].first().unwrap().clone();
                let new_end = letter_pos[index].last().unwrap().clone();
                // if new_end < right {
                println!("further deciding sandwich for {:?} from {:?}: {:?}, {:?}", range_c, sticky_members, new_start, new_end);
                
                if !sticky_members[index] && Solution::current_cs_not_sandwiched_by(letter_pos, &sticky_members, new_start, new_end) {
                    println!("further not sandwich for {:?} from {:?}: {:?}, {:?}", range_c, sticky_members, new_start, new_end);
                    
                    if non_push_ranges.len() == 0 {
                        let range = (i,i);
                        non_push_ranges.push(range);
                    } else {
                        let length = non_push_ranges.len();
                        let last_range = non_push_ranges[length-1];
                        let (pre, post) = last_range;
                        if post == i - 1 {
                            println!("extends last non_push_ranges ele {:?}", non_push_ranges);
                            
                            std::mem::replace(&mut non_push_ranges[length-1] , (pre, i));
                            println!("after extends {:?}", non_push_ranges);
                        } else {
                            let range = (i,i);
                            non_push_ranges.push(range);
                        }
                    }
                    println!("current non_push_ranges: {:?}", non_push_ranges);
                }
                
                if new_end > end {
                    end = new_end;
                }
            }
        }


        let mut result: Vec<(usize, usize)> = Vec::new();
        if non_push_ranges.len() > 0 {
            for range in non_push_ranges.iter() {
                result.append(&mut Solution::find_all_from(s, letter_pos, range.0, range.1));
            }
            // return result;
        } else {
            // let mut package_one = vec![(from, right)];
            result.push((from, right));
            // return package_one;
        }

        if end < original_end {
            result.append(&mut Solution::find_all_from(s, letter_pos, end+1, original_end));
        }

        return result;
    }

    fn find_result(s: &str, letter_pos: &Vec<Vec<usize>>, begin: usize, end: usize) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();

        if begin == end {
            return vec![(begin, end)];
        }

        
        let pairs = Solution::find_all_from(s, letter_pos, begin, end);

        println!("all pairs: {:?}", pairs);
        

        // result.push((2,2));
        // result.push((3,3));
        // result.push((8,10));
        return pairs;
    }

    #[allow(dead_code)]
    pub fn max_num_of_substrings(s: String) -> Vec<String> {

        let mut result: Vec<String> = vec![];

        
        let mut letter_pos: Vec<Vec<usize>> = vec![Vec::new(); 26];

        Solution::build_letter_pos(&mut letter_pos, &s);

        let positions = Solution::find_result(&s, &letter_pos, 0, s.len()-1);

        for pos in positions.iter() {
            let mut sub_string = String::new();
            for x in pos.0..=pos.1 {
                sub_string.push(s.chars().nth(x).unwrap());
            }
            result.push(sub_string);
        }
        return result;
        
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = Solution::max_num_of_substrings(String::from("bbcacbaba"));
        //let result = Solution::max_num_of_substrings(String::from("adefaddaccchgecc"));
        //let result = Solution::max_num_of_substrings(String::from("badadbeabc"));
        println!("result: {:?}", result);
    }
}
