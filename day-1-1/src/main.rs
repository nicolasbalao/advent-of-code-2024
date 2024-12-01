use std::{collections::HashMap, fs};

fn main() {
    let (array1, array2) = get_input();

    println!(
        "Result day 1 _ 1: {}",
        day1_1(array1.clone(), array2.clone())
    );
    println!("Result day 1 _ 2: {}", day1_2(array1, array2));
}

fn day1_1(mut array1: Vec<u32>, mut array2: Vec<u32>) -> u32 {
    // 1. sort arrays

    array1.sort();
    array2.sort();

    // 2. Sum diff

    let mut result = 0;

    for i in 0..array1.len() {
        result += array1[i].abs_diff(array2[i]);
    }

    result
}

fn day1_2(array1: Vec<u32>, array2: Vec<u32>) -> u32 {
    let mut num_array2_freq = HashMap::new();

    for num in array2 {
        let value = num_array2_freq.entry(num).or_insert(0);
        *value += 1;
    }

    let mut result = 0;

    for num in array1 {
        result += num_array2_freq.get(&num).unwrap_or(&0) * num;
    }

    result
}

fn get_input() -> (Vec<u32>, Vec<u32>) {
    let mut array1 = vec![];
    let mut array2 = vec![];

    let input_file = fs::read_to_string("input.txt").expect("Failed to read input file");

    for line in input_file.lines() {
        let mut numbers = line.split_ascii_whitespace();

        let num1 = numbers.next().unwrap().parse::<u32>().unwrap();
        let num2 = numbers.next().unwrap().parse::<u32>().unwrap();

        array1.push(num1);
        array2.push(num2);
    }

    (array1, array2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let array1 = vec![3, 4, 2, 1, 3, 3];
        let array2 = vec![4, 3, 5, 3, 9, 3];

        let expected = 11;

        assert_eq!(day1_1(array1, array2), expected);
    }
    #[test]
    fn case_2() {
        let array1 = vec![3, 4, 2, 1, 3, 3];
        let array2 = vec![4, 3, 5, 3, 9, 3];

        let expected = 31;

        assert_eq!(day1_2(array1, array2), expected);
    }
}
