use std::fmt;
use std::cmp;
use crate::puzzle::Puzzle;

enum Direction {
	Up,
	Left,
	Down,
	Right,
}

pub struct Node {
    state: Puzzle,
    distance: usize,
    upper_state: Option<Box<Node>>,
    lower_state: Option<Box<Node>>,
    left_state: Option<Box<Node>>,
    right_state: Option<Box<Node>>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.state)?;
        write!(f, "{} nodes away from the start position\n", self.distance)?;
        Ok(())
    }
}

impl cmp::PartialEq for Node {
	fn eq(&self, other: &Self) -> bool {
		self.state == other.state
	}
}

impl Node {
	pub fn new_starting_node(state: Puzzle) -> Node {
		let start = 0;
		let empty = None;
		Node{state, start, empty, empty, empty, empty}
	}

//	pub fn add_node(curr_node: &Node, dir: Direction) -> Option<Node> {

//	}

	pub fn calculate_next_state(puzzle: &Puzzle, dir: &Direction) -> Option<Puzzle> {
		let (x, y) = get_void_position(puzzle);
		let mut new_puzzle;

		match dir {
			Direction::Up => if x == 0 {
					return None;
				}
				else {
					new_puzzle = swap_two_positions(puzzle, x, y, x - 1, y);
				},
			Direction::Down => if x == puzzle.size - 1 {
					return None;
				}
				else {
					new_puzzle = swap_two_positions(puzzle, x, y, x + 1, y);
				},
			Direction::Left =>  if y == 0 {
					return None;
				}
				else {
					new_puzzle = swap_two_positions(puzzle, x, y, x, y - 1);
				},
			Direction::Right =>  if y == puzzle.size - 1 {
					return None;
				}
				else {
					new_puzzle = swap_two_positions(puzzle, x, y, x, y + 1);
				},
		}

		Some(new_puzzle)
	}

	pub fn swap_two_positions(puzzle: &Puzzle, x: usize, y: usize, x_next: usize, y_next: usize) -> Puzzle {
		let mut new_data = puzzle.data;
		new_data.iter().nth(y).iter().nth(x) = puzzle.data.iter().nth(y_next).iter().nth(x_next);
		new_data.iter().nth(y_next).iter().nth(y_next) = puzzle.data.iter().nth(y).iter().nth(x);
		let size = new_data.len();

		Puzzle{new_data, size}
	}

	 pub fn get_void_position(puzzle: &Puzzle) -> (usize, usize) {
		let mut x = 0;
		let mut y = 0;

		for y in puzzle.data {
			for x in puzzle.data.iter().nth(y) {
				if puzzle.data.iter().nth(y).iter().nth(x) == 0 {
					return (x, y)
				}
			}
		}

		(x, y)
	}
}

#[cfg(test)]
mod graph_tests {
	mod swap_two_positions {
		use crate::graph::*;

		#[test]
		fn normal_swap() {
			let size = 3;
			let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
			let puzzle = Puzzle{data, size};
			let result_data = vec![vec![3, 1, 2], vec![0, 4, 5], vec![6, 8, 7]];
			let result_puzzle = Puzzle{result_data, size};

			assert_eq!(swap_two_positions(puzzle, 0, 0, 0, 1), result_puzzle);
		}
	}

	mod get_void_position {
		use crate::graph::*;

		#[test]
		fn center_pos() {
			let size = 3;
			let data = vec![vec![4, 1, 2], vec![3, 0, 5], vec![6, 8, 7]];
			let puzzle = Puzzle{data, size};

			let (x, y) = get_void_position(puzzle);
			assert!(x == 1 && y == 1);
		}

		#[test]
		fn lower_right_pos() {
			let size = 3;
			let data = vec![vec![4, 1, 2], vec![3, 7, 5], vec![6, 8, 0]];
			let puzzle = Puzzle{data, size};

			let (x, y) = get_void_position(puzzle);
			assert!(x == 2 && y == 2);
		}
	}

	mod calculate_next_state {
		use crate::graph::*;

		#[test]
		fn wrong_direction() {
			let size = 3;
			let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
			let puzzle = Puzzle{data, size};

			assert!(Node::calculate_next_state(puzzle, Direction::Up) == None);
		}

		#[test]
		fn good_direction() {
			let size = 3;
			let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
			let puzzle = Puzzle{data, size};

			let result_data = vec![vec![3, 1, 2], vec![0, 4, 5], vec![6, 8, 7]];
			let result_puzzle = Puzzle{result_data, size};

			assert_eq!(Node::calculate_next_state(puzzle, Direction::Down), Some(result_puzzle));
		}
	}

	mod PartialEq {
		use crate::graph::*;

		#[test]
		fn equals() {
			let len = 0;
			let size = 3;
			let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
			let puzzle = Puzzle{data, size};
			let empty: Option<Box<Node>> = None;
			let node = Node{puzzle, len, empty, empty, empty, empty};

			let len2 = 1;
			let data2 = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
			let puzzle2 = Puzzle{data2, size};
			let next = Some(box(node));
			let node2 = Node{puzzle2, len2, empty, next, empty, empty};

			assert_eq!(node, node2);
		}

		#[test]
		fn not_equals() {
			let len = 0;
			let size = 3;
			let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
			let puzzle = Puzzle{data, size};
			let empty: Option<Box<Node>> = None;
			let node = Node{puzzle, len, empty, empty, empty, empty};

			let data2 = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
			let puzzle2 = Puzzle{data2, size};
			let node2 = Node{puzzle2, len, empty, empty, empty, empty};

			assert!(node != node2);
		}
	}

	// mod add_node {
	// 	use crate::graph::*;

	// 	#[test]
	// 	fn node_is_correctly_added() {
	// 		let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
	// 		let puzzle = Puzzle{data, 3};
	// 		let node = Node::new_starting_node(puzzle);

	// 		let result_data = vec![vec![1, 0, 2], vec![3, 4, 5], vec![6, 8, 7]];
	// 		let result_puzzle = Puzzle{result_data, 3};

	// 		assert_eq!(Node::add_node(node, Direction::Left), Some(result_puzzle));
	// 	}

	// 	fn trying_an_unpossible_move() {
	// 		let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
	// 		let puzzle = Puzzle{data, 3};
	// 		let node = Node::new_starting_node(puzzle);

	// 		assert_eq!(Node::add_node(node, Direction::Left), None);
	// 	}
	// }
}