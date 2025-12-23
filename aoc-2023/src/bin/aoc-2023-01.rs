use std::error::Error;
use std::sync::OnceLock;

use regex::Regex;

const DAY: &str = "01";
const DIGIT: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() -> anyhow::Result<()> {
    println!("1: {}", day_01(aoc_2023::read_input_lines(DAY)?)?);
    println!("2: {}", day_01_pt2(aoc_2023::read_input_lines(DAY)?)?);
    Ok(())
}

fn day_01<I, S, E>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = Result<S, E>>,
    S: AsRef<str>,
    E: Error + Send + Sync + 'static,
{
    let mut r = 0;
    for line in lines {
        let l = line?;
        match extract_number(l.as_ref()) {
            Some((left, right)) => {
                r += 10 * left + right;
            }
            _ => {
                anyhow::bail!("Line without a value: {}", l.as_ref());
            }
        }
    }
    Ok(r)
}

const DECIMAL: u32 = 10;

fn extract_number(s: &str) -> Option<(u32, u32)> {
    let l = s.chars().nth(s.find(DIGIT)?)?;
    let r = s.chars().nth(s.rfind(DIGIT)?)?;
    Some((l.to_digit(DECIMAL)?, r.to_digit(DECIMAL)?))
}

fn day_01_pt2<I, S, E>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = Result<S, E>>,
    S: AsRef<str>,
    E: Error + Send + Sync + 'static,
{
    let mut r = 0;
    for line in lines {
        let l = line?;
        match extract_number_pt2(l.as_ref()) {
            Some((left, right)) => {
                r += 10 * left + right;
            }
            _ => {
                anyhow::bail!("Line without a value: {}", l.as_ref());
            }
        }
    }
    Ok(r)
}

const NUMBERS: &str = r"one|two|three|four|five|six|seven|eight|nine|[0-9]";

fn extract_number_pt2(s: &str) -> Option<(u32, u32)> {
    static R: OnceLock<Regex> = OnceLock::new();
    let re = R.get_or_init(|| Regex::new(NUMBERS).unwrap());

    // a naive match is enough to find the left-most match
    let mut l_match = re.find(s);
    let l = l_match?.as_str();

    // We can find the right-most match by taking candidates until successive searches return
    // nothing more. It'll be _at least_ `l`, but we'll double assign at first. Is there a 'do
    // while let' loop or something...
    let mut r: &str = l;
    while let Some(m) = l_match {
        r = l_match?.as_str();
        l_match = re.find_at(s, m.start() + 1);
    }
    Some((string_to_number(l)?, string_to_number(r)?))
}

fn string_to_number(s: &str) -> Option<u32> {
    match s {
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doesnt_blow_up() {
        let input = aoc_2023::read_test_input_lines(DAY).expect("boom");
        assert!(day_01(input).is_ok());
    }

    #[test]
    fn sample_works() -> anyhow::Result<()> {
        let input = aoc_2023::read_test_input_lines(DAY)?;
        assert_eq!(day_01(input)?, 142);
        Ok(())
    }

    #[test]
    fn second_sample_works() -> anyhow::Result<()> {
        // TODO: what the fuck is this stringery lmao. Consider maybe not having test files inline
        //  and just providing it as an iterator of lines over a string? This would unfuck the
        //  shenanigans done for testing bullshizz...
        let input = aoc_2023::read_test_input_lines((String::from(DAY) + "-2").as_str())?;
        assert_eq!(day_01_pt2(input)?, 281);
        Ok(())
    }

    #[test]
    fn only_one_number_to_capture() {
        let s = "abcd1dcba";
        assert_eq!(extract_number_pt2(s).unwrap(), (1, 1));
    }

    #[test]
    fn overlapping_number_names() {
        let s = "xtwonex";
        assert_eq!(extract_number_pt2(s).unwrap(), (2, 1));
    }
}
