
pub fn remove_comments(mut lines: Vec<String>) -> Vec<String> {
	
	lines.retain( |ref l| (
			l.chars().count() != 0 
			&& l.chars().nth(0).unwrap() != '#'
		) 
		|| l.chars().count() == 0);

	lines
}