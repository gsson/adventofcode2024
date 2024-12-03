const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

fn consume_num(input: &str) -> Result<(usize, &str), &str> {
    let mut num = 0;
    for (i, c) in input.char_indices().take(3) {
        if let Some(digit) = c.to_digit(10) {
            num = num * 10 + digit as usize;
        } else if i > 0 {
            return Ok((num, &input[i..]));
        } else {
            return Err(input);
        }
    }

    Ok((num, &input[3..]))
}

fn consume_literal<'input>(lit: &str, input: &'input str) -> Result<&'input str, &'input str> {
    if let Some(prefix) = input.strip_prefix(lit) {
        Ok(prefix)
    } else {
        Err(input)
    }
}

fn consume_mul(input: &str) -> Result<(usize, &str), &str> {
    let input = consume_literal("mul(", input)?;
    let (num1, input) = consume_num(input)?;
    let input = consume_literal(",", input)?;
    let (num2, input) = consume_num(input)?;
    let input = consume_literal(")", input)?;

    Ok((num1 * num2, input))
}

fn consume_dodont(input: &str) -> Result<(bool, &str), &str> {
    let input = consume_literal("do", input)?;
    let (enable, input) = match consume_literal("n't", input) {
        Ok(rest) => (false, rest),
        Err(_) => (true, input),
    };
    let input = consume_literal("()", input)?;
    Ok((enable, input))
}

fn parse(mut input: &str) -> usize {
    let mut total = 0;

    while !input.is_empty() {
        if let Ok((mul, rest)) = consume_mul(input) {
            input = rest;
            total += mul;
        } else {
            input = &input[1..];
        }
    }

    total
}

fn parse2(mut input: &str) -> usize {
    let mut total = 0;
    let mut enabled = true;

    while !input.is_empty() {
        if enabled {
            if let Ok((mul, rest)) = consume_mul(input) {
                input = rest;
                total += mul;
            } else if let Ok((enable, rest)) = consume_dodont(input) {
                input = rest;
                enabled = enable;
            } else {
                input = &input[1..];
            }
        } else if let Ok((enable, rest)) = consume_dodont(input) {
            input = rest;
            enabled = enable;
        } else {
            input = &input[1..];
        }
    }
    total
}

fn part1(input: &str) -> usize {
    parse(input)
}

fn part2(input: &str) -> usize {
    parse2(input)
}

#[test]
fn part1_example() {
    assert_eq!(161, part1(include_str!("example1.txt")))
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(184122457, part1(INPUT))
}

#[test]
fn part2_example() {
    assert_eq!(48, part2(include_str!("example2.txt")))
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(107862689, part2(INPUT))
}
