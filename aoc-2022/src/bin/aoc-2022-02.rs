fn main() -> Result<(), Box<dyn std::error::Error>>{
    let input = std::fs::read_to_string("./aoc-2022/inputs/02")?;
    println!("One: {:?}", part_one(input.clone())?);
    println!("Two: {:?}", part_two(input)?);
    Ok(())
}

fn part_one(_input: String) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn part_two(_input: String) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        part_one(String::from("0000")).expect("be good")
    }

    #[test]
    fn test_part_two() {
        part_two(String::from("0000")).expect("be good")
    }
}