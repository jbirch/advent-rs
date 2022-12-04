fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./aoc-2022/inputs/03")?;
    println!("One: {:?}", part_one(input.clone())?);
    println!("Two: {:?}", part_two(input)?);
    Ok(())
}

fn part_one(_input: String) -> Result<i32, Box<dyn std::error::Error>> {
    Ok(0)
}


fn part_two(_input: String) -> Result<i32, Box<dyn std::error::Error>> {
    Ok(0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<(), Box<dyn std::error::Error>> {
        // Current directory isn't the root of the workspace in tests in CLion for reasons I
        // don't know.
        let test_input = std::fs::read_to_string("./inputs/03_test")?;
        assert_eq!(part_one(test_input)?, 0);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn std::error::Error>> {
        // Current directory isn't the root of the workspace in tests in CLion for reasons I
        // don't know.
        let test_input = std::fs::read_to_string("./inputs/03_test")?;
        assert_eq!(part_one(test_input)?, 0);
        Ok(())
    }
}