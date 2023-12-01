use std::io;

#[derive(Debug)]
struct AocError(String);

fn main() -> Result<(), io::Error> {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output).ok();
    Ok(())
}
fn part1(input: &str) -> Result<u32, AocError> {
    Ok(input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character| character.to_digit(10));
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let result = part1(input)?;
        assert_eq!(result, 142);
        Ok(())
    }
}
