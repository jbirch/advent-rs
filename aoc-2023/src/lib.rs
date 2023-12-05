use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

/// Do a dodgy for rustrover.
///
/// This is a function that can be called from the doctests to make the doctests behave as if
/// they were called in the same way as binaries are invoked. This is a crappy solution to me not
/// understanding why rustrover gives a different current working directory to binaries, tests,
/// and doctests in multi-module projects.
///
/// Ideally it would be private, but it needs to be public to be used in doctests to make them
/// run, even if the invocation is hidden. Just don't call this yourself.
///
/// It'll stop walking once it gets to the root of "../aoc-2023/".
///
/// # Panics
///
/// If something is extremely extremely fucked. Like you're running in a directory that got
/// deleted or something? I honestly don't know what might fuck this up. But don't call this
/// yourself ay.
pub fn ensure_we_look_like_rustrover_runtime() {
    let current_dir = env::current_dir().expect("That's really not good");
    if current_dir.ends_with(PathBuf::from("advent-rs")) {
        return;
    }
    let parent_dir = current_dir.parent().expect("That's also really not good");
    env::set_current_dir(parent_dir).expect("That's fucked up");
}

/// Do a dodgy for rustrover.
///
/// This is a function that can be called from the tests repeatedly in case you ever called the
/// other version of this function. It's a really shitty solution, I'll grant you that. Perhaps
/// there's a way to run all the doctests at once, so I'm giving it the same restrictions as the
/// other version.
///
/// Ideally it would be private, but it needs to be public to be used in doctests to make them
/// run, even if the invocation is hidden. Just don't call this yourself.
///
/// # Panics
///
/// If something is extremely extremely fucked. Like you're running in a directory that got
/// deleted or something? I honestly don't know what might fuck this up. But don't call this
/// yourself ay.
pub fn ensure_we_look_like_rustrover_testtime() {
    ensure_we_look_like_rustrover_runtime();
    let subdir = PathBuf::from("aoc-2023");
    env::set_current_dir(subdir.to_str().expect("not great")).expect("really not great");
}

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
/// # aoc_2023::ensure_we_look_like_rustrover_runtime();
/// let whole_file = aoc_2023::read_input("00").expect("boom");
/// println!("{}",{whole_file})
/// ```
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
/// # aoc_2023::ensure_we_look_like_rustrover_runtime();
/// let lines = aoc_2023::read_input_lines("00").expect("boom");
/// for l in lines.into_iter().flatten() {
///     println!("{l}");
/// }
/// ```
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
/// # aoc_2023::ensure_we_look_like_rustrover_testtime();
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
/// # aoc_2023::ensure_we_look_like_rustrover_testtime();
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

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_DAY: &str = "00";
    const INVALID_DAY: &str = "bonghits";

    #[test]
    fn read_reads() {
        ensure_we_look_like_rustrover_runtime();
        let input = read_input(VALID_DAY);
        assert!(input.is_ok());
    }

    #[test]
    fn read_explodes() {
        ensure_we_look_like_rustrover_runtime();
        let input = read_input(INVALID_DAY);
        assert!(input.is_err());
    }

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
    fn read_test_reads() {
        ensure_we_look_like_rustrover_testtime();
        let input = read_test_input(VALID_DAY);
        assert!(input.is_ok());
    }

    #[test]
    fn read_test_explodes() {
        ensure_we_look_like_rustrover_testtime();
        let input = read_test_input(INVALID_DAY);
        assert!(input.is_err());
    }

    #[test]
    fn read_test_lines_reads() {
        ensure_we_look_like_rustrover_testtime();
        let input = read_test_input_lines(VALID_DAY);
        assert!(input.is_ok());
    }

    #[test]
    fn read_test_lines_explodes() {
        ensure_we_look_like_rustrover_testtime();
        let input = read_test_input_lines(INVALID_DAY);
        assert!(input.is_err());
    }

    #[test]
    fn read_test_lines_reads_lines() {
        ensure_we_look_like_rustrover_testtime();
        let input = read_test_input_lines(VALID_DAY).expect("not good");
        assert_eq!(input.collect::<Vec<_>>().len(), 2);
    }
}
