mod front_of_house;

//use crate::front_of_house::DerefMutExample;

pub struct Solution {}

use std::collections::HashSet;



impl Solution {

    fn build_letter_pos(s: &str, letter_pos: &mut Vec<Vec<usize>>) {

        for (position, letter) in s.chars().enumerate() {
            let index = letter as usize - 97;
            letter_pos[index].push(position);
        }

        for (i, l) in letter_pos.iter().enumerate() {
            println!("{:?} : {:?}", (i as u8 + 97) as char, l);
        }
        

    }

    fn sticky_letter (letter: char, sticky_letters: &Vec<bool>, letter_pos: &mut Vec<Vec<usize>>) -> bool {
        let letter_index = letter as usize - 97;
        if sticky_letters[letter_index] {
            // if letter == 'e' {
            //     println!("e hit with already sticky set");
            // }
            return true;
        }

        for (pos, sticky) in sticky_letters.iter().enumerate() {
            if *sticky {
                let cur_left = *letter_pos[letter_index].first().unwrap();
                let cur_right = *letter_pos[letter_index].last().unwrap();
                for sticky_letter_pos in letter_pos[pos].iter() {
                    if *sticky_letter_pos > cur_left && *sticky_letter_pos < cur_right {
                        return true;
                    }
                }
                // let sticky_left = *letter_pos[pos].first().unwrap();
                // let sticky_right = *letter_pos[pos].last().unwrap();

                // if !(cur_right < sticky_left || sticky_right < cur_left) {
                //     if letter == 'e' {
                //         println!("e hit with cur_left: {:?}, cur_right: {:?}, sticky_left: {:?}, sticky_right: {:?}", cur_left,cur_right, sticky_left, sticky_right);
                //     }
                //     return true;
                // }
            }
            
        }
        return false;
    }

    fn push_from(s: &str, from: usize, letter_pos: &mut Vec<Vec<usize>>) ->  Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = vec![];

        let current_c = s.chars().nth(from).unwrap();
        println!("run push_from from {:?} for letter: {:?}",from, current_c);
        let mut sticky_members: Vec<bool> = vec![false; 26];
        sticky_members[current_c as usize - 97] = true;
        let index = current_c as usize - 97;
        let left = *letter_pos[index].first().unwrap();
        let right = *letter_pos[index].last().unwrap();

        if right - left <= 1 { // no need to push, since same letter only appear twice adjacent to each other
            result.push((left, right));
            if right < s.len() - 1 {
                result.append(&mut Solution::push_from(s, right+1, letter_pos));
            }
            return result;
        }

        let mut right_bound = right;
        let mut cursor = left + 1;

        while cursor < right_bound {
            let cursor_letter = s.chars().nth(cursor).unwrap();
            if Solution::sticky_letter(cursor_letter, &sticky_members, letter_pos) {
                sticky_members[cursor_letter as usize - 97] = true;
            }
            let cursor_right = *letter_pos[cursor_letter as usize - 97].last().unwrap();
            if right_bound < cursor_right {
                right_bound = cursor_right;
            }
            cursor += 1;
        }

        // move the cursor to the right most position
        let mut cursor = right_bound;
        // do the sticky letter detecting from right_bound back to left
        // to find the ones not detected by first turn from left to right
        while cursor >= left {
            let cursor_letter = s.chars().nth(cursor).unwrap();
            if Solution::sticky_letter(cursor_letter, &sticky_members, letter_pos) {
                sticky_members[cursor_letter as usize - 97] = true;
            }
            if cursor > 0 {
                cursor -= 1;
            } else {
                break;
            }
        }
        // find out all possible free ranges and recursively call againt each one and append the result
        let mut sentinel = usize::MAX;
        let mut all_sticky: bool = true;
        for cur_index in left..=right_bound {
            let current_c = s.chars().nth(cur_index).unwrap();
            if !sticky_members[current_c as usize - 97] {
                if sentinel == usize::MAX {
                    sentinel = cur_index;
                }
            } else {
                if sentinel != usize::MAX {
                    let (pair_l, pair_r) = (sentinel, cur_index - 1);
                    println!("got inner non-sticky: {:?}", (pair_l, pair_r));
                    result.push((pair_l, pair_r));
                    all_sticky = false;
                    sentinel = usize::MAX;
                }
            }
        }

        if all_sticky {
            result.push((left, right_bound));
        }

        println!("sticky_members: {:?}", sticky_members);

        if right_bound < s.len() - 1 {
            result.append(&mut Solution::push_from(s, right_bound+1, letter_pos));
        }
        println!("right_bould: {:?}", right_bound);
        println!("sticky letters: {:?}", sticky_members);

        return result;
        
    }

    fn find_all_from(s: &str, from: usize, original_end: usize, letter_pos: &mut Vec<Vec<usize>>) -> Vec<(usize, usize)> {
        println!("find_all_from got called for FROM {:?} TO {:?}", from, original_end);
        if from > original_end {
            return vec![];
        }
        let pos_pairs = Solution::push_from(s, from, letter_pos);
        return pos_pairs;
    }

    fn find_result(s: &str, begin: usize, end: usize, letter_pos: &mut Vec<Vec<usize>>) -> Vec<(usize, usize)> {
        if begin == end {
            return vec![(begin, end)];
        }
        let pairs = Solution::find_all_from(s, begin, end, letter_pos);
        println!("all pairs: {:?}", pairs);
        return pairs;
    }

    #[allow(dead_code)]
    pub fn max_num_of_substrings(s: String) -> Vec<String> {

        let mut result: Vec<String> = vec![];

        let mut letter_pos: Vec<Vec<usize>> = vec![Vec::new(); 26];

        Solution::build_letter_pos(&s, &mut letter_pos);

        let positions = Solution::find_result(&s, 0, s.len()-1, &mut letter_pos);

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
        //let result = Solution::max_num_of_substrings(String::from("bbcacbabaef"));
        //let result = Solution::max_num_of_substrings(String::from("adefaddaccchgecc"));
        let result = Solution::max_num_of_substrings(String::from("badadbeabcyxzxzyw"));
        println!("result: {:?}", result);
    }
}
