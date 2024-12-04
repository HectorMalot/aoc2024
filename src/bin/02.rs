advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(safe)
        .filter(|&x| x)
        .count();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(permutate)
        .map(|elems| elems.into_iter().map(safe).any(|x| x))
        .filter(|&x| x)
        .count();

    Some(result as u32)
}

enum Status {
    Safe(Direction),
    Unsafe,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
}

fn safe(input: Vec<u32>) -> bool {
    let (a, b) = (input[0], input[1]);
    let first_direction = if a < b && b - a <= 3 {
        Direction::Up
    } else if a > b && a - b <= 3 {
        Direction::Down
    } else {
        return false;
    };

    input
        .windows(2)
        .map(|w| {
            let (a, b) = (w[0], w[1]);
            if a < b && b - a <= 3 {
                Status::Safe(Direction::Up)
            } else if a > b && a - b <= 3 {
                Status::Safe(Direction::Down)
            } else {
                Status::Unsafe
            }
        })
        .all(|x| match x {
            Status::Safe(dir) => dir == first_direction,
            Status::Unsafe => false,
        })
}

fn permutate(input: Vec<u32>) -> Vec<Vec<u32>> {
    let mut result = vec![];
    for i in 0..input.len() {
        let mut clone = input.clone();
        clone.remove(i);
        result.push(clone);
    }
    result.push(input);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
