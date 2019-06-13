use std::fmt;
use std::cmp;

mod puzzle;

enum Direction {
	Up,
	Left,
	Down,
	Right,
}

pub struct Node {
    state: Puzzle,
    distance: usize,
    upper_state: Option<Node>,
    lower_state: Option<Node>,
    left_state: Option<Node>,
    right_state: Option<Node>,
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
		Node{state, 0, None, None, None, None}
	}

	pub fn add_node(curr_node: &Node, dir: Direction) -> Option<Node> {

	}

	fn check_dir(puzzle: &Puzzle, dir: &Direction) -> Option<Puzzle> {
		let mut x, y = 0;

		for i in puzzle.data {
			for j in puzzle.data.nth(i)
				if puzzle.data.nth(i).nth(j) == 0 {
					x = j;
					y = i;
				}
			}
		}
		match dir {
			Direction::up => if x == 0 {
					return None;
				}
				else {
					
				},
			Direction::down => return { y != puzzle.size - 1 },
			Direction::left => return { x != 0 },
			Direction::right => return { x != puzzle.size - 1 },
		}
	}
}

#[cfg(test)]
mod graph_tests {
	mod check_dir {
		use crate::graph::*;

		#[test]
		fn wrong_direction() {
			let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
			let puzzle = Puzzle{data, 3};

			assert!(Node::check_dir(puzzle, Direction::Up) == None);
		}

		#[test]
		fn good_direction() {
			let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
			let puzzle = Puzzle{data, 3};

			let result_data = vec![vec![3, 1, 2], vec![0, 4, 5], vec![6, 8, 7]];
			let result_puzzle = Puzzle{result_data, 3};

			assert_eq!(Node::check_dir(puzzle, Direction::Down), Some(result_puzzle));
		}
	}

	mod PartialEq {
		use crate::graph::*;

		#[test]
		fn equals() {
			let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
			let puzzle = Puzzle{data, 3};
			let node = Node{puzzle, 0, None, None, None, None};

			let data2 = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
			let puzzle2 = Puzzle{data2, 3};
			let node2 = Node{puzzle2, 1, None, Some(node), None, None};

			assert_eq!(node, node2);
		}

		#[test]
		fn not_equals() {
			let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
			let puzzle = Puzzle{data, 3};
			let node = Node{puzzle, 0, None, None, None, None};

			let data2 = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
			let puzzle2 = Puzzle{data2, 3};
			let node2 = Node{puzzle2, 0, None, None, None, None};

			assert!(node != node2);
		}
	}

	mod add_node {
		use crate::graph::*;

		#[test]
		fn node_is_correctly_added() {
			let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
			let puzzle = Puzzle{data, 3};
			let node = Node::new_starting_node(puzzle);

			let result_data = vec![vec![1, 0, 2], vec![3, 4, 5], vec![6, 8, 7]];
			let result_puzzle = Puzzle{result_data, 3};

			assert_eq!(Node::add_node(node, Direction::Left), Some(result_puzzle));
		}

		fn trying_an_unpossible_move() {
			let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
			let puzzle = Puzzle{data, 3};
			let node = Node::new_starting_node(puzzle);

			assert_eq!(Node::add_node(node, Direction::Left), None);
		}
	}
}