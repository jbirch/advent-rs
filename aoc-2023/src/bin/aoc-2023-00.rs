const DAY: &str = "00";

fn main() -> anyhow::Result<()> {
    let lines = aoc_2023::read_input_lines(DAY)?;
    Ok(day_00(lines)?)
}

fn day_00<I, T, E>(lines: I) -> anyhow::Result<(), E>
where
    I: IntoIterator<Item = Result<T, E>>,
    T: AsRef<str>,
{
    lines.into_iter().try_for_each(|line| {
        let _ = line?.as_ref();
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use super::*;

    #[test]
    fn test_ok() {
        let input = aoc_2023::read_test_input_lines(DAY).expect("boom");
        assert!(day_00(input).is_ok());
    }

    #[test]
    fn test_boom() {
        let input = vec![Ok("one"), Err(anyhow!("boom"))];
        assert!(day_00(input).is_err());
    }
}
