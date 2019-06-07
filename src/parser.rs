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
pub fn get_data(lines: Vec<String>, size: u32) -> [[mut u32; size]; size] { // could fail because fo lifetime
		let data = [[mut u32; size] size];

		lines.iter()
				.for_each(|line| line.split_whitespace()
			 							.iter()
			 							.map(|token| token.parse::<u32>()
			 					  							.expect("Unable to parse data into u32")
			 					  							.unwrap())
}
