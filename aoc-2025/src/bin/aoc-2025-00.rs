use std::error::Error;

const DAY: &str = "00";

fn main() -> anyhow::Result<()> {
    let lines = aoc_2025::read_input_lines(DAY)?;
    let result = day_00(lines)?;
    println!("{}", { result });
    Ok(())
}

fn day_00<I, S, E>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = Result<S, E>>,
    S: AsRef<str>,
    E: Error + Send + Sync + 'static,
{
    for line in lines {
        let _ = line?.as_ref();
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A 1\nB 2\nC 3\nD 4";

    #[test]
    fn test_ok() {
        let input = aoc_2025::read_test_input_lines(TEST_INPUT);
        assert!(day_00(input).is_ok());
    }

    #[test]
    fn test_boom() {
        let input = vec![Ok("A 1"), Err(aoc_2025::AoCError {})];
        assert!(day_00(input).is_err());
    }
}
