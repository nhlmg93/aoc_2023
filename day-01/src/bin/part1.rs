fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    print!("{}", input);
    input.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("4");
        assert_eq!(result, "4".to_string())
    }
}
