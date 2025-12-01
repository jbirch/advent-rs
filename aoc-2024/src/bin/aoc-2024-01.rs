use std::collections::HashMap;
use std::error::Error;

const DAY: &str = "01";

fn main() -> anyhow::Result<()> {
    let lines = aoc_2024::read_input_lines(DAY)?;
    let result = part_one(lines)?;
    println!("Part 1: {}", { result });

    // I actually have no idea how to clone this lmao
    let lines = aoc_2024::read_input_lines(DAY)?;
    let result = part_two(lines)?;
    println!("Part 2: {}", { result });
    Ok(())
}

fn part_one<I, S, E>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = Result<S, E>>,
    S: AsRef<str>,
    E: Error + Send + Sync + 'static,
{
    let mut left = vec![];
    let mut right = vec![];

    for line in lines {
        let l = line?;
        let split_line: Vec<_> = l.as_ref().split(' ').collect();
        // Why they gotta be separated by three spaces huh
        if split_line.len() != 4 {
            anyhow::bail!("{split_line:?} was not what we expected");
        }

        left.push(
            split_line[0]
                .parse::<u32>()
                .expect("not a number on the left"),
        );
        right.push(
            split_line[3]
                .parse::<u32>()
                .expect("not a number on the right"),
        );
    }

    left.sort_unstable();
    right.sort_unstable();

    if left.len() != right.len() {
        anyhow::bail!("Different lengths somehow, which really should not be possible")
    }

    let mut accum = 0;
    left.iter().zip(right.iter()).for_each(|(l, r)| {
        accum += l.abs_diff(*r);
    });

    Ok(accum)
}

fn part_two<I, S, E>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = Result<S, E>>,
    S: AsRef<str>,
    E: Error + Send + Sync + 'static,
{
    let mut left = vec![];
    let mut right_count = HashMap::new();

    for line in lines {
        let l = line?;
        let split_line: Vec<_> = l.as_ref().split(' ').collect();
        // Why they gotta be separated by three spaces huh
        if split_line.len() != 4 {
            anyhow::bail!("{split_line:?} was not what we expected");
        }

        left.push(
            split_line[0]
                .parse::<u32>()
                .expect("not a number on the left"),
        );
        let r = split_line[3]
            .parse::<u32>()
            .expect("not a number on the right");
        match right_count.get(&r) {
            Some(current) => {
                right_count.insert(r, current + 1);
            }
            None => {
                right_count.insert(r, 1);
            }
        }
    }

    let mut accum = 0;
    for l in left {
        // L things that weren't in R don't have any effect.
        if let Some(r) = right_count.get(&l) {
            accum += l * r;
        }
    }

    Ok(accum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_one() {
        let input = aoc_2024::read_test_input_lines(TEST_INPUT);
        assert_eq!(part_one(input).unwrap(), 11);
    }

    #[test]
    fn test_two() {
        let input = aoc_2024::read_test_input_lines(TEST_INPUT);
        assert_eq!(part_two(input).unwrap(), 31);
    }

    #[test]
    fn actual_one() {
        aoc_2024::ensure_we_look_like_rustrover_runtime();
        let input = aoc_2024::read_input_lines(DAY).unwrap();
        assert_eq!(part_one(input).unwrap(), 1_941_353);
    }

    #[test]
    fn actual_two() {
        aoc_2024::ensure_we_look_like_rustrover_runtime();
        let input = aoc_2024::read_input_lines(DAY).unwrap();
        assert_eq!(part_two(input).unwrap(), 22_539_317);
    }
}
