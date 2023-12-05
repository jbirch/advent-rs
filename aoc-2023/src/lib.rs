use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

/// Return, as a giant string, the content of the day's input file.
///
/// This is done relative to the base project directory, as naively expected by rustrover's "run the
/// main function of this binary please" functionality.
///
/// # Arguments
///
/// * `f`: Name of the file to load - conventionally the advent day.
///
/// returns: `Result<String, Error>`
///
/// # Examples
///
/// ```
/// let whole_file = aoc_2023::read_input("00").expect("boom");
/// println!("{}",{whole_file})
/// ```
///
/// TODO: This doctest won't work naively in rustrover because the working directory of a doctest is
///     different from the working directory of an invoked binary within a multi-module project.
///
/// # Errors
///
/// Whatever `std::fs::read_to_string` does. Be wary of the current directory.
pub fn read_input(f: &str) -> anyhow::Result<String> {
    Ok(std::fs::read_to_string(format!("./aoc-2023/inputs/{f}"))?)
}

/// Return, as a bunch of buffered lines, the content of the day's input file.
///
/// This is done relative to the base project directory, as naively expected by rustrover's "run the
/// main function of this binary please" functionality.
///
/// # Arguments
///
/// * `f`: Name of the file to load - conventionally the advent day.
///
/// returns: `Result<Lines<BufReader<File>>, Error>`
///
/// # Examples
///
/// ```
/// let lines = aoc_2023::read_input_lines("00").expect("boom");
/// for l in lines.into_iter().flatten() {
///     println!("{l}");
/// }
/// ```
/// TODO: This doctest won't work naively in rustrover because the working directory of a doctest is
///     different from the working directory of an invoked binary within a multi-module project.
///
/// # Errors
///
/// Whatever `File::open` does. Be wary of the current directory.
pub fn read_input_lines(f: &str) -> anyhow::Result<Lines<BufReader<File>>> {
    let file = File::open(format!("./aoc-2023/inputs/{f}"))?;
    Ok(BufReader::new(file).lines())
}

/// Return, as a giant string, the content of the day's test file.
///
/// This is done relative to the "aoc-2023" directory, as naively expected by rustrover's "run this
/// test please" functionality.
///
/// # Arguments
///
/// * `f`: Name of the file to load - conventionally the advent day.
///
/// returns: `Result<String, Error>`
///
/// # Examples
///
/// ```
/// let whole_file = aoc_2023::read_test_input("00").expect("boom");
/// println!("{}",{whole_file})
/// ```
///
/// # Errors
///
/// Whatever `std::fs::read_to_string` does. Be wary of the current directory.
pub fn read_test_input(f: &str) -> anyhow::Result<String> {
    Ok(std::fs::read_to_string(format!("./inputs/{f}_test"))?)
}

/// Return, as a bunch of buffered lines, the content of the day's test file.
///
/// This is done relative to the "aoc-2023" directory, as naively expected by rustrover's "run this
/// test please" functionality.
///
/// # Arguments
///
/// * `f`: Name of the file to load - conventionally the advent day.
///
/// returns: `Result<Lines<BufReader<File>>, Error>`
///
/// # Examples
///
/// ```
/// let lines = aoc_2023::read_test_input_lines("00").expect("boom");
/// for l in lines.into_iter().flatten() {
///     println!("{l}");
/// }
/// ```
///
/// # Errors
///
/// Whatever `File::open` does. Be wary of the current directory.
pub fn read_test_input_lines(f: &str) -> anyhow::Result<Lines<BufReader<File>>> {
    let file = File::open(format!("./inputs/{f}_test"))?;
    Ok(BufReader::new(file).lines())
}
