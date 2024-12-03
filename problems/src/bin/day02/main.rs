use adventofcode2024_common::StrExt;

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

#[derive(Debug, Default, Copy, Clone)]
enum Mode {
    #[default]
    Initial,
    Undecided(i32),
    Increasing(i32),
    Decreasing(i32),
}

impl Mode {
    fn next(self, next: i32) -> Result<Mode, ()> {
        match self {
            Mode::Initial => Ok(Mode::Undecided(next)),
            Mode::Undecided(m) | Mode::Increasing(m) if m - next > 0 && m - next <= 3 => {
                Ok(Mode::Increasing(next))
            }
            Mode::Undecided(m) | Mode::Decreasing(m) if next - m > 0 && next - m <= 3 => {
                Ok(Mode::Decreasing(next))
            }
            _ => Err(()),
        }
    }
}

fn is_safe_without_dampener(report: impl AsRef<str>) -> bool {
    report
        .as_ref()
        .split_ascii_whitespace()
        .map(|s| s.to_i32())
        .try_fold(Mode::Initial, Mode::next)
        .is_ok()
}

fn is_safe_with_dampener(report: impl AsRef<str>) -> bool {
    fn is_safe_inner(report_levels: &[i32], i: usize, errors: usize, mode: Mode) -> bool {
        if errors > 1 {
            return false;
        }
        if i == report_levels.len() {
            return true;
        }
        match mode.next(report_levels[i]) {
            Ok(next) if is_safe_inner(report_levels, i + 1, errors, next) => true,
            _ => is_safe_inner(report_levels, i + 1, errors + 1, mode),
        }
    }

    let report_levels = report
        .as_ref()
        .split_ascii_whitespace()
        .map(|s| s.to_i32())
        .collect::<Vec<_>>();

    is_safe_inner(&report_levels, 0, 0, Mode::Initial)
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|report| is_safe_without_dampener(report))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|report| is_safe_with_dampener(report))
        .count()
}

#[test]
fn part1_example() {
    assert_eq!(2, part1(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(356, part1(INPUT));
}

#[test]
fn part2_example() {
    assert_eq!(4, part2(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(413, part2(INPUT));
}
