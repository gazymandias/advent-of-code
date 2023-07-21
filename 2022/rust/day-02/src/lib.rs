// here i decide to use tuple matching
pub fn process_part1(input: &str) -> i32 {
    let mut points: i32 = 0;
    let win_bonus: i32 = 6;
    let draw_bonus: i32 = 3;
    let rock_score: i32 = 1;
    let paper_score: i32 = 2;
    let scissors_score: i32 = 3;
    for line in input.lines() {
        let (opponent_choice, your_choice) = line.rsplit_once(' ').unwrap();
        match (opponent_choice, your_choice) {
            ("A", "X") => points += rock_score + draw_bonus,
            ("A", "Y") => points += paper_score + win_bonus,
            ("A", "Z") => points += scissors_score,

            ("B", "X") => points += rock_score,
            ("B", "Y") => points += paper_score + draw_bonus,
            ("B", "Z") => points += scissors_score + win_bonus,

            ("C", "X") => points += rock_score + win_bonus,
            ("C", "Y") => points += paper_score,
            ("C", "Z") => points += scissors_score + draw_bonus,
            _ => println!("Something is broken {}", your_choice),
        }
    }
    return points;
}

pub fn process_part2(input: &str) -> i32 {
    let mut points: i32 = 0;
    let win_bonus: i32 = 6;
    let draw_bonus: i32 = 3;
    let rock_score: i32 = 1;
    let paper_score: i32 = 2;
    let scissors_score: i32 = 3;
    for line in input.lines() {
        let (opponent_choice, your_choice) = line.rsplit_once(' ').unwrap();
        match (opponent_choice, your_choice) {
            ("A", "X") => points += scissors_score, 
            ("A", "Y") => points += rock_score + draw_bonus,
            ("A", "Z") => points += paper_score + win_bonus,

            ("B", "X") => points += rock_score,
            ("B", "Y") => points += paper_score + draw_bonus,
            ("B", "Z") => points += scissors_score + win_bonus,

            ("C", "X") => points += paper_score,
            ("C", "Y") => points += scissors_score + draw_bonus,
            ("C", "Z") => points += rock_score + win_bonus,
            _ => println!("Something is broken {}", your_choice),
        }
    }
    return points;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn is_part_1_working() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, 15);
    }
    #[test]
    fn is_part_2_working() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, 12);
    }
}
