advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = parse_input(input);
    input.0.sort();
    input.1.sort();
    Some(
        input
            .0
            .iter()
            .zip(input.1.iter())
            .map(|(a, b)| if a < b { b - a } else { a - b })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let result = input.0.iter().fold(0u64, |acc, v| {
        let count = input.1.iter().filter(|&x| x == v).count();
        acc + (count as u64 * v)
    });
    Some(result)
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .fold((vec![], vec![]), |acc, v| {
            let mut acc = acc;
            acc.0.push(v[0]);
            acc.1.push(v[1]);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
