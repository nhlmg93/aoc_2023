use std::io;

#[derive(Debug)]
struct AocError(String);

fn main() -> Result<(), io::Error> {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output).ok();
    Ok(())
}
fn part2(input: &str) -> Result<u32, AocError> {
    Ok(input
        .lines()
        .map(|line| {
            let mut it = (0..line.len()).filter_map(|index| {
                let reduced_line = &line[index..];
                let result = if reduced_line.starts_with("one") {
                    '1'
                } else if reduced_line.starts_with("two") {
                    '2'
                } else if reduced_line.starts_with("three") {
                    '3'
                } else if reduced_line.starts_with("four") {
                    '4'
                } else if reduced_line.starts_with("five") {
                    '5'
                } else if reduced_line.starts_with("six") {
                    '6'
                } else if reduced_line.starts_with("seven") {
                    '7'
                } else if reduced_line.starts_with("eight") {
                    '8'
                } else if reduced_line.starts_with("nine") {
                    '9'
                } else {
                    reduced_line.chars().next().unwrap()
                };

                result.to_digit(10)
            });
            let first = it.next().expect("should be number");
            match it.last() {
                Some(num) => format!("{first}{num}").parse::<u32>(),
                None => format!("{first}{first}").parse::<u32>(),
            }
            .expect("should be valid num")
        })
        .sum::<u32>())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() -> Result<(), AocError> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = part2(input)?;
        assert_eq!(result, 281);
        Ok(())
    }
}
