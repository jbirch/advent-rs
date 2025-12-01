use std::collections::HashSet;
use std::ops::RangeInclusive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./aoc-2022/inputs/03")?;
    println!("One: {:?}", part_one(&input)?);
    println!("Two: {:?}", part_two(&input)?);
    Ok(())
}

fn part_one(input: &str) -> anyhow::Result<u32> {
    let mut total = 0;
    for line in input.split('\n') {
        let (left, right) = line.split_at(line.len() / 2);
        total += priority(intersection(left, right)?)?;
    }

    Ok(total)
}

/// This will be inelegant with copy pasta because I ceebs
fn part_two(input: &str) -> anyhow::Result<u32> {
    let mut accum: u32 = 0;
    let lines: Vec<&str> = input.split('\n').collect();
    let chunks = lines.chunks(3);
    for chunk in chunks {
        if chunk.len() != 3 {
            anyhow::bail!("Each chunk needs to be of length three")
        }

        let left_set: HashSet<char> = chunk.first().expect("bad").chars().collect();
        let middle_set: HashSet<char> = chunk.get(1).expect("bad").chars().collect();
        let right_set: HashSet<char> = chunk.get(2).expect("bad").chars().collect();

        let left_intersection: HashSet<char> = left_set.intersection(&middle_set).copied().collect();
        let overlap: HashSet<&char> = left_intersection.intersection(&right_set).collect();


        if overlap.len() != 1 {
            anyhow::bail!("wrong number of intersecting items: {}", overlap.len())
        }

        accum += priority(**overlap.iter().next().expect("somehow empty"))?;
    }

    Ok(accum)
}

/// Get the single intersection character in two equal-sized strings.
///
/// If there is not exactly one character the same between two equal-length
/// strings, then explode.
fn intersection(left: &str, right: &str) -> anyhow::Result<char> {
    if left.len() != right.len() {
        anyhow::bail!("uneven split: {} {}", left.len(), right.len())
    }

    let left_set: HashSet<char> = left.chars().collect();
    let right_set: HashSet<char> = right.chars().collect();

    let overlap: HashSet<&char> = left_set.intersection(&right_set).collect();
    if overlap.len() != 1 {
        anyhow::bail!("wrong number of intersecting items: {}", overlap.len())
    }

    Ok(**overlap.iter().next().expect("somehow empty"))
}

static VALID_UPPERCASE: RangeInclusive<u32> = 65..=90;
static VALID_LOWERCASE: RangeInclusive<u32> = 97..=122;

/// Map a given ASCII character to a priority value.
///
/// Priorities run from a..z,A..Z, for values 1..26,27..52.
fn priority(letter: char) -> anyhow::Result<u32> {
    let l = letter as u32;
    match letter {
        _ if VALID_UPPERCASE.contains(&l) => Ok(l - 38),
        _ if VALID_LOWERCASE.contains(&l) => Ok(l - 96),
        _ => anyhow::bail!("Out of bounds: {letter}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(priority('A')?, 27);
        assert_eq!(priority('Z')?, 52);
        assert_eq!(priority('a')?, 1);
        assert_eq!(priority('z')?, 26);
        assert!(priority('`').is_err());
        Ok(())
    }

    #[test]
    fn test_intersection() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(intersection("abc", "ced")?, 'c');
        assert_eq!(intersection("Abcd", "qweA")?, 'A');
        assert!(intersection("abc", "ABC").is_err()); // There's always one duplicate
        assert!(intersection("abc", "a").is_err()); // Strings are the same length
        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<(), Box<dyn std::error::Error>> {
        // The current directory isn't the root of the workspace in tests in CLion for reasons I
        // don't know.
        let test_input = std::fs::read_to_string("./inputs/03_test")?;
        assert_eq!(part_one(&*test_input)?, 157);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn std::error::Error>> {
        // The current directory isn't the root of the workspace in tests in CLion for reasons I
        // don't know.
        let test_input = std::fs::read_to_string("./inputs/03_test")?;
        assert_eq!(part_two(&*test_input)?, 70);
        Ok(())
    }
}