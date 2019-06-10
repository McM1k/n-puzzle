pub fn remove_comments(mut lines: Vec<String>) -> Vec<String> {
    //unsafe if used before check_empty_lines

    lines.retain(|ref line| line.chars().nth(0).unwrap() != '#');
    lines.iter_mut().for_each(|ref mut line| {
        let pos = match line.chars().position(|c| c == '#') {
            Some(pos) => pos,
            None => line.len(),
        };
        line.truncate(pos);
    });

    lines
}

pub fn check_empty_lines(lines: &Vec<String>) {
    lines.iter().for_each(|line| {
        if line.is_empty() {
            panic!("There is an empty line in the file")
        }
    })
}

pub fn check_empty_vec(lines: &Vec<String>) {
    if lines.is_empty() {
        panic!("No data.")
    }
}

pub fn check_numbers_or_spaces(lines: &Vec<String>) {
    lines.iter()
		 .for_each(|line| {
		 	if !line.chars().all(|c| c.is_digit(10) || c.is_whitespace()) {
		 		panic!("Unexpected character in file (only spaces and numbers are allowed beside comments)")
		 	}
		 })
}

pub fn check_values_are_incremental(size: &usize, data: &Vec<Vec<u32>>) {
    let mut all_the_values: Vec<usize> = (0..(size * size)).collect();

    data.iter().for_each(|one_line_data| {
        one_line_data.iter().for_each(|value| {
            all_the_values.remove(
                all_the_values
                    .clone()
                    .iter()
                    .position(|&x| x == *value as usize)
                    .unwrap(),
            );
        })
    });
    if !all_the_values.is_empty() {
        panic!("All the values needed for chosen size are not present.")
    }
}

pub fn check_values_form_correct_square(size: &usize, data: &Vec<Vec<u32>>) {
    let mut all_the_values: Vec<usize> = (0..(size * size)).collect();

    //if data.clone().iter().count() != 2;
    data.iter().for_each(|one_line_data| {
        one_line_data.iter().for_each(|value| {
            all_the_values.remove(
                all_the_values
                    .clone()
                    .iter()
                    .position(|&x| x == *value as usize)
                    .unwrap(),
            );
        })
    });
}

/*
fn get_size(lines: & mut Vec<String>) -> u32 {
    let mut first_digit_line = lines.iter()
                                    .nth(0)
                                    .unwrap()
                                    .split_whitespace();
    if first_digit_line.clone().count() != 1 {
        panic!("First line should contain only one number.")
    }
    let size = first_digit_line.nth(0)
                                .unwrap()
                                .parse::<u32>()
                                .expect("Unable to parse size into u32")
                                .unwrap();
    lines.remove(0);

    size
}

pub fn get_data(lines: Vec<String>, size: u32) -> [[u32; size]; size] { // could fail because fo lifetime

}
*/

#[cfg(test)]
mod parser_tests {
    use crate::parser::*;

    mod remove_comments {
        use crate::parser::*;

        #[test]
        #[should_panic]
        fn panic_because_empty_line() {
            let lines: Vec<String> = vec!["".to_string()];
            remove_comments(lines);
        }

        #[test]
        fn no_panic() {
            let lines: Vec<String> = vec!["1".to_string()];
            remove_comments(lines);
        }
    }

    mod check_empty_vec {
        use crate::parser::*;

        #[test]
        #[should_panic]
        fn panic_because_empty_vec() {
            let lines: Vec<String> = vec![];
            check_empty_vec(&lines);
        }

        #[test]
        fn no_panic() {
            let lines: Vec<String> = vec!["1".to_string()];
            check_empty_vec(&lines);
        }
    }

    mod check_values_are_incremental {
        use crate::parser::*;

        #[test]
        #[should_panic]
        fn panic_because_value_out_of_scope() {
            let size: usize = 2;
            let tab = vec![vec![0, 1], vec![2, 5]];
            check_values_are_incremental(&size, &tab);
        }

        #[test]
        #[should_panic]
        fn panic_because_two_time_same_value() {
            let size: usize = 2;
            let tab = vec![vec![0, 1], vec![2, 2]];
            check_values_are_incremental(&size, &tab);
        }

        #[test]
        #[should_panic]
        fn panic_because_too_few_values() {
            let size: usize = 2;
            let tab = vec![vec![0, 1], vec![2]];
            check_values_are_incremental(&size, &tab);
        }

        #[test]
        fn no_panic() {
            let size: usize = 2;
            let tab = vec![vec![0, 1], vec![2, 3]];
            check_values_are_incremental(&size, &tab);
        }
    }

    mod check_values_form_correct_square {
        use crate::parser::*;

        #[test]
        #[should_panic]
        fn panic_because_too_few_values_in_one_line() {
            let size: usize = 2;
            let tab = vec![vec![0, 1], vec![2]];
            check_values_form_correct_square(&size, &tab);
        }

        #[test]
        #[should_panic]
        fn panic_because_too_much_values_in_one_line() {
            let size: usize = 2;
            let tab = vec![vec![0, 1], vec![2, 2, 2]];
            check_values_form_correct_square(&size, &tab);
        }

        #[test]
        #[should_panic]
        fn panic_because_too_few_lines() {
            let size: usize = 2;
            let tab = vec![vec![0, 1]];
            check_values_form_correct_square(&size, &tab);
        }

        #[test]
        #[should_panic]
        fn panic_because_too_much_lines() {
            let size: usize = 2;
            let tab = vec![vec![0, 1], vec![0, 1], vec![0, 1]];
            check_values_form_correct_square(&size, &tab);
        }

        #[test]
        fn no_panic() {
            let size: usize = 2;
            let tab = vec![vec![0, 1], vec![2, 3]];
            check_values_form_correct_square(&size, &tab);
        }
    }
}
