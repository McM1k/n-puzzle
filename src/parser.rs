pub fn remove_comments(mut lines: Vec<String>) -> Vec<String> { //unsafe if used before check_empty_lines

	lines.retain( |ref line| line.chars().nth(0).unwrap() != '#');
	lines.iter_mut()
			.for_each( |ref mut line| {
		 		let pos = match line.chars().position( |c| c == '#') {
		 			Some(pos) => pos,
		 			None	  => line.len(),  
		 		};
		 		line.truncate(pos);
			});

	lines
}

pub fn check_empty_lines(lines: & Vec<String>) {
	lines.iter()
			.for_each(|line| if line.is_empty() { panic!("There is an empty line in the file")})
}

pub fn check_empty_vec(lines: & Vec<String>) {
	if lines.is_empty() {
		panic!("No data.")
	}
}

pub fn check_numbers_or_spaces(lines: & Vec<String>)  {
	lines.iter()
			.for_each(|line| {
		 		if !line.chars().all(|c| c.is_digit(10) || c.is_whitespace()) {
		 			panic!("Unexpected character in file (only spaces and numbers are allowed beside comments)")
		 		}
		 	})
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
*/
pub fn get_data(lines: Vec<String>) ->  Vec<Vec<usize>>{ // could fail because fo lifetime
	lines.iter()
			.map(|line| -> Vec<usize> {
				line.split_whitespace()
	 				.into_iter()
					.map(|token| -> usize {
 						token.parse::<usize>()
 					  			.expect("Unable to parse data into u32")
					})
 					.collect()
			})
			.collect()
}

#[cfg(test)]
mod tests {
	use crate::parser::*;

	#[test]
	fn get_data_use_case_test() {
		let data_string : Vec<String> = vec!["0  3  4".to_string(), 
											"1 5     6".to_string(),
											"   2 7 8   ".to_string()];
		let data_number : Vec<Vec<usize>> = vec![vec![0, 3, 4],
												vec![1, 5, 6],
												vec![2, 7, 8]];

		assert_eq!(get_data(data_string), data_number);
	}
}