// use itertools::Itertools;
use std::char;
use std::collections::HashSet;

pub fn process_part1(input: &str) -> i32 {
    let mut sum_total: i32 = 0;
    for line in input.lines() {
        let (compartment_one, compartment_two) = line.split_at(line.len() / 2);
        
        // my initial attempt - create hash set of unique chars from one compartment to iterate through
        // let set: HashSet<char> = compartment_one.chars().collect();
        // we could have used a vec
        // let char_vec: Vec<char> = compartment_one.chars().into_iter().unique().collect();
        // for char in set {
        //     if compartment_two.chars().contains(&char) {
        //         let item_priority = char_to_num(char);
        //         // dbg!(item_priority);
        //         sum_total += item_priority;
        //     } else {
        //         continue;
        //     }
        // }

        let compartment_one_chars: HashSet<char> = compartment_one.chars().collect();
        let compartment_two_chars: HashSet<char> = compartment_two.chars().collect();
        let common_chars: HashSet<char> = compartment_one_chars.intersection(&compartment_two_chars).copied().collect();
        let single_char: char = common_chars.into_iter().next().unwrap();
        let item_priority = char_to_num(single_char);
        sum_total += item_priority;
    }
    return sum_total;
}

pub fn process_part2(input: &str) -> i32 {
    let mut sum_total: i32 = 0;
    let blocks: Vec<&str> = input.split("\n").collect();
    let mut group_vec_vecs: Vec<Vec<&str>> = Vec::new();
    for i in (0..blocks.len()).step_by(3) {
        let mut block: Vec<&str> = Vec::new();
        block.push(blocks[i]);
        block.push(blocks[i + 1]);
        block.push(blocks[i + 2]);
        group_vec_vecs.push(block);
    }

    // Iterate through each block and find characters present in all strings
    for (block_index, block) in group_vec_vecs.iter().enumerate() {
        let mut common_chars: HashSet<char> = HashSet::new();
        let mut first_string = true;

        for &string in block {
            // skip the first string in the block for the initial comparison
            if first_string {
                first_string = false;
                common_chars = string.chars().collect();
            } else {
                // find the intersection of the current set with the previous one
                let current_chars: HashSet<char> = string.chars().collect();
                common_chars = common_chars.intersection(&current_chars).copied().collect();
            }
        }
        println!("Block {}:", block_index + 1);
        println!("Characters present in all strings: {:?}", common_chars);
        // in this case there should only be a single char found
        let single_char: char = common_chars.into_iter().next().unwrap();
        let item_priority = char_to_num(single_char);
        sum_total += item_priority;
    }
    return sum_total;
}

fn char_to_num(c: char) -> i32 {
    if c.is_alphabetic() {
        if c.is_lowercase() {
            return c as i32 - 'a' as i32 + 1;
        } else {
            return c as i32 - 'A' as i32 + 27;
        }
    } else {
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn is_part_1_working() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, 157);
    }
    #[test]
    fn is_part_2_working() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, 70);
    }
}
