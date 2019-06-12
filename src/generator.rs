// pub fn new_puzzle(size: usize) -> Vec<Vec<usize>> {

// }

pub fn is_solvable(puzzle: & Vec<Vec<usize>>) -> bool {
	let mut data : Vec<usize> = Vec::new();
	puzzle.iter().for_each(|line| line.iter().for_each(|value| data.push(*value)));
	let mut sort_count = 0;

	for _ in 0 ..= data.len() {
		for i in 0 .. data.len() {
			if data[i] > data[i + 1] {
				sort_count += 1;
				data.swap(i, i + 1);
			}
		}
	}

	if sort_count % 2 == 0 {
		return true;
	}
	else {
		return false;
	}
}

#[cfg(test)]
mod generator_tests {
	mod is_solvable {
		use crate::generator::is_solvable;

		#[test]
		fn solvable_puzzle_already_solved() {
			let puzzle: Vec<Vec<usize>> = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
			assert_eq!(is_solvable(puzzle), true);
		}

		#[test]
		fn solvable_puzzle() {
			let puzzle: Vec<Vec<usize>> = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
			assert_eq!(is_solvable(puzzle), true);
		}

		#[test]
		fn unsolvable_puzzle() {
			let puzzle: Vec<Vec<usize>> = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
			assert_eq!(is_solvable(puzzle), false);
		}
	}
}