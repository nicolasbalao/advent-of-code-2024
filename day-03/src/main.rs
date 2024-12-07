use std::fs;

use regex::Regex;

fn main() {
    let input =
        String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

    let input = process_input();
    // let input =
    // String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))do()xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");

    println!("Output: {}", mull_it_over_do_dont(input));
}

fn mull_it_over(input: String) -> u32 {
    let mut result = 0;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Failed to create regex");

    let matches: Vec<_> = re
        .find_iter(&input)
        .map(|matche| {
            let first_digit = matche
                .as_str()
                .split("(")
                .nth(1)
                .unwrap()
                .split(",")
                .nth(0)
                .unwrap();
            let second_digit = matche
                .as_str()
                .split("(")
                .nth(1)
                .unwrap()
                .split(",")
                .nth(1)
                .unwrap();

            let second_digit = second_digit.replace(")", "");

            (
                first_digit.parse::<u32>().expect("Failed to parse"),
                second_digit.parse::<u32>().expect("Failed to parse"),
            )
        })
        .collect();

    for (a, b) in matches {
        result += a * b;
    }

    return result;
}

fn mull_it_over_do_dont(mut input: String) -> u64 {
    let mut score: u64 = 0;

    loop {
        println!("------------ INPUT -------------");
        println!("{}", input);
        if let Some(begin) = input.find("don't()") {
            let tmp = String::from(&input[0..begin]);
            input.drain(0..begin);

            if let Some(end) = input.find("do()") {
                println!("------------Process tmp-------------");
                println!("{}", tmp);
                score += mull_it_over(tmp) as u64;
                input.drain(0..end + "do()".len());
            }
        } else {
            score += mull_it_over(input) as u64;
            break;
        }

        println!("");
        println!("");
    }

    score
}

fn process_input() -> String {
    let input_file = fs::read_to_string("input.txt").expect("Failed to open the input file");
    return input_file;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_par_1() {
        let input =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

        let expected = 161;

        assert_eq!(mull_it_over(input), expected);
    }
    #[test]
    fn example_part_2() {
        let input = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );

        let expected = 48;

        assert_eq!(mull_it_over_do_dont(input), expected);
    }
}
