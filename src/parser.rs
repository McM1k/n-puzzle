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
            let immutable_all_the_values = all_the_values.clone();
            all_the_values.remove(
                immutable_all_the_values
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
                               .unwrap();
    lines.remove(0);

    size
}

pub fn get_data(lines: Vec<String>, size: u32) -> [[u32; size]; size] { // could fail because fo lifetime

}
*/

#[cfg(test)]
mod tests {
    use crate::parser;

    #[test]
    #[should_panic]
    fn remove_comments_panic_because_empty_line() {
        let lines: Vec<String> = vec!["".to_string()];
        parser::remove_comments(lines);
    }

    #[test]
    fn remove_comments_no_panic() {
        let lines: Vec<String> = vec!["1".to_string()];
        parser::remove_comments(lines);
    }

    #[test]
    #[should_panic]
    fn check_empty_vec_panic_because_empty_vec() {
        let lines: Vec<String> = vec![];
        parser::check_empty_vec(&lines);
    }

    #[test]
    fn check_empty_vec_no_panic() {
        let lines: Vec<String> = vec!["1".to_string()];
        parser::check_empty_vec(&lines);
    }

    #[test]
    #[should_panic]
    fn check_values_are_incremental_panic_because_value_out_of_scope() {
        let size: usize = 2;
        let tab = vec![vec![0, 1], vec![2, 5]];
        parser::check_values_are_incremental(&size, &tab);
    }

    #[test]
    fn check_values_are_incremental_no_panic() {
        let size: usize = 2;
        let tab = vec![vec![0, 1], vec![2, 3]];
        parser::check_values_are_incremental(&size, &tab);
    }
}
