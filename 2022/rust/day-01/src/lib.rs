pub fn process_part1(input: &str) -> i32 {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let mut block_sums: Vec<i32> = Vec::new();
    for block in blocks {
        let sum: i32 = block
            .lines()
            .filter_map(|line| line.parse::<i32>().ok()) // parse lines as i32 and ignore invalids
            .sum();
        block_sums.push(sum)
    }

    // // we can use debug but need to ensure we pass a reference of block_sums so as not to consume the vector!
    // dbg!(&block_sums);

    if let Some(max_value) = block_sums.iter().max() {
        return *max_value;
    } else {
        return 0;
    };
}

pub fn process_part2(input: &str) -> i32 {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let mut block_sums: Vec<i32> = Vec::new();
    for block in blocks {
        let sum: i32 = block
            .lines()
            .filter_map(|line| line.parse::<i32>().ok())
            .sum(); //
        block_sums.push(sum)
    }

    block_sums.sort();
    let sum = block_sums.iter().rev().take(3).sum();
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn is_part_1_working() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, 24000);
    }
    #[test]
    fn is_part_2_working() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, 45000);
    }
}
