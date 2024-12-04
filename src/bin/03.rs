use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let result = re
        .captures_iter(input)
        .map(|cap| {
            cap[0]
                .trim_start_matches("mul(")
                .trim_end_matches(')')
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|v| v[0] * v[1])
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.replace("\n", "");
    let input = input.replace("don't", ">>>>>");
    let input = input.replace("do", "<<<<<");
    let re = Regex::new(r">>>>>.*?<<<<<").unwrap();
    let result = re.replace_all(&input, "");
    part_one(&result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
