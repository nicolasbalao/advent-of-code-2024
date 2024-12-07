use std::fs;

fn main() {
    let input = vec![
        "MMMSXXMASM".to_string(),
        "MSAMXMSMSA".to_string(),
        "AMXSXMAAMM".to_string(),
        "MSAMASMSMX".to_string(),
        "XMASAMXAMM".to_string(),
        "XXAMMXXAMA".to_string(),
        "SMSMSASXSS".to_string(),
        "SAXAMASAAA".to_string(),
        "MAMMMXMMMM".to_string(),
        "MXMXAXMASX".to_string(),
    ];
    let input = process_input();

    let result = count_xmas(input.clone());
    println!("XMAS appears {} times.", result);

    let result = count_x_mas(input.clone());
    println!("X-MAS appears {} times.", result)
}

fn count_xmas(input: Vec<String>) -> usize {
    let directions: Vec<(isize, isize)> = vec![
        // Right
        (0, 1),
        // left
        (0, -1),
        // Dow
        (1, 0),
        // Up
        (-1, 0),
        // Diag down-right
        (1, 1),
        // Diag up-right
        (-1, 1),
        // Diag down-left
        (1, -1),
        // Diag up-left
        (-1, -1),
    ];

    let word = "XMAS";
    let grid: Vec<Vec<char>> = input.iter().map(|row| row.chars().collect()).collect();

    count_word_direction(grid, word.to_string(), directions)
}

fn count_x_mas(input: Vec<String>) -> usize {
    let grid: Vec<Vec<char>> = input.into_iter().map(|row| row.chars().collect()).collect();

    let nb_rows = grid.len();
    let nb_cols = grid[0].len();

    let mut count = 0;

    for row in 1..nb_rows - 1 {
        for col in 1..nb_cols - 1 {
            let mut found = true;

            if grid[row][col] == 'A' {
                let d_up_row = row - 1;
                let d_down_row = row + 1;
                let d_left_col = col - 1;
                let d_right_col = col + 1;

                let diag_up_left = grid[d_up_row][d_left_col];
                let diag_down_left = grid[d_down_row][d_left_col];

                let diag_up_right = grid[d_up_row][d_right_col];
                let diag_down_right = grid[d_down_row][d_right_col];

                let good_letter = vec!['S', 'M'];

                if !good_letter.contains(&diag_up_left)
                    || !good_letter.contains(&diag_down_left)
                    || !good_letter.contains(&diag_up_right)
                    || !good_letter.contains(&diag_down_right)
                {
                    continue;
                }

                if diag_up_left == diag_down_right || diag_down_left == diag_up_right {
                    continue;
                }
            } else {
                found = false;
            }

            if found {
                count += 1;
            }
        }
    }
    count
}

fn count_word_direction(
    grid: Vec<Vec<char>>,
    word: String,
    directions: Vec<(isize, isize)>,
) -> usize {
    let nb_rows = grid.len();
    let nb_cols = grid[0].len();

    let mut count = 0;

    for row in 0..nb_rows {
        for col in 0..nb_cols {
            for (dy, dx) in &directions {
                let mut found = true;

                for i in 0..word.len() {
                    let nx = row as isize + i as isize * dx;
                    let ny = col as isize + i as isize * dy;

                    if nx < 0
                        || ny < 0
                        || nx >= nb_rows as isize
                        || ny >= nb_cols as isize
                        || grid[nx as usize][ny as usize] != word.chars().nth(i).unwrap()
                    {
                        found = false;
                        break;
                    }
                }

                if found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn process_input() -> Vec<String> {
    let input_file = fs::read_to_string("input.txt").unwrap();

    input_file.lines().map(|line| line.to_string()).collect()
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part_1_example() {
        let input = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];

        let expected = 18;
        assert_eq!(count_xmas(input), expected);
    }
    #[test]
    fn part_2_example() {
        let input = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];

        let expected = 9;
        assert_eq!(count_x_mas(input), expected);
    }
}
