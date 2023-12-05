const DAY: &str = "01";

fn main() -> anyhow::Result<()> {
    let lines = aoc_2023::read_input_lines(DAY).expect("boom");
    Ok(day_01(lines)?)
}

fn day_01<I, T, E>(lines: I) -> anyhow::Result<(), E>
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
    use super::*;

    #[test]
    fn doesnt_blow_up() {
        let input = aoc_2023::read_test_input_lines(DAY).expect("boom");
        assert!(day_01(input).is_ok());
    }
}
