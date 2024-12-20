use std::fs;

fn main() {
    // let input: Vec<Vec<u32>> = vec![
    //     vec![7, 6, 4, 2, 1],
    //     vec![1, 2, 7, 8, 9],
    //     vec![9, 7, 6, 2, 1],
    //     vec![1, 3, 2, 4, 5],
    //     vec![8, 6, 4, 4, 1],
    //     vec![1, 3, 6, 7, 9],
    // ];

    let input = extract_input();

    println!("Part-1: Output: {}", red_nosed_reports(input.clone()));
    println!("Part-2: Output: {}", red_nosed_reports_with_dampener(input));
}

fn red_nosed_reports(input: Vec<Vec<u32>>) -> u32 {
    let mut result = 0;

    for report in input {
        if report.len() < 2 {
            continue;
        }

        let is_decreasing = report[0] > report[1];
        let mut is_safe = true;

        for i in 1..report.len() {
            if (report[i - 1] > report[i]) != is_decreasing {
                is_safe = false;
                break;
            }

            let diff = report[i - 1].abs_diff(report[i]);

            if diff < 1 || diff > 3 {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            result += 1;
        }
    }

    result
}

fn red_nosed_reports_with_dampener(input: Vec<Vec<u32>>) -> u32 {
    let mut result = 0;

    for report in input {
        if report.len() < 2 {
            continue;
        }

        if report_is_safe(&report) {
            result += 1;
            continue;
        }

        let mut dampened_safe = false;

        for i in 0..report.len() {
            let mut temp = report.clone();

            temp.remove(i);

            if report_is_safe(&temp) {
                dampened_safe = true;
                break;
            };
        }

        if dampened_safe {
            result += 1;
        }
    }

    result
}

fn report_is_safe(report: &Vec<u32>) -> bool {
    if report.len() < 2 {
        return false;
    }

    let is_decreasing = report[0] > report[1];

    for i in 1..report.len() {
        if (report[i - 1] > report[i]) != is_decreasing {
            return false;
        }

        let diff = report[i - 1].abs_diff(report[i]);

        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}
fn extract_input() -> Vec<Vec<u32>> {
    let mut input = vec![];

    let input_file = fs::read_to_string("input.txt").expect("Failed to read input file");

    for line in input_file.lines() {
        let numbers = line.split_whitespace();

        let numbers: Vec<u32> = numbers
            .map(|num| num.parse::<u32>().expect("Failed to parse num to u32"))
            .collect();

        input.push(numbers);
    }

    input
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input: Vec<Vec<u32>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let expected = 2;

        assert_eq!(red_nosed_reports(input), expected);
    }
    #[test]
    fn example_part_2() {
        let input: Vec<Vec<u32>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let expected = 4;

        assert_eq!(red_nosed_reports_with_dampener(input), expected);
    }
}
