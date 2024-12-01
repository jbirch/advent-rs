use std::env;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

/// Do a dodgy for rustrover.
///
/// This is a function that can be called from the tests and doctests to make them behave as if
/// they were called in the same way as binaries are invoked. This is a crappy solution to me not
/// understanding why rustrover gives a different current working directory to binaries, tests,
/// and doctests in multi-module projects.
///
/// Ideally it would be private, but it needs to be public to be used in doctests to make them
/// run, even if the invocation is hidden, as well as tests.
///
/// It'll stop walking once it gets to the root of "../aoc-2024/".
///
/// # Panics
///
/// If something is extremely extremely fucked. Like you're running in a directory that got
/// deleted or something? I honestly don't know what might fuck this up. But don't call this
/// yourself ay.
pub fn ensure_we_look_like_rustrover_runtime() {
    let current_dir = env::current_dir().expect("That's really not good");
    // Already here
    if current_dir.ends_with(PathBuf::from("advent-rs")) {
        return;
    }
    let parent_dir = current_dir.parent().expect("That's also really not good");
    env::set_current_dir(parent_dir).expect("That's fucked up");
}

#[derive(thiserror::Error, Debug)]
pub struct AoCError {}

impl Display for AoCError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Oh no.")
    }
}

/// Return, as a bunch of buffered lines, the content of the day's input file.
///
/// This is done relative to the base project directory, as naively expected by rustrover's "run the
/// main function of this binary please" functionality.
///
/// # Arguments
///
/// * `f`: Name of the file to load - conventionally the advent day, but might be something like
/// "01-2" in the case of days that have two parts.
///
/// Returns: `Result<Lines<BufReader<File>>, std::io::Error>`
///
/// # Examples
///
/// ```
/// let lines = aoc_2024::read_input_lines("00").expect("boom");
/// for l in lines.into_iter().flatten() {
///     println!("{l}");
/// }
/// ```
///
/// # Errors
///
/// Whatever `File::open` does. Be wary of the current directory.
pub fn read_input_lines(f: &str) -> Result<Lines<BufReader<File>>, std::io::Error> {
    let file = File::open(format!("./aoc-2024/inputs/{f}"))?;
    Ok(BufReader::new(file).lines())
}

/// Return an iterator over the lines of the test string.
///
/// Frankly, it's annoying to write this function, what is essentially a one-liner that's more
/// clear in context than this function could ever be. Perhaps I'll delete it one day after using
/// it a bit. It exists only as a comparable test analogue to `read_input_lines`, pretending that
/// each line might fail to be read, even though it never will.
///
/// # Arguments
///
/// * `input`: The complete input for a problem, as a single string.
///
/// Returns: `core::std::iter::Lines<'a>`
///
/// # Examples
///
/// ```
/// let lines = aoc_2024::read_test_input_lines("A 1\nB 2");
/// for l in lines.into_iter().flatten() {
///     println!("{l}");
/// }
/// ```
#[must_use = "Don't make an iterator and not make it do some work"]
pub fn read_test_input_lines(input: &str) -> Box<dyn Iterator<Item = Result<&str, AoCError>> + '_> {
    Box::new(input.lines().map(Ok))
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_DAY: &str = "00";
    const INVALID_DAY: &str = "bonghits";

    #[test]
    fn read_lines_reads() {
        ensure_we_look_like_rustrover_runtime();
        let input = read_input_lines(VALID_DAY);
        assert!(input.is_ok());
    }

    #[test]
    fn read_lines_explodes() {
        ensure_we_look_like_rustrover_runtime();
        let input = read_input_lines(INVALID_DAY);
        assert!(input.is_err());
    }

    #[test]
    fn read_lines_reads_lines() {
        ensure_we_look_like_rustrover_runtime();
        let input = read_input_lines(VALID_DAY).expect("not good");
        assert_eq!(input.collect::<Vec<_>>().len(), 4);
    }

    #[test]
    fn read_test_lines_reads_lines() {
        let input = "A 1\nB 2\nC 3\nD 4";
        let lines = read_test_input_lines(input);
        assert_eq!(lines.collect::<Vec<_>>().len(), 4);
    }

    #[test]
    fn read_test_lines_empty() {
        let empty_input = "";
        let lines = read_test_input_lines(empty_input);
        assert_eq!(lines.collect::<Vec<_>>().len(), 0);
    }

    #[test]
    fn read_test_lines_always_is_ok() {
        let input = "A 1\nB 2\nC 3\nD 4";
        let lines = read_test_input_lines(input);
        for l in lines {
            assert!(l.is_ok());
        }
    }
}
