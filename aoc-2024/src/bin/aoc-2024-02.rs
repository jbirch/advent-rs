use std::error::Error;

const DAY: &str = "02";

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
    let mut accum = 0;
    for line in lines {
        let l = line?.as_ref().to_owned();
        let report = l.split(' ').filter_map(|s| s.parse::<i32>().ok()).collect();
        if safe(&report) {
            accum += 1;
        }
    }
    Ok(accum)
}

/// A safe report is one that strictly increases or decreases, with gaps of only 1, 2, or 3.
///
/// This is equivalent to asking if the differences are all positive numbers between 1 and 3, or
/// all negative numbers between -1 and -3.
fn safe(report: &Vec<i32>) -> bool {
    let shifted_report = report.iter().skip(1);

    let diffs: Vec<_> = report
        .iter()
        .zip(shifted_report)
        .map(|(n, c)| n - c)
        .collect();
    diffs.iter().all(|i| *i > 0 && *i < 4) || diffs.iter().all(|i| *i > -4 && *i < 0)
}

/// A dampened-safe report is one that strictly increases or decreases, with gaps of only 1, 2,
/// or 3 OR could do so if one particular level didn't exist.
///
/// This second case can only happen if there is a mix of positive and negative differences, as
/// differences that are all the same sign would only result in larger differences when an item
/// is removed. It's annoying to do that though, so just be exhaustive on them.
///
/// I *think* it might be possible to like, look for any *one* difference that's the opposite sign
/// (or zero) of the others, and then if that is within some range of its neighbours then it
/// represents a difference that could be cancelled out by removing that item, but I cannot brain
/// today.
fn dampened_safe(report: &Vec<i32>) -> bool {
    // if it's already safe, or...
    if safe(report) {
        return true;
    }

    // if we can make it safe by yeeting a single element.
    for (i, _) in report.iter().enumerate() {
        let mut dampened_report = report.clone();
        dampened_report.remove(i);
        if safe(&dampened_report) {
            return true;
        }
    }
    false
}

fn part_two<I, S, E>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = Result<S, E>>,
    S: AsRef<str>,
    E: Error + Send + Sync + 'static,
{
    let mut accum = 0;
    for line in lines {
        let l = line?.as_ref().to_owned();
        let report = l.split(' ').filter_map(|s| s.parse::<i32>().ok()).collect();
        if dampened_safe(&report) {
            accum += 1;
        }
    }
    Ok(accum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_one() {
        let input = aoc_2024::read_test_input_lines(TEST_INPUT);
        assert_eq!(part_one(input).unwrap(), 2);
    }

    #[test]
    fn test_two() {
        let input = aoc_2024::read_test_input_lines(TEST_INPUT);
        assert_eq!(part_two(input).unwrap(), 4);
    }

    #[test]
    fn actual_one() {
        aoc_2024::ensure_we_look_like_rustrover_runtime();
        let input = aoc_2024::read_input_lines(DAY).unwrap();
        assert_eq!(part_one(input).unwrap(), 371);
    }

    #[test]
    fn actual_two() {
        aoc_2024::ensure_we_look_like_rustrover_runtime();
        let input = aoc_2024::read_input_lines(DAY).unwrap();
        assert_eq!(part_two(input).unwrap(), 426);
    }
}
