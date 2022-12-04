fn main() {
    let input = std::fs::read_to_string("./aoc-2022/inputs/01").expect("No input");
    let calories = part_one(input.clone()).expect("aoc-2022-01 exploded");
    println!("Fattest elf: {}", calories);
    let fatties = part_two(input).expect("aoc-2022-01 exploded");
    println!("Fatties: {}", fatties);

}

fn part_one(input: String) -> Result<i32, Explode> {
    let totals: Vec<i32> = input
        .split("\n\n")
        .map(|elf| eat_the_whole_elf(elf).expect("Bad Elf"))
        .collect();

    totals.iter().max().cloned().ok_or(Explode{})
}

fn part_two(input: String) -> Result<i32, Explode> {
    let mut totals: Vec<i32> = input
        .split("\n\n")
        .map(|elf| eat_the_whole_elf(elf).expect("Bad Elf"))
        .collect();

    totals.sort();

    Ok(totals.iter().rev().take(3).sum())

}

fn eat_the_whole_elf(elf: &str) -> Result<i32, Explode> {
    let mut calories= 0;
    for entry in elf.split("\n") {
        calories += entry.parse::<i32>().expect("Not a number")
    }

    Ok(calories)
}

#[derive(Debug)]
struct Explode {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_no_explode() {
        assert!(part_one(String::from("0000")).is_ok())
    }

    #[test]
    fn two_no_explode() {
        assert!(part_two(String::from("0000")).is_ok())
    }
}
