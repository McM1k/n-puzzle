use crate::puzzle::Puzzle;

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

pub fn check_empty_lines(lines: &[String]) {
    lines.iter().for_each(|line| {
        if line.is_empty() {
            panic!("There is an empty line in the file\n")
        }
    })
}

pub fn check_empty_vec(lines: &[String]) {
    if lines.is_empty() {
        panic!("No data.\n")
    }
}

pub fn check_only_numbers_and_spaces(lines: &[String]) {
    lines.iter()
		 .for_each(|line| {
		 	if !line.chars().all(|c| c.is_digit(10) || c.is_whitespace()) {
		 		panic!("Unexpected character in file (only spaces and numbers are allowed beside comments)\n")
		 	}
		 })
}

pub fn check_values_are_incremental(size: usize, data: Vec<usize>) {
    let mut all_the_values: Vec<usize> = (0..(size * size)).collect();

    data.iter().for_each(|value| {
        all_the_values.remove(
            all_the_values
                .clone()
                .iter()
                .position(|&x| x == *value)
                .unwrap(),
        );
    });

    if !all_the_values.is_empty() {
        panic!("All the values needed for chosen size are not present.\n")
    }
}

pub fn check_values_form_correct_square(size: usize, data: Vec<usize>) {
    if size < 3 {
        panic!("Square too small, must be at least 3 of size !\n")
    }
    if data.len() != size * size {
        panic!("Wrong number of lines !\n")
    }
}

pub fn get_data(mut lines: Vec<String>) -> (usize, Vec<usize>) {
    let size = match lines.remove(0).parse::<usize>() {
        Err(_e) => panic!("first line should only contain one value\n"),
        Ok(x) => x,
    };

    let mut data = vec![];
    for line in lines {
        data.append(
            &mut line
                .split_whitespace()
                .map(|token| -> usize {
                    token
                        .parse::<usize>()
                        .expect("Unable to parse data into u32\n")
                })
                .collect(),
        )
    }

    (size, data)
}

pub fn parse(mut lines: Vec<String>) -> Puzzle {
    //bad naming, possible confusion with str::parse
    check_empty_lines(&lines);
    lines = remove_comments(lines);
    check_empty_vec(&lines);
    check_only_numbers_and_spaces(&lines);
    let (size, data) = get_data(lines);
    check_values_are_incremental(size, data.clone());
    check_values_form_correct_square(size, data.clone());

    Puzzle::new_from_file(data, size)
}

#[cfg(test)]
mod parser_tests {
    mod parse {
        use crate::parser::*;

        #[test]
        #[should_panic]
        fn panic_no_size() {
            let lines: Vec<String> = vec![
                "0  3  4".to_string(),
                "1 5     6".to_string(),
                "   2 7 8   ".to_string(),
            ];
            parse(lines);
        }

        #[test]
        fn no_panic() {
            let lines: Vec<String> = vec![
                "3".to_string(),
                "1 2 3".to_string(),
                "8 0 4".to_string(),
                "7 6 5".to_string(),
            ];
            parse(lines);
        }
    }

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
            let tab = vec![0, 1, 2, 5];
            check_values_are_incremental(size, tab);
        }

        #[test]
        #[should_panic]
        fn panic_because_two_time_same_value() {
            let size: usize = 2;
            let tab = vec![0, 1, 2, 2];
            check_values_are_incremental(size, tab);
        }

        #[test]
        #[should_panic]
        fn panic_because_too_few_values() {
            let size: usize = 2;
            let tab = vec![0, 1, 2];
            check_values_are_incremental(size, tab);
        }

        #[test]
        fn no_panic() {
            let size: usize = 2;
            let tab = vec![0, 1, 2, 3];
            check_values_are_incremental(size, tab);
        }
    }

    mod check_values_form_correct_square {
        use crate::parser::*;

        #[test]
        #[should_panic]
        fn panic_because_too_few_values_in_one_line() {
            let size: usize = 3;
            let tab = vec![0, 1, 2, 3, 5, 6, 7, 8];
            check_values_form_correct_square(size, tab);
        }

        #[test]
        #[should_panic]
        fn panic_because_too_much_values_in_one_line() {
            let size: usize = 3;
            let tab = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            check_values_form_correct_square(size, tab);
        }

        #[test]
        #[should_panic]
        fn panic_because_wrong_number_of_lines() {
            let size: usize = 3;
            let tab = vec![0, 1, 2, 3, 4, 5];
            check_values_form_correct_square(size, tab);
        }

        #[test]
        #[should_panic]
        fn panic_because_square_is_too_small() {
            let size: usize = 2;
            let tab = vec![0, 1, 2, 3];
            check_values_form_correct_square(size, tab);
        }

        #[test]
        fn no_panic() {
            let size: usize = 3;
            let tab = vec![0, 1, 2, 2, 3, 5, 8, 7, 6];
            check_values_form_correct_square(size, tab);
        }
    }

    mod get_data {
        use crate::parser::*;

        #[test]
        fn use_case_test() {
            let data_string: Vec<String> = vec![
                "3".to_string(),
                "0  3  4".to_string(),
                "1 5     6".to_string(),
                "   2 7 8   ".to_string(),
            ];
            let data_number: (usize, Vec<usize>) = (3, vec![0, 3, 4, 1, 5, 6, 2, 7, 8]);

            assert_eq!(get_data(data_string), data_number);
        }

        #[test]
        #[should_panic]
        fn panic_if_cannot_get_size() {
            let data_string: Vec<String> = vec![];
            let data_number: (usize, Vec<usize>) = (3, vec![0, 3, 4, 1, 5, 6, 2, 7, 8]);

            assert_eq!(get_data(data_string), data_number);
        }
    }
}
