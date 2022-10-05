use std::str::FromStr;

use aoc_plumbing::Problem;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct {{project-name|upper_camel_case}};

impl FromStr for {{project-name|upper_camel_case}} {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self)
    }
}

impl Problem for {{project-name|upper_camel_case}} {
    const DAY: usize = {{day}};
    const TITLE: &'static str = "{{project-name|title_case|downcase}}";

    type P1 = &'static str;
    type P2 = &'static str;

    fn part_one(&mut self) -> Self::P1 {
        "not implemented"
    }

    fn part_two(&mut self) -> Self::P2 {
        "not implemented"
    }
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = {{project-name|upper_camel_case}}::solve(input).unwrap();
        assert_eq!(solution, Solution::new("not implemented", "not implemented"));
    }

    #[test]
    fn example() {
        let input = "";
        let solution = {{project-name|upper_camel_case}}::solve(input).unwrap();
        assert_eq!(solution, Solution::new("not implemented", "not implemented"));
    }
}